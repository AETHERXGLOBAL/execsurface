use std::hint::black_box;
use std::time::Instant;

use execsurface_authority::EvidenceProposition;
use execsurface_hybrid_authority::{
    ExplicitHybridAdapter, HybridCapabilityContext, HybridHealthReport,
};

const BATCHES: usize = 21;
const ITERATIONS_PER_BATCH: usize = 10_000;

fn accepted_context() -> HybridCapabilityContext {
    HybridCapabilityContext {
        platform: "linux".to_owned(),
        architecture: "x86_64".to_owned(),
        kernel_release: Some("m11-8-controlled-capability-context".to_owned()),
        bpf_lsm_active: true,
        kernel_btf_readable: true,
        bpf_operation_permitted: true,
        privilege_explicitly_acknowledged: true,
        producer_loss_accounting_available: true,
        exec_success_confirmation_available: true,
        connect_completion_confirmation_available: true,
    }
}

fn percentile(sorted: &[u128], percentile: usize) -> u128 {
    let rank = (percentile * sorted.len()).div_ceil(100).saturating_sub(1);
    sorted[rank.min(sorted.len() - 1)]
}

fn main() {
    let context = accepted_context();

    // Warm-up is deliberately outside measured batches.
    for _ in 0..1_000 {
        exercise(&context);
    }

    let mut ns_per_iteration = Vec::with_capacity(BATCHES);
    for _ in 0..BATCHES {
        let start = Instant::now();
        for _ in 0..ITERATIONS_PER_BATCH {
            exercise(&context);
        }
        let elapsed = start.elapsed().as_nanos();
        ns_per_iteration.push(elapsed / ITERATIONS_PER_BATCH as u128);
    }

    ns_per_iteration.sort_unstable();
    let median = percentile(&ns_per_iteration, 50);
    let p95 = percentile(&ns_per_iteration, 95);
    let min = *ns_per_iteration.first().expect("non-empty benchmark");
    let max = *ns_per_iteration.last().expect("non-empty benchmark");

    println!(
        "{{\n  \"schema_version\": 1,\n  \"scope\": \"authority-adapter-only-not-end-to-end-hybrid-observation\",\n  \"batches\": {BATCHES},\n  \"iterations_per_batch\": {ITERATIONS_PER_BATCH},\n  \"median_ns_per_iteration\": {median},\n  \"p95_ns_per_iteration\": {p95},\n  \"min_ns_per_iteration\": {min},\n  \"max_ns_per_iteration\": {max},\n  \"product_default_authority\": false,\n  \"end_to_end_hybrid_latency_measured\": false\n}}"
    );
}

#[inline(never)]
fn exercise(context: &HybridCapabilityContext) {
    let adapter = ExplicitHybridAdapter::explicit_research_v1(black_box(context.clone()));
    let contract = adapter
        .contract()
        .expect("frozen accepted capability context must build a contract");

    black_box(contract.support_for(EvidenceProposition::FileOpenObjectIdentity));
    black_box(contract.support_for(EvidenceProposition::ProcessExecSuccess));
    black_box(contract.support_for(EvidenceProposition::ProcessExecObjectIdentity));
    black_box(contract.support_for(EvidenceProposition::NetworkConnectAttemptDestination));
    black_box(contract.support_for(EvidenceProposition::NetworkConnectSuccess));
    black_box(HybridHealthReport::default().evidence_health());
}
