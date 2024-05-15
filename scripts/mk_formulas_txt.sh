#!/usr/bin/env bash
# Create a formulas text file with one formula per line

dir=$1  # Replace with the actual directory path

# Check if the directory exists
if [ ! -d "$dir" ]; then
    echo "This script creates a formulas.txt file with one formula per line in the given directory."
    echo "Usage: $0 <directory>"
    exit 1
fi

# Create a new file to store the combined content
touch formulas.txt

# Iterate through each file in the directory and append them to a single file with a new line after each file content
for file in "$dir"/*; do
    cat "$file" >> formulas.txt
    echo "" >> formulas.txt
done
