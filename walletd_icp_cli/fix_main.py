with open('src/main.rs', 'r') as f:
    lines = f.readlines()

# Fix the testnet case - it seems there might be indentation issues
fixed_lines = []
in_c_case = False
skip_next_bracket = False

for i, line in enumerate(lines):
    # Skip the extra closing bracket after C case
    if i == 125 and line.strip() == '}':  # This is likely the extra bracket
        continue
    
    fixed_lines.append(line)

with open('src/main.rs', 'w') as f:
    f.writelines(fixed_lines)
    
print("Fixed main.rs")
