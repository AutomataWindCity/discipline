#!/bin/bash

# Check if a glob pattern was provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 'glob_pattern'"
    echo "Example: $0 '*.txt'"
    echo "Example: $0 'src/*.js'"
    echo "Example: $0 '**/*.log' (requires globstar option)"
    exit 1
fi

# Enable globstar for recursive matching (if using **)
shopt -s globstar nullglob

# Store the pattern
pattern="$1"

# Find all files matching the pattern
files=($pattern)

# Check if any files were found
if [ ${#files[@]} -eq 0 ]; then
    echo "No files found matching pattern: $pattern"
    exit 0
fi

# Process each file
for file in "${files[@]}"; do
    # Skip if it's a directory (only process regular files)
    if [ -f "$file" ]; then
        echo "File $file: \`\`\`"
        cat "$file"
        echo "\`\`\`"
        echo  # Add a blank line between files for readability
    fi
done