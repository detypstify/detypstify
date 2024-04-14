use std::io::{BufRead, Write};

use regex::{Captures, Regex};

// performs processing
// (1) s/\cal/\mathcal
fn process_line(s: String) -> Option<String> {
    Some(s)
        .and_then(process_mathcal)
        .and_then(process_sp)
        .and_then(process_table)
        .and_then(process_vspace)
        .and_then(process_hspace)
        .and_then(process_bf)
}

fn process_mathcal(s: String) -> Option<String> {
    Some(s.replace(r"\cal", r"\mathcal"))
}

fn process_sp(s: String) -> Option<String> {
    Some(s.replace(r"\sp ", r"^ "))
}

fn process_bf(s: String) -> Option<String> {
    Some(s.replace(r"\bf", r"\mathbf"))
}

fn process_table(s: String) -> Option<String> {
    let regex = Regex::new(r"\\begin\{array\}\s+\{[\scl]*\}").unwrap();
    let result = regex.replace_all(&s, |caps: &Captures| caps[0].replace(" ", ""));
    Some(result.to_string())
}

// pandoc doesn't support vspace. Trash result
fn process_vspace(s: String) -> Option<String> {
    if s.contains(r"\vspace") {
        None
    } else {
        Some(s)
    }
    // let regex = Regex::new(r"(\\vspace\s*)\{([0-9a-z\s]*)\}").unwrap();
    // let result = regex.replace_all(&s, |caps: &Captures| {
    //     format!("{}{{ {:} }}", &caps[1], caps[2].replace(" ", ""))
    // });
    // result.to_string()
}

fn process_hspace(s: String) -> Option<String> {
    let regex = Regex::new(r"(\\hspace\s*)\{([0-9a-z\s]*)\}").unwrap();
    let result = regex.replace_all(&s, |caps: &Captures| {
        format!("{}{{ {:} }}", &caps[1], caps[2].replace(" ", ""))
    });
    Some(result.to_string())
}

// returns: (expr, filename)
fn split_to_pieces(s: &str) -> (String, String) {
    let chars = s.chars().rev();
    let mut expr: String = "".to_string();
    let mut filename: String = "".to_string();
    let mut is_fn = true;

    for char in chars {
        if is_fn {
            if char == ',' {
                is_fn = false;
            } else {
                filename.push(char);
            }
        } else {
            if char != '"' {
                expr.push(char);
            }
        }
    }

    (
        expr.chars().rev().collect(),
        filename.chars().rev().collect(),
    )
}

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

    let their_file = std::fs::File::open("../../dataset/im2latex_train.csv").unwrap();
    let mut success_idx = 0;

    // iterate through their_file line by line
    for (idx, line_wrap) in std::io::BufReader::new(their_file).lines().enumerate() {
        // if idx >= 2000 {
        //     break;
        // }
        // get the line
        let Ok(line) = line_wrap else {
            println!("ERERRROR");
            continue;
        };
        let (expr, filename) = split_to_pieces(&line);
        let Some(processed_expr) = process_line(expr.clone()) else {
            println!("skipped {expr}");
            continue;
        };

        let line = format!("${}$", processed_expr);

        // write `$` to out_file
        // write!(&out_file, "$").unwrap();
        // pandoc -f latex -t typst -o OUTPUT_FILE.typ -

        match std::process::Command::new("pandoc")
            .arg("-f")
            .arg("latex")
            .arg("-t")
            .arg("typst")
            .arg("--fail-if-warnings=true")
            .arg("-")
            .stdin(std::process::Stdio::piped())
            .stdout(out_file.try_clone().unwrap()) // Clone the file handle for each iteration
            .spawn()
        {
            Ok(mut pandoc_cmd) => {
                // Pass the line as input to the process
                match pandoc_cmd
                    .stdin
                    .as_mut()
                    .unwrap()
                    .write_all(line.as_bytes())
                {
                    Ok(_ok) => {
                        // write `$` to out_file
                        // write!(&out_file, "$").unwrap();

                        // write newline to outfile
                        writeln!(&mut out_file).unwrap();

                        // Wait for the process to finish
                        match pandoc_cmd.wait() {
                            Ok(o) => {
                                if o.success() {
                                    success_idx += 1;
                                    println!("SUCESS:{:?}", success_idx);
                                } else {
                                    println!("ERROR");
                                    return;
                                }

                                continue;
                            }
                            Err(e) => {
                                // println!("err {e}");
                                println!("ERROR");
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        println!("ERROR");
                        // println!("err {e}");
                        continue;
                    }
                }
            }
            Err(e) => {
                println!("ERROR");
                // println!("err {e}");
                continue;
            }
        }
    }
}
