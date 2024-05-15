#!/usr/bin/env python3
from PIL import Image
import os
import sys

def pad_image(input_path, output_path):
    # Open the original image
    original = Image.open(input_path)

    # Desired dimensions are the next multiple of 16 from the original dimensions
    desired_width = (original.width + 15) // 16 * 16
    desired_height = (original.height + 15) // 16 * 16

    print(f"Original size: {original.width}x{original.height}")
    print(f"Resizing {input_path} to {desired_width}x{desired_height}")

    # Calculate padding sizes
    original_width, original_height = original.size
    left = (desired_width - original_width) // 2
    top = (desired_height - original_height) // 2
    right = desired_width - original_width - left
    bottom = desired_height - original_height - top

    # Create a new image with white background
    new_image = Image.new("RGBA", (desired_width, desired_height), "WHITE")

    # Paste the original image into the center of the new image
    new_image.paste(original, (left, top))

    # Save the padded image
    new_image.save(output_path)

def main(in_directory, out_directory):
# Usage
    idx = 0
# Resize all PNGs to match the dimensions of the largest PNGj
    for filename in os.listdir(in_directory):
        print(filename)
        idx += 1
        if filename.endswith('.png'):
            image_path = os.path.join(in_directory, filename)
            out_image_path = os.path.join(out_directory, filename)
            with Image.open(image_path) as img:
                print(f"resizing image {image_path}. {idx}")
                pad_image(image_path, out_image_path)

if __name__ == "__main__":
    # Check if the number of arguments is correct
    if len(sys.argv) != 3:
        print("Usage: python resize.py <in_directory> <out_directory>")
        sys.exit(1)

    # Check if the input directory exists
    if not os.path.exists(sys.argv[1]):
        print(f"Directory {sys.argv[1]} does not exist.")
        sys.exit(1)

    # Check if the output directory exists if not create it
    if not os.path.exists(sys.argv[2]):
        os.makedirs(sys.argv[2])

    in_directory = sys.argv[1]
    out_directory = sys.argv[2]

    main(in_directory, out_directory)
