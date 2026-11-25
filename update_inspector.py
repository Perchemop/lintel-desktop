#!/usr/bin/env python3
import re

# Update inspector.rs
with open('apps/lintel/src/inspector.rs', 'r') as f:
    content = f.read()

# Pattern for start anchor (lines 1345-1348)
pattern1 = r'DimensionAnchor::new\(\s+lintel_core::ids::WallId\(start_wall\),\s+start_param,\s+\)'
replacement1 = '''DimensionAnchor::anchored(
                            lintel_core::ids::WallId(start_wall),
                            start_param,
                            false,
                        )'''

content = re.sub(pattern1, replacement1, content, flags=re.MULTILINE)

# Pattern for end anchor (lines 1407-1410)
pattern2 = r'DimensionAnchor::new\(\s+lintel_core::ids::WallId\(end_wall\),\s+end_param,\s+\)'
replacement2 = '''DimensionAnchor::anchored(
                            lintel_core::ids::WallId(end_wall),
                            end_param,
                            false,
                        )'''

content = re.sub(pattern2, replacement2, content, flags=re.MULTILINE)

with open('apps/lintel/src/inspector.rs', 'w') as f:
    f.write(content)
print('Done inspector.rs')