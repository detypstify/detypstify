use std::{fs::File, io::{BufRead, Read, Write}, process::Stdio};

use rayon::iter::ParallelIterator;
use regex::{Captures, Regex};
pub const OUTFILE: &str = "OUT/out";
pub const FILE_TO_READ: &str = "/home/jrestivo/Data/ml_final_project/PRINTED_TEX_230k/final_png_formulas.txt";
pub const OUT_DIR: &str = "OUT";
pub const IMAGE_OUT_DIR: &str = "OUT/images/";
pub const FORMULA_DIR: &str = "OUT/formulas";

fn gen_uid(idx: usize) -> String {
    format!("{:020}", idx)
}

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
        .and_then(process_text_mods)
        .and_then(process_partial)
        .and_then(nuke_trailing)
        .and_then(process_small_space)
        .and_then(process_weird_characters)
}

fn process_small_space(s: String) -> Option<String> {
    Some(s.replace(r"\b ", r"\! "))

}

fn process_weird_characters(s: String) -> Option<String> {
    // \L is a weird L. Replace it with L
    Some(s.replace(r"\L ", r"L ").replace(r"\l ", r"l "))

}

fn process_mathcal(s: String) -> Option<String> {
    Some(s.replace(r"\cal", r"\mathcal"))
}

fn process_sp(s: String) -> Option<String> {
    Some(s.replace(r"\sp ", r"^ ").replace(r"\sb ", r"_"))
}

fn process_bf(s: String) -> Option<String> {
    Some(s.replace(r"\bf", r"\mathbf").replace(r"\boldmath", r"\mathbf").replace(r"\it", r"\mathit").replace(r"\tt", r"\texttt").replace(r"\sf", r"").replace(r"\i ", r"i "))
}

fn process_partial(s: String) -> Option<String> {
    Some(s.replace(r"\d ", r"\partial "))
}

// pandoc doesn't support vspace. Trash result
fn process_text_mods(s: String) -> Option<String> {
    if s.contains(r"\small") || s.contains(r"\tiny") || s.contains(r"\c ") || s.contains(r"\footnotesize") || s.contains(r"\scriptsize") || s.contains(r"\Bigl") || s.contains(r"\Bigr") || s.contains(r"\hline") || s.contains(r"\raisebox") || s.contains(r"\vphantom") || s.contains(r"\textup") || s.contains(r"`") || s.contains(r"\mit ") || s.contains(r"\do ") || s.contains(r"\em ") || s.contains(r"\atop") || s.contains(r"\large ") || s.contains(r"\label") || s.contains(r"\raise "){
        None
    } else {
        Some(s)
    }
}

fn nuke_trailing(mut s: String) -> Option<String> {
    if s.ends_with('\\') {
        s.pop();
    }
    Some(s)
}

fn process_table(s: String) -> Option<String> {
    let regex = Regex::new(r"\\begin\{array\}\s+\{[\sclr]*\}").unwrap();
    let result = regex.replace_all(&s, |caps: &Captures| caps[0].replace(" ", ""));
    Some(result.to_string())
}

// pandoc doesn't support vspace. Trash result
fn process_vspace(s: String) -> Option<String> {
    if s.contains(r"\vspace") || s.contains(r"\thinspace") {
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

// hspace pandoc doesn't support negative
fn process_hspace(s: String) -> Option<String> {
    let negative_hspace_regex = Regex::new(r"\\hspace\s*\{\s*[-~]").unwrap();
    if negative_hspace_regex.is_match(&s) {
        return None;
    }
    let negative_hspace_regex = Regex::new(r"\\hspace\s*\{[\s0-9]*\.").unwrap();
    if negative_hspace_regex.is_match(&s) {
        return None;
    }




    let regex = Regex::new(r"(\\hspace\s*)\{([0-9a-z\s]*)\}").unwrap();
    let result = regex.replace_all(&s, |caps: &Captures| {
        format!("{} {{ {:} }}", &caps[1], caps[2].replace(" ", ""))
    }).to_string();

    let has_hspace_ex_regex = Regex::new(r"\\hspace\s*\{.*ex.*\}").unwrap();
    if has_hspace_ex_regex.is_match(&result){
        return None;
    }

    let regex_2 = Regex::new(r"\\hspace\s*\{ ([0-9][0-9]+)mm \}").unwrap();
    let result_2 = regex_2.replace_all(&result, |caps: &regex::Captures| {
        // Capture the digits and convert to an integer
        let value: i32 = caps[1].parse().unwrap();
        // Multiply by 10 to convert from mm to cm
        let converted_value = value * 10;
        // Replace 'mm' with 'cm' and insert the new value
        format!("\\hspace {{{}cm}}", converted_value)
    }).to_string();

    let regex_3 = Regex::new(r"\\hspace\s*\{ ([\s\.0-9]*)mm \}").unwrap();
    let regex_4 = Regex::new(r"\\hspace\s*\*\s*\{ ").unwrap();
    if regex_3.is_match(&result_2) || regex_4.is_match(&result_2) {
        return None;
    }

    Some(result_2)
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
    // create the directory
    std::fs::create_dir_all(OUT_DIR).unwrap();
    // create child directory
    std::fs::create_dir_all(IMAGE_OUT_DIR).unwrap();
    std::fs::create_dir_all(FORMULA_DIR).unwrap();

    // remove the file if it already exists
    if let Err(e) = std::fs::remove_file(&OUTFILE) {
        if e.kind() != std::io::ErrorKind::NotFound {
            // Handle the error if it's not "file not found."
            panic!("Failed to remove file: {:?}", e);
        }
    }

    // create the file
    let mut out_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&OUTFILE)
        .unwrap();

    let their_file = std::fs::File::open(FILE_TO_READ).unwrap();
    // let out_file = File::create(FILE_TO_READ).unwrap();
    // let mut writer = BufWriter::new(file);
    let mut success_idx = 0;
    let mut success = 0;
    let mut fail_parse = 0;
    let mut fail_with_parse = 0;

    std::io::BufReader::new(their_file).lines().enumerate().into_iter().for_each(|(idx, line_wrap)| {
        let uid = gen_uid(idx);
        // get the line
        let Ok(line) = line_wrap else {
            println!("ERROR READING LINE");
            return;
        };
        // let (expr, filename) = split_to_pieces(&line);
        // println!("proccessing {filename}");
        let Some(processed_expr) = process_line(line.clone()) else {
            println!("skipped {line} for file {uid}");
            fail_parse += 1;
            return;
        };
        // println!("expression: {processed_expr}");

        let line = format!("${}$", processed_expr);

        // println!("RUNNING PROCESS");
        match std::process::Command::new("pandoc")
            .arg("-f")
            .arg("latex")
            .arg("-t")
            .arg("typst")
            .arg("--fail-if-warnings=true")
            .arg("-")
            .stdin(Stdio::piped())
            // .stdout(out_file.try_clone().unwrap()) // Clone the file handle for each iteration
            .stdout(Stdio::piped()) // Clone the file handle for each iteration
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
                        // writeln!(&mut out_file).unwrap();

                        // Wait for the process to finish
                        match pandoc_cmd.wait() {
                            Ok(o) => {
                                if o.success() {
                                    let formula_file_name = format!("{}/{}", FORMULA_DIR, uid);

                                    let mut output = String::new();
                                    if let Some(mut stdout) = pandoc_cmd.stdout.take() {
                                        stdout.read_to_string(&mut output).unwrap();
                                    }
                                    let mut formula_file = File::create(formula_file_name.clone()).unwrap();
                                    formula_file.write_all(output.as_bytes()).unwrap();
                                    formula_file.flush().unwrap();

                                    success_idx += 1;
                                    success += 1;
                                    let file_name_new = format!("{uid}.svg");
                                    match std::process::Command::new("typst")
                                        .arg("compile")
                                        .arg(formula_file_name)
                                        .arg(format!("{}/{}", IMAGE_OUT_DIR, file_name_new))
                                        .stdin(Stdio::piped())
                                        .spawn() {
                                        Ok(mut typst_cmd) => {
                                            match typst_cmd.stdin.as_mut().unwrap().write_all(output.as_bytes()) {
                                                Ok(_) => {
                                                    println!("SUCCESS:{:?}", success_idx);
                                                    if let Err(e) = writeln!(&mut out_file, "{output},{file_name_new}") {
                                                        println!("ERR writing to file {file_name_new}: {e}");
                                                    };
                                                },
                                                Err(e) => {
                                                    println!("error {e} running typst compile on {file_name_new}");

                                                },
                                            }

                                        },
                                        Err(e) => {
                                            println!("error {e} running typst compile on {file_name_new}");

                                        },
                                    }
                                } else {
                                    fail_with_parse += 1;
                                    // println!("ERROR");
                                }

                                return;
                            }
                            Err(_e) => {
                                fail_with_parse += 1;
                                // println!("err {e}");
                                // println!("ERROR");
                            }
                        }
                    }
                    Err(e) => {
                        fail_with_parse += 1;
                        // println!("ERROR");
                        // println!("err {e}");
                    }
                }
            }
            Err(e) => {
                fail_with_parse += 1;
                // println!("ERROR");
                // println!("err {e}");
                return;
            }
        }

    });


    println!("success: {success}, fail_parse: {fail_parse}, fail_with_parse: {fail_with_parse}");
}
