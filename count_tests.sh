#!/bin/bash
# Count tests in original and ported projects

cd /Users/xuming/projects/ink-c-sharp-to-rust

echo "=== C# Tests ==="
echo

python3 << 'PYEOF'
import subprocess, re, os

acronyms = {'api', 'id', 'url', 'css', 'html', 'xml', 'json', 'sql', 'http', 'https', 'tcp', 'udp', 'rest', 'rpc', 'gui', 'cli', 'etc'}

def rust_to_pascal(name):
    """Convert Rust snake_case test name to PascalCase."""
    s = re.sub(r'^test_', '', name.strip())
    s = s.replace('_', ' ')
    parts = s.split()
    return ''.join(p.capitalize() if p.lower() not in acronyms else p.upper() for p in parts)

def get_csharp_tests():
    path = '/Users/xuming/projects/ink-c-sharp-to-rust/ink-c-sharp/tests/Tests.cs'
    content = open(path).read()
    tests = []
    for m in re.finditer(r'public void (Test\w+)\s*\(\)', content):
        name = m.group(1)[4:]  # remove 'Test' prefix
        tests.append(name.strip())
    return tests

def get_rust_tests():
    """Extract test names from c_sharp.rs by finding #[test] followed by fn test_."""
    path = '/Users/xuming/projects/ink-c-sharp-to-rust/ink-rust/crates/ink-integration-tests/src/c_sharp.rs'
    content = open(path).read()
    tests = []
    lines = content.splitlines()
    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped == '#[test]':
            for j in range(i+1, len(lines)):
                next_line = lines[j].strip()
                if not next_line:
                    continue
                m = re.match(r'fn (test_\w+)\s*\(', next_line)
                if m:
                    tests.append(m.group(1))
                break
    return tests

csharp_raw = get_csharp_tests()
rust_raw = get_rust_tests()

# C# names as PascalCase (already in PascalCase)
# Rust names: convert to PascalCase
csharp_set = set(csharp_raw)  # PascalCase
rust_pascal_map = {rust_to_pascal(t): t for t in rust_raw}
rust_set = set(rust_pascal_map.keys())

# Detect name collisions in Rust (multiple Rust names -> same PascalCase)
from collections import Counter
pascal_counts = Counter(rust_pascal_map.keys())
collisions = {k: v for k, v in pascal_counts.items() if v > 1}
if collisions:
    print("WARNING: Rust name collisions:")
    for name, count in collisions.items():
        rust_names = [rn for rn, p in rust_pascal_map.items() if p == name]
        print(f"  {name}: {rust_names}")

# Not ported: C# tests whose PascalCase name is NOT in Rust's PascalCase set
# But exclude names that collide (those are handled separately)
collision_pascal = set(collisions.keys())
not_ported = [t for t in csharp_raw if t not in rust_set and t not in collision_pascal]

# Extra in Rust: PascalCase names that don't match any C# name
# These include collision cases and genuinely extra tests
extra_pascal = [p for p in rust_set if p not in csharp_set]

print(f"\nOriginal (C#):  {len(csharp_raw)} tests")
print(f"Ported (Rust): {len(rust_raw)} tests")
print(f"Missing from Rust: {len(not_ported)} tests")
print(f"Extra in Rust: {len(extra_pascal)} (includes name collisions)")

print("\nNot ported C# tests:")
for t in not_ported:
    print(f"  {t}")

print(f"\nExtra in Rust ({len(extra_pascal)}):")
for p in sorted(extra_pascal):
    rust_name = rust_pascal_map[p]
    print(f"  {p}  ({rust_name})")
PYEOF
echo "=== Blade Tests ==="
echo

# Blade tests - get all test names with file path (preserve order)
rm -f /tmp/blade_all.txt
for f in blade-ink-rs/conformance-tests/tests/*.rs; do
  fname=$(basename "$f")
  grep -A1 "#\[test\]" "$f" | grep "^fn " | sed "s/fn //; s/().*//; s/^/$fname: /"
done > /tmp/blade_all.txt

# Blade Rust ported test names (from all files)
find ink-rust/crates/ink-integration-tests/src/blade -name "*.rs" -exec grep -A1 "#\[test\]" {} \; | grep "^[[:space:]]*fn " | sed 's/.*fn //; s/().*//' | sort > /tmp/blade_rust.txt

blade_all=$(wc -l < /tmp/blade_all.txt)
blade_rust=$(wc -l < /tmp/blade_rust.txt)
echo "Original: $blade_all tests"
echo "Ported:   $blade_rust tests"
echo "Difference: $((blade_rust - blade_all)) tests"
echo
echo "Not ported Blade tests (by file, in original order):"
while IFS=: read -r fname testname; do
  # Trim whitespace from testname
  testname=$(echo "$testname" | xargs)
  if ! grep -qx "$testname" /tmp/blade_rust.txt; then
    echo "  $fname: $testname"
  fi
done < /tmp/blade_all.txt
