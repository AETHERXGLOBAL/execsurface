//! Bounded, opt-in derived projection for GCC ephemeral assembly identities.
//!
//! This module never mutates raw canonical evidence and never treats frequency,
//! similarity, or filename shape alone as authorization.

use std::collections::{BTreeMap, BTreeSet};

use execsurface_model::canonical::{
    CanonicalEffect, CanonicalExecutable, CanonicalPath, CanonicalSurface, PathClass,
};
use execsurface_model::FileOperation;

pub const CANDIDATE_NORMALIZATION_PROFILE_VERSION: u32 = 4;
pub const GCC_TEMP_CANONICAL: &str = "$TMP/cc<gcc-ephemeral>.s";
const BOUNDED_GCC_PATH: &str = "/usr/bin/gcc";

#[derive(Debug, Clone, Default)]
struct CandidateState {
    create_open: bool,
    delete: bool,
    conflict: bool,
}

/// Return only identities that satisfy the frozen GCC producer/role/grammar
/// contract. This is a derived classification; it does not authorize behavior.
pub fn eligible_gcc_temp_paths(surface: &CanonicalSurface) -> BTreeSet<String> {
    let mut states: BTreeMap<String, CandidateState> = BTreeMap::new();

    for effect in &surface.effects {
        match effect {
            CanonicalEffect::FilePathAccess {
                actor,
                execution_chain,
                operation,
                target,
                open_intent,
            } if matches_gcc_temp_identity(target) => {
                let state = states.entry(target.value.clone()).or_default();
                if !is_bounded_gcc_actor(actor.as_ref(), execution_chain) {
                    state.conflict = true;
                    continue;
                }

                match operation {
                    FileOperation::Open
                        if open_intent
                            .as_ref()
                            .is_some_and(|intent| intent.create && intent.write) =>
                    {
                        state.create_open = true;
                    }
                    FileOperation::Delete => state.delete = true,
                    _ => state.conflict = true,
                }
            }
            CanonicalEffect::FileRename { from, to, .. } => {
                for path in [from, to] {
                    if matches_gcc_temp_identity(path) {
                        states.entry(path.value.clone()).or_default().conflict = true;
                    }
                }
            }
            _ => {}
        }
    }

    states
        .into_iter()
        .filter_map(|(path, state)| {
            (state.create_open && state.delete && !state.conflict).then_some(path)
        })
        .collect()
}

/// Build a deterministic derived view. The input surface is never mutated.
pub fn apply_gcc_ephemeral_projection(surface: &CanonicalSurface) -> CanonicalSurface {
    let eligible = eligible_gcc_temp_paths(surface);
    let mut candidate = surface.clone();
    candidate.normalization.profile_version = CANDIDATE_NORMALIZATION_PROFILE_VERSION;
    candidate.effects = candidate
        .effects
        .into_iter()
        .map(|effect| normalize_effect(effect, &eligible))
        .collect();
    candidate.effects.sort();
    candidate.effects.dedup();
    candidate
}

fn normalize_effect(effect: CanonicalEffect, eligible: &BTreeSet<String>) -> CanonicalEffect {
    match effect {
        CanonicalEffect::FilePathAccess {
            actor,
            execution_chain,
            operation,
            mut target,
            open_intent,
        } => {
            if eligible.contains(&target.value) {
                target.value = GCC_TEMP_CANONICAL.to_owned();
            }
            CanonicalEffect::FilePathAccess {
                actor,
                execution_chain,
                operation,
                target,
                open_intent,
            }
        }
        other => other,
    }
}

fn is_bounded_gcc_actor(
    actor: Option<&CanonicalExecutable>,
    execution_chain: &[CanonicalExecutable],
) -> bool {
    let Some(actor) = actor else {
        return false;
    };
    if actor.path.value != BOUNDED_GCC_PATH || actor.family != "gcc" {
        return false;
    }

    execution_chain.last().is_some_and(|last| last == actor)
}

fn matches_gcc_temp_identity(path: &CanonicalPath) -> bool {
    if path.class != PathClass::Temp {
        return false;
    }
    let Some(basename) = path.value.strip_prefix("$TMP/") else {
        return false;
    };
    if basename.contains('/') || basename.len() != 10 {
        return false;
    }

    let bytes = basename.as_bytes();
    bytes[0] == b'c'
        && bytes[1] == b'c'
        && bytes[2..8].iter().all(u8::is_ascii_alphanumeric)
        && bytes[8] == b'.'
        && bytes[9] == b's'
}
