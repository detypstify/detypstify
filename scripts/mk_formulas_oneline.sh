#!/usr/bin/env bash
# This script replaces newline characters in all files in a directory with spaces

dir=$1  # Replace with the actual directory path

# Check if the directory exists
if [ ! -d "$dir" ]; then
    echo "This script replaces newline characters in all files in a directory with spaces."
    echo "Usage: $0 <directory>"
    exit 1
fi

# Iterate through each file in the directory
for file in "$dir"/*; do
    # Check if the file is a regular file
    if [ -f "$file" ]; then
        # Replace newline characters with spaces using sed
        tr '\n' ' ' < "$file" > tmp_file && mv tmp_file "$file" && sed -i 's/ $//' "$file"
        echo "Newline characters replaced with spaces in '$file'."
    fi
done
