use std::io::{BufRead, Write};

fn main() {
    // create new out directory containing file `OUT`
    let out_dir = "out";

    // create the directory
    std::fs::create_dir_all(out_dir).unwrap();

    // remove the file if it already exists
    let out_file_path = format!("{}/OUT", out_dir);
    if let Err(e) = std::fs::remove_file(&out_file_path) {
        if e.kind() != std::io::ErrorKind::NotFound {
            // Handle the error if it's not "file not found."
            panic!("Failed to remove file: {:?}", e);
        }
    }

    // create the file
    let mut out_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&out_file_path)
        .unwrap();

    let their_file = std::fs::File::open("im2latex-100k/im2latex_formulas.lst").unwrap();

    // iterate through their_file line by line
    for (idx, line) in std::io::BufReader::new(their_file).lines().enumerate() {
        if idx >= 2000 {
            break;
        }
        // get the line
        let line = format!("${}$", line.unwrap());

        // write `$` to out_file
        // write!(&out_file, "$").unwrap();
// pandoc -f latex -t typst -o OUTPUT_FILE.typ -

        // write `line` to stdin then run `bruh`
        match std::process::Command::new("pandoc").arg("-f").arg("latex").arg("-t").arg("typst").arg("-")
            .stdin(std::process::Stdio::piped())
            .stdout(out_file.try_clone().unwrap()) // Clone the file handle for each iteration
            .spawn() {
                Ok(mut pandoc_cmd) => {

                    // Pass the line as input to the bruh process
                    match pandoc_cmd.stdin.as_mut().unwrap().write_all(line.as_bytes()) {
                        Ok(ok) => {
                            // write `$` to out_file
                            // write!(&out_file, "$").unwrap();

                            // write newline to outfile
                            writeln!(&mut out_file).unwrap();

                            // Wait for the bruh process to finish
                            match pandoc_cmd.wait() {
                                Ok(_o) => {
                                    continue;

                                },
                                Err(e) => {
                                    println!("err {e}");
                                    continue;
                                },
                            }


                        },
                        Err(e) => {
                            println!("err {e}");
                            continue;
                        },
                    }
                },
                Err(e) => {
                    println!("err {e}");
                    continue;

                }

        }
    }
}
