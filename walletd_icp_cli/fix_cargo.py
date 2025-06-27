import re

with open('Cargo.toml', 'r') as f:
    content = f.read()

# Split into sections
sections = re.split(r'(\[[^\]]+\])', content)

# Process dependencies section
for i, section in enumerate(sections):
    if section == '[dependencies]':
        # Get the content after [dependencies]
        deps_content = sections[i + 1]
        
        # Remove duplicate entries
        lines = deps_content.strip().split('\n')
        seen = set()
        unique_lines = []
        
        for line in lines:
            if '=' in line:
                key = line.split('=')[0].strip()
                if key not in seen:
                    seen.add(key)
                    unique_lines.append(line)
            else:
                unique_lines.append(line)
        
        sections[i + 1] = '\n' + '\n'.join(unique_lines) + '\n'

# Reconstruct the file
with open('Cargo.toml', 'w') as f:
    f.write(''.join(sections))
