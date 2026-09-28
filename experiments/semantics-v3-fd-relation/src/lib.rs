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
            classify_fd_table_relation(
                SpawnMechanism::Clone,
                Some(libc::CLONE_FILES as u64)
            ),
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
            classify_fd_table_relation(
                SpawnMechanism::Clone,
                Some(libc::CLONE_THREAD as u64)
            ),
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
}
