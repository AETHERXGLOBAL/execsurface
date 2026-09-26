from pathlib import Path

path = Path('experiments/m8-ebpf/persistent-observer/src/main.rs')
text = path.read_text()

old_gate = '''        && membership_map_empty
        && pending_mechanism_map_empty
        && session.exec_count >= 2
        && session.spawn_count >= 1
        && session.exit_count >= 2;
'''
new_gate = '''        && membership_map_empty
        && pending_mechanism_map_empty
        // M9-V1 external workloads are not required to fork. Preserve the
        // lifecycle/loss/integrity gates while replacing the old fixture-tree
        // shape assumption with the minimum observable root lifecycle.
        && session.exec_count >= 1
        && session.exit_count >= 1;
'''

if text.count(old_gate) != 1:
    raise SystemExit(f'expected exactly one M8.7 fixture-shape gate, found {text.count(old_gate)}')

text = text.replace(old_gate, new_gate)
path.write_text(text)
print('M9_V1_PERSISTENT_EXTERNAL_ADAPTER_APPLIED')
