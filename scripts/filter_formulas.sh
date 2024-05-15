#!/usr/bin/env bash

# This script filters out the formulas that have corresponding images in the images_png directory
# iterate through the formulas directory
i=0

im_dir=$2
form_dir=$1

# check arguments are provided
if [ -z "$im_dir" ] || [ -z "$form_dir" ]; then
  echo "This script filters out the formulas that have corresponding images in the images_png directory"
  echo "Usage: $0 <images_png_dir> <formulas_dir>"
  exit 1
fi

mkdir -p new_formulas
mkdir -p new_images

for f in $form_dir/*; do
  # find if the corresponding .png file exists in the images_png directory
  f=$(basename $f)
  if [ -f images_png/$f.png ]; then
    # if it exists cp it f to the new_formulas directore and the image to the new_images directory
    # extract the filename from the path
    cp $form_dir/$f new_formulas/$i
    cp $im_dir/$f.png new_images/$i.png
    echo "Copied $f to new_formulas/$i and images_png/$f.png to new_images/$i.png, i is $i"
    i=$((i+1))
  else
    echo "No image for $f, i is $i"
  fi
done
