import os
import shutil
import random

def split_dataset(src_formulas_dir, src_images_dir, train_formulas_dir, train_images_dir, val_formulas_dir, val_images_dir):
    # Create destination directories if they don't exist
    for directory in [train_formulas_dir, train_images_dir, val_formulas_dir, val_images_dir]:
        if not os.path.exists(directory):
            os.makedirs(directory)

    # List all files in the source directories
    formulas_files = sorted([f for f in os.listdir(src_formulas_dir) if not f.endswith('.png')])
    images_files = [f + '.png' for f in formulas_files]

    # Randomly shuffle the files
    combined_files = list(zip(formulas_files, images_files))
    random.shuffle(combined_files)

    # Compute the split index for 20%
    split_idx = int(0.2 * len(combined_files))

    # Split files into validation and training sets
    validation_files = combined_files[:split_idx]
    training_files = combined_files[split_idx:]

    # Copy files to the appropriate validation and training folders
    for files, src_dir, dest_dir, file_type in [
        (validation_files, src_formulas_dir, val_formulas_dir, 'formula'),
        (validation_files, src_images_dir, val_images_dir, 'image'),
        (training_files, src_formulas_dir, train_formulas_dir, 'formula'),
        (training_files, src_images_dir, train_images_dir, 'image')
    ]:
        for f in files:
            src_path = os.path.join(src_dir, f[0] if file_type == 'formula' else f[1])
            dest_path = os.path.join(dest_dir, f[0] if file_type == 'formula' else f[1])
            shutil.copy(src_path, dest_path)

def rename_files(validation_dir):
    formulas_dir = os.path.join(validation_dir, 'new_formulas')
    images_dir = os.path.join(validation_dir, 'new_images_resized')

    formulas_files = sorted(os.listdir(formulas_dir))
    images_files = sorted(os.listdir(images_dir))

    for idx, (formula_file, image_file) in enumerate(zip(formulas_files, images_files), start=0):
        new_formula_name = f"{idx}"
        new_image_name = f"{idx}.png"

        os.rename(os.path.join(formulas_dir, formula_file), os.path.join(formulas_dir, new_formula_name))
        os.rename(os.path.join(images_dir, image_file), os.path.join(images_dir, new_image_name))

if __name__ == "__main__":
    # Define directories
    src_formulas_dir = 'new_formulas'
    src_images_dir = 'new_images_resized'
    train_formulas_dir = 'training/new_formulas'
    train_images_dir = 'training/new_images_resized'
    val_formulas_dir = 'validation/new_formulas'
    val_images_dir = 'validation/new_images_resized'

    # Perform the split
    split_dataset(src_formulas_dir, src_images_dir, train_formulas_dir, train_images_dir, val_formulas_dir, val_images_dir)

    # Rename validation files
    rename_files('validation')
    print("Dataset split and files renamed successfully.")

