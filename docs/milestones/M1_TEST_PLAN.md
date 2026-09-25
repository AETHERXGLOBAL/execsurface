# M1 Test Plan

## M1 acceptance evidence

The production observer is metadata-only native ptrace on Linux x86_64.

Required CI tests:

1. descendant exec:
   - fixture launches `/bin/true`;
   - observer records confirmed executable path.

2. argv privacy:
   - fixture receives a high-entropy sentinel only through argv;
   - serialize the entire Observation;
   - sentinel MUST NOT occur.

3. file path:
   - fixture reads a controlled temporary file;
   - observer records the selected path-based open syscall.

4. network:
   - test creates a loopback listener;
   - fixture connects to its ephemeral port;
   - observer records `127.0.0.1:<port>`;
   - no external network dependency.

5. backend evidence:
   - observation records backend/platform/architecture/capabilities/limitations.

## Explicit non-claims

M1 does not yet prove:

- fd-level read/write attribution,
- canonicalization stability,
- complete syscall coverage,
- low overhead,
- support beyond Linux x86_64,
- hostname attribution,
- safety or malware status.

## Gate

M1 cannot close from source review alone. The branch must produce a successful independent GitHub Actions run, and failures must be preserved/fixed rather than hidden.
