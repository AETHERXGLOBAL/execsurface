//! Research-only classifier for Semantics v3 shared-FD exactness work.
//!
//! This experiment does not change ExecSurface alpha.4 observation semantics.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnMechanism {
    Fork,
    Vfork,
    Clone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FdTableRelation {
    Shared,
    IndependentCopy,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetainedSpawnEvidence {
    pub mechanism: SpawnMechanism,
    pub clone_files: Option<bool>,
    pub clone_thread: Option<bool>,
    pub fd_table_relation: FdTableRelation,
}

pub fn classify_fd_table_relation(
    mechanism: SpawnMechanism,
    clone_flags: Option<u64>,
) -> FdTableRelation {
    match mechanism {
        SpawnMechanism::Fork | SpawnMechanism::Vfork => FdTableRelation::IndependentCopy,
        SpawnMechanism::Clone => match clone_flags {
            Some(flags) if flags & libc::CLONE_FILES as u64 != 0 => FdTableRelation::Shared,
            Some(_) => FdTableRelation::IndependentCopy,
            None => FdTableRelation::Unknown,
        },
    }
}

/// Retain only the semantic bits currently justified for v3 fd/thread
/// relationship analysis. This deliberately avoids treating unrestricted raw
/// clone flags as canonical baseline data.
pub fn retain_spawn_evidence(
    mechanism: SpawnMechanism,
    clone_flags: Option<u64>,
) -> RetainedSpawnEvidence {
    let (clone_files, clone_thread) = match mechanism {
        SpawnMechanism::Fork | SpawnMechanism::Vfork => (None, None),
        SpawnMechanism::Clone => match clone_flags {
            Some(flags) => (
                Some(flags & libc::CLONE_FILES as u64 != 0),
                Some(flags & libc::CLONE_THREAD as u64 != 0),
            ),
            None => (None, None),
        },
    };

    RetainedSpawnEvidence {
        mechanism,
        clone_files,
        clone_thread,
        fd_table_relation: classify_fd_table_relation(mechanism, clone_flags),
    }
}

/// Mirrors the current ptrace state-machine behavior without changing it:
/// only a proved shared relation reuses the parent table. Unknown remains
/// conservatively modeled as a copied table while higher-level completeness
/// must remain non-admissible.
pub fn reuses_parent_fd_table(relation: FdTableRelation) -> bool {
    matches!(relation, FdTableRelation::Shared)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fork_is_independent_copy() {
        assert_eq!(
            classify_fd_table_relation(SpawnMechanism::Fork, None),
            FdTableRelation::IndependentCopy
        );
    }

    #[test]
    fn vfork_is_independent_copy() {
        assert_eq!(
            classify_fd_table_relation(SpawnMechanism::Vfork, None),
            FdTableRelation::IndependentCopy
        );
    }

    #[test]
    fn clone_files_is_shared() {
        assert_eq!(
            classify_fd_table_relation(SpawnMechanism::Clone, Some(libc::CLONE_FILES as u64)),
            FdTableRelation::Shared
        );
    }

    #[test]
    fn known_clone_without_clone_files_is_independent_copy() {
        assert_eq!(
            classify_fd_table_relation(SpawnMechanism::Clone, Some(0)),
            FdTableRelation::IndependentCopy
        );
    }

    #[test]
    fn clone_thread_without_clone_files_does_not_launder_shared_authority() {
        assert_eq!(
            classify_fd_table_relation(SpawnMechanism::Clone, Some(libc::CLONE_THREAD as u64)),
            FdTableRelation::IndependentCopy
        );
    }

    #[test]
    fn unavailable_clone_flags_are_unknown() {
        assert_eq!(
            classify_fd_table_relation(SpawnMechanism::Clone, None),
            FdTableRelation::Unknown
        );
        assert!(!reuses_parent_fd_table(FdTableRelation::Unknown));
    }

    #[test]
    fn only_shared_relation_reuses_parent_table() {
        assert!(reuses_parent_fd_table(FdTableRelation::Shared));
        assert!(!reuses_parent_fd_table(FdTableRelation::IndependentCopy));
        assert!(!reuses_parent_fd_table(FdTableRelation::Unknown));
    }

    #[test]
    fn retained_shared_clone_exposes_only_required_semantic_bits() {
        let flags = libc::CLONE_FILES as u64 | libc::CLONE_THREAD as u64 | libc::CLONE_VM as u64;
        let evidence = retain_spawn_evidence(SpawnMechanism::Clone, Some(flags));

        assert_eq!(evidence.mechanism, SpawnMechanism::Clone);
        assert_eq!(evidence.clone_files, Some(true));
        assert_eq!(evidence.clone_thread, Some(true));
        assert_eq!(evidence.fd_table_relation, FdTableRelation::Shared);
    }

    #[test]
    fn retained_known_private_clone_is_explicitly_independent() {
        let evidence = retain_spawn_evidence(SpawnMechanism::Clone, Some(libc::CLONE_VM as u64));

        assert_eq!(evidence.clone_files, Some(false));
        assert_eq!(evidence.clone_thread, Some(false));
        assert_eq!(evidence.fd_table_relation, FdTableRelation::IndependentCopy);
    }

    #[test]
    fn retained_unknown_clone_does_not_invent_semantic_bits() {
        let evidence = retain_spawn_evidence(SpawnMechanism::Clone, None);

        assert_eq!(evidence.clone_files, None);
        assert_eq!(evidence.clone_thread, None);
        assert_eq!(evidence.fd_table_relation, FdTableRelation::Unknown);
    }
}
