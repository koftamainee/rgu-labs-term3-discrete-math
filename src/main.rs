use std::env;

use rgu_labs_term3_discrete_math::task1;
use rgu_labs_term3_discrete_math::task2;
use rgu_labs_term3_discrete_math::task3;
use rgu_labs_term3_discrete_math::task4;

fn print_banner(lines: &[&str]) {
    let max_len = lines.iter().map(|s| s.len()).max().unwrap_or(0);
    let border = "=".repeat(max_len + 4);

    println!("\n{}", border);
    for line in lines {
        println!("| {: <width$} |", line, width = max_len);
    }
    println!("{}\n", border);
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        return Err("Usage: program <flag1> <file1> [<flag2> <file2> ...]".to_string());
    }

    let mut i = 0;
    while i < args.len() {
        let flag = &args[i];
        let file_path = &args[i + 1];
        i += 2;

        match flag.as_str() {
            "-t1" => {
                print_banner(&[
                    "Running Task 1 (Set operations)",
                    &format!("Input file: {}", file_path),
                ]);
                task1::run_task_1(file_path)?;
            }
            "-t2" => {
                print_banner(&[
                    "Running Task 2 (Relation analysis)",
                    &format!("Input file: {}", file_path),
                ]);
                task2::run_task_2(file_path, false)?;
            }
            "-t2s" => {
                print_banner(&[
                    "Running Task 2 silent (Relation analysis)",
                    &format!("Input file: {}", file_path),
                ]);
                task2::run_task_2(file_path, true)?;
            }
            "-t3" => {
                print_banner(&[
                    "Running Task 3 (Boolean functions)",
                    &format!("Input file: {}", file_path),
                ]);
                task3::run_task_3(file_path)?;
            }
            "-t4" => {
                print_banner(&["Running Task 4 (Graphs)"]);

                i -= 1; // custom task, will parse all cli args in run_task_4

                let mut t4_args: Vec<String> = Vec::new();

                while i < args.len() {
                    let a = &args[i];

                    if a == "--" {
                        i += 1;
                        break;
                    }

                    t4_args.push(a.clone());
                    i += 1;
                }

                task4::run_task_4(&t4_args)?;
            }
            _ => return Err(format!("Unknown flag: {}", flag)),
        }
    }

    Ok(())
}
