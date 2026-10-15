'use strict';
/*
 * Learn and check a real, deterministic Node.js command, retaining an
 * unchanged PASS and a deliberate-write REVIEW against stable ExecSurface v1.
 *
 * Mirrors examples/python-pytest/run.py: the observed target command is
 * executed BY ExecSurface (learn/check), not by this runner, and the same
 * command + fixture stay selected across every step. Only the public fixture's
 * boolean flips between the unchanged check and the drift check.
 *
 * Node.js 18+, zero dependencies, node: built-ins only. No network during the
 * measured run.
 */
const { spawnSync } = require('node:child_process');
const crypto = require('node:crypto');
const fs = require('node:fs');
const path = require('node:path');
const process = require('node:process');

class RecipeError extends Error {}

function parseArgs(argv) {
  const args = { execsurface: null, node: process.execPath, output: null };
  for (let i = 0; i < argv.length; i += 1) {
    const a = argv[i];
    if (a === '--execsurface') args.execsurface = argv[++i];
    else if (a === '--node') args.node = argv[++i];
    else if (a === '--output') args.output = argv[++i];
    else throw new RecipeError(`unknown argument: ${a}`);
  }
  if (!args.execsurface || !args.output) {
    throw new RecipeError('both --execsurface and --output are required');
  }
  return args;
}

function invoke(command, cwd, name, expected) {
  // Retain command, both streams and actual exit before checking expectation.
  const result = spawnSync(command[0], command.slice(1), {
    cwd,
    encoding: 'buffer',
    timeout: 60000,
  });
  const outPath = path.join(cwd, `${name}.stdout`);
  const errPath = path.join(cwd, `${name}.stderr`);
  const receiptPath = path.join(cwd, `${name}-command.json`);
  fs.writeFileSync(outPath, result.stdout || Buffer.alloc(0));
  fs.writeFileSync(errPath, result.stderr || Buffer.alloc(0));
  if (result.error) {
    fs.writeFileSync(
      receiptPath,
      JSON.stringify(
        { command, returncode: null, expectedReturncode: expected, outcome: 'spawn-error', error: String(result.error) },
        null,
        2,
      ) + '\n',
    );
    throw new RecipeError(`${name}: could not spawn ${command[0]}: ${result.error}`);
  }
  fs.writeFileSync(
    receiptPath,
    JSON.stringify({ command, returncode: result.status, expectedReturncode: expected }, null, 2) + '\n',
  );
  if (result.status !== expected) {
    throw new RecipeError(
      `${name}: expected exit ${expected}, got ${result.status}; retained receipts: ${cwd}`,
    );
  }
}

function check(binary, node, root, name, expected, verdict) {
  const command = [
    binary,
    'check',
    '--baseline',
    path.join(root, 'execsurface.lock.json'),
    '--workspace',
    root,
    '--json-output',
    path.join(root, `${name}-verdict.json`),
    '--',
    node,
    'test.js',
  ];
  invoke(command, root, name, expected);
  const report = JSON.parse(fs.readFileSync(path.join(root, `${name}-verdict.json`), 'utf8'));
  if (
    !report.target ||
    report.target.exit_code !== 0 ||
    report.target.signal !== null
  ) {
    throw new RecipeError(`${name}: node test.js did not exit successfully; retained ${root}`);
  }
  if (report.policy && report.policy.source !== 'builtin:review-unmatched-drift') {
    throw new RecipeError(`${name}: selected built-in policy differs; retained ${root}`);
  }
  if (verdict === 'REVIEW') {
    // A deliberate new file write under the declared workspace is the single
    // expected drift finding. fs.writeFileSync on a new file may be recorded
    // as file_create or file_write; either is the documented effect here.
    const hasDriftWrite = (report.findings || []).some(
      (f) =>
        f.change === 'added' &&
        (f.effect_kind === 'file_write' || f.effect_kind === 'file_create') &&
        f.evidence &&
        f.evidence.effect &&
        f.evidence.effect.target &&
        f.evidence.effect.target.value === '$WORKSPACE/drift.txt',
    );
    if (!hasDriftWrite) {
      throw new RecipeError(`${name}: deliberate $WORKSPACE/drift.txt write finding absent; retained ${root}`);
    }
  }
  if (report.verdict !== verdict.toLowerCase()) {
    throw new RecipeError(`${name}: expected ${verdict} verdict, got ${report.verdict}; retained ${root}`);
  }
}

function sha256(file) {
  return crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');
}

function run(args) {
  const binary = path.resolve(args.execsurface);
  const node = path.resolve(args.node);
  if (!fs.existsSync(binary)) throw new RecipeError(`execsurface binary not found: ${binary}`);
  if (!fs.existsSync(node)) throw new RecipeError(`node executable not found: ${node}`);

  const out = path.resolve(args.output);
  if (fs.existsSync(out)) {
    throw new RecipeError(
      `output directory already exists; use a new path each run: ${out}`,
    );
  }
  fs.mkdirSync(out, { recursive: true });

  // Copy the public, dependency-free fixture into the retained run directory.
  const fixtureDir = path.join(__dirname, 'fixture');
  for (const name of ['test.js', 'fixture.json', 'package.json']) {
    fs.copyFileSync(path.join(fixtureDir, name), path.join(out, name));
  }

  const selection = {
    binarySha256: sha256(binary),
    nodeExecutable: node,
    fixtureSha256: sha256(path.join(fixtureDir, 'test.js')),
    policy: 'unchanged built-in REVIEW for unmatched drift',
    privacy:
      'public synthetic fixture; no environment or secret/file-content collection added',
  };
  fs.writeFileSync(path.join(out, 'selection.json'), JSON.stringify(selection, null, 2) + '\n');

  try {
    // Identity: this recipe targets the stable v1 binary, not a historical pin.
    invoke([binary, '--version'], out, 'version', 0);
    const versionLine = fs.readFileSync(path.join(out, 'version.stdout'), 'utf8').trim();
    if (versionLine !== 'execsurface 1.0.0') {
      throw new RecipeError(`recipe requires stable ExecSurface 1.0.0, got: ${versionLine}`);
    }
    // Native readiness within the declared Linux x86_64 / ptrace boundary.
    invoke([binary, 'doctor'], out, 'doctor', 0);

    const learn = [
      binary,
      'learn',
      '--output',
      path.join(out, 'execsurface.lock.json'),
      '--label',
      'node-npm-fixture',
      '--workspace',
      out,
      '--',
      node,
      'test.js',
    ];
    invoke(learn, out, 'learn', 0);
    const before = fs.readFileSync(path.join(out, 'execsurface.lock.json'));

    check(binary, node, out, 'unchanged', 0, 'PASS');

    // Only this public fixture value changes; command and fixture stay the same.
    fs.writeFileSync(path.join(out, 'fixture.json'), '{"left":2,"right":3,"write_extra":true}\n');
    check(binary, node, out, 'drift', 10, 'REVIEW');

    if (!bufferEqual(fs.readFileSync(path.join(out, 'execsurface.lock.json')), before)) {
      throw new RecipeError('baseline changed while checking drift');
    }
    if (fs.readFileSync(path.join(out, 'drift.txt'), 'utf8') !== 'controlled fixture drift\n') {
      throw new RecipeError('deliberate native write is missing');
    }

    const report = {
      status: 'verified-recipe',
      unchanged: { verdict: 'PASS', exit: 0 },
      drift: { verdict: 'REVIEW', exit: 10 },
      baselineSha256: crypto.createHash('sha256').update(before).digest('hex'),
      scope:
        'stable-v1 Node.js/npm fixture; runtime drift example, not independent security validation',
      receipts: out,
    };
    fs.writeFileSync(path.join(out, 'recipe-report.json'), JSON.stringify(report, null, 2) + '\n');
    return report;
  } catch (error) {
    fs.writeFileSync(
      path.join(out, 'failure.json'),
      JSON.stringify({ status: 'failed', reason: error.message }, null, 2) + '\n',
    );
    throw error;
  }
}

function bufferEqual(a, b) {
  if (!Buffer.isBuffer(a) || !Buffer.isBuffer(b) || a.length !== b.length) return false;
  for (let i = 0; i < a.length; i += 1) if (a[i] !== b[i]) return false;
  return true;
}

if (require.main === module) {
  try {
    const args = parseArgs(process.argv.slice(2));
    console.log(JSON.stringify(run(args), null, 2));
  } catch (error) {
    if (error instanceof RecipeError) {
      console.error(`ExecSurface node-npm recipe: ERROR: ${error.message}`);
      process.exitCode = 1;
    } else {
      throw error;
    }
  }
}
