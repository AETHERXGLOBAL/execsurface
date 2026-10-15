'use strict';
/* One deterministic node test; the optional extra write is deliberate runtime drift. */
const fs = require('fs');
const path = require('path');

// The public fixture is a two-value selection plus an optional drift switch.
const selection = JSON.parse(fs.readFileSync('fixture.json', 'utf8'));

// Deterministic invariant — holds on the unchanged fixture.
if (selection.left + selection.right !== 5) {
  console.error('fixture invariant broken: left + right !== 5');
  process.exit(1);
}

// Deliberate controlled runtime effect: write a file only when the fixture
// enables it. This is the single NEW execution-surface effect that ExecSurface
// should surface as REVIEW when the fixture is flipped.
if (selection.write_extra) {
  fs.writeFileSync(path.join(process.cwd(), 'drift.txt'), 'controlled fixture drift\n');
}

console.log('node fixture passed');
