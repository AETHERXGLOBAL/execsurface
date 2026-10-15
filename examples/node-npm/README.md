# Node.js/npm: unchanged PASS and deliberate-write REVIEW

Current example, aligned with stable ExecSurface **v1.0.0** on Linux x86_64 with
native `ptrace`; `doctor` must succeed on the host. This recipe learns and
checks the **same real Node.js command** against a tiny dependency-free
fixture, mirroring the [Python/pytest recipe](../python-pytest/README.md). It is
a reproducible integration example, not independent external validation or a
safety certification.

## What it demonstrates

The observed target is a one-file Node script that is deterministic: it reads a
public `fixture.json`, asserts a sum invariant, and — only when the fixture's
`write_extra` flag is set — writes a harmless `drift.txt` into its working
directory. That single new file write is the controlled runtime effect that
ExecSurface should surface as `REVIEW`.

From the repository root, first obtain the stable v1 binary using the
installation notes of the
[v1.0.0 release](https://github.com/AETHERXGLOBAL/execsurface/releases/tag/v1.0.0).
Keep its checksum-verified path in `EXEC_SURFACE_BIN`; this recipe records its
digest and checks its version. Then run the runner with Node.js 18 or later.
The runner uses only `node:` built-ins and downloads nothing.

```sh
EXEC_SURFACE_BIN=/absolute/path/to/checksum-verified/execsurface
node examples/node-npm/run.js \
  --execsurface "$EXEC_SURFACE_BIN" \
  --output /tmp/execsurface-node-npm-first-run
```

Use a new output directory each time. The runner refuses to overwrite a
retained run, copies the three public fixture files, and retains the native
command receipts, stdout/stderr, baseline and verdict JSON. A failed
expectation writes `failure.json` and exits nonzero, retaining the failure.
`--node` may explicitly select another Node executable; otherwise the runner
uses its own interpreter.

The observed command is:

```sh
node test.js
```

The runner performs the ordinary `learn --output ... --label ... --workspace ...`
and `check --baseline ... --workspace ... --json-output ...` flows:

| Step | Node result | ExecSurface result | Asserted exit |
| --- | --- | --- | --- |
| Learn `write_extra: false` | `node test.js` exits 0 | Baseline written | 0 |
| Check unchanged fixture | `node test.js` exits 0 | PASS; no drift | 0 |
| Set public fixture `write_extra: true`, check again | `node test.js` exits 0 | REVIEW; added `$WORKSPACE/drift.txt` write | 10 |

The same assertion, command, Node interpreter and baseline remain selected.
Only the public fixture's boolean changes. The test then writes the harmless
`drift.txt` inside its fresh workspace. The runner verifies that a matching
`file_create`/`file_write` finding appears and that the baseline bytes did not
change.

The **baseline** records the selected observed execution surface. The **policy**
interprets differences; this recipe uses the unchanged built-in REVIEW policy
for unmatched drift. It does not relearn after drift or weaken policy to get
PASS. ERROR, BLOCK, an unexpected native exit, a lost report or an unavailable
observer fails the recipe rather than counting as either expected outcome.

## Support boundary

Stable v1.0.0 supports **Linux x86_64** with native `ptrace` (see
[`docs/STATUS.md`](../../docs/STATUS.md)). Windows and ARM64 are not claimed.
The observed `node test.js` command is a familiar Node.js/npm developer
workload; ExecSurface observes the resulting Linux process/file/network
effects, not Node.js-specific semantics.

## What this example is and is not

The fixture contains no secrets. It adds no environment enumeration or
secret/file-content collection. ExecSurface's existing metadata observer may
record local executable and filesystem paths; review retained receipts before
sharing them. A PASS here establishes this selected observed fixture
comparison; it does not establish containment, malware safety, complete
observation of every possible effect or independent custody.
