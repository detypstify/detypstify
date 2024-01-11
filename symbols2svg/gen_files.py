#!/usr/bin/env python3

import subprocess
import re
import os

TMP_NAME = "tmp"

with open("symbols.txt", 'r') as file:
    if os.path.exists(TMP_NAME):
        os.remove(TMP_NAME)

    for line in file:
        symbol_name = line.strip()
        with open(TMP_NAME, 'w') as input_file:
            input_file.write(f"${symbol_name}$")

        # TODO in a directory
        file_name = f"{symbol_name}.svg"
        command = f"typst compile --format svg {TMP_NAME} {file_name}"
        with open(file_name, 'w') as output_file:
            process = subprocess.Popen(command, shell=True, stdout=output_file, stderr=subprocess.PIPE)
            _, stderr = process.communicate()

        if stderr:
            print(f"ERROR WHILE EXECUTING {command}: {stderr.decode()}")
        os.remove(TMP_NAME)
