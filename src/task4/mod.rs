use std::fs;

use crate::task4::{graph::Graph, task_results::GraphResults};

mod args_parser;
mod graph;
mod task_results;

fn print_help() {
    println!("Author: koftamainee");
    println!("Group: ITPM-124");
    println!("Keys:");
    println!("  -e <file>   Edge list");
    println!("  -m <file>   Adjacency matrix");
    println!("  -l <file>   Adjacency list");
    println!("  -o <file>   Output file");
    println!("  -n <num>    Start vertex");
    println!("  -d <num>    End vertex");
    println!("  -h          Help");
}

pub fn run_task_4(args: &[String]) -> Result<(), String> {
    let cli_args_result =
        args_parser::parse_args(args).map_err(|e| format!("Error occured in parse_args: {:?}", e));

    if cli_args_result.is_err() {
        print_help();
    }

    let cli_args = cli_args_result?;

    if cli_args.help {
        print_help();
        return Ok(());
    }

    let path = cli_args.input_file.expect("input file is required");
    let input_format = cli_args.input_format.expect("input format is required");

    let graph = match input_format {
        args_parser::InputFormat::List => Graph::parse_adjust_from_file(&path),
        args_parser::InputFormat::Edges => Graph::parse_edgelist_from_file(&path),
        args_parser::InputFormat::Matrix => Graph::parse_matrix_from_file(&path),
    }
    .expect("failed to parse graph");

    let mut results = graph.analyze();
    results.compute_graph_metrics();

    let results_str = format_text(&results);

    output_results(&results_str, cli_args.output_file.as_deref())
        .expect("failed to output results");

    Ok(())
}

pub fn output_results(content: &str, output_file: Option<&str>) -> std::io::Result<()> {
    match output_file {
        Some(path) => {
            println!("Writing results to file: {}", path);
            fs::write(path, content)?;
            println!("Finished writing results to file.");
        }
        None => {
            println!("Printing results to stdout...");
            println!("{}", content);
            println!("Finished printing results.");
        }
    }

    Ok(())
}

pub fn format_text(results: &GraphResults) -> String {
    let mut s = String::new();

    s.push_str(&format!(
        "1. Graph is {}.\n\n",
        if results.directed {
            "directed"
        } else {
            "undirected"
        }
    ));

    if results.directed {
        s.push_str(&format!(
            "2. deg_in = {:?}\n   deg_out = {:?}.\n\n",
            results.deg_in, results.deg_out
        ));
    } else {
        let mut deg: Vec<usize> = results.deg_out.clone();
        deg.iter_mut().for_each(|elem| *elem /= 2);
        s.push_str(&format!("2. deg = {:?}\n\n", deg));
    }

    if results.directed {
        s.push_str("3. Connected components:\nNone\n\n");

        s.push_str("4. Weakly connected components:\n");
        if results.weak_components.is_empty() {
            s.push_str("None\n\n");
        } else {
            for comp in &results.weak_components {
                s.push_str(&format!(
                    "[{}]\n",
                    comp.iter()
                        .map(|v| v + 1)
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            s.push('\n');
        }

        s.push_str("5. Strongly connected components:\n");
        if results.strong_components.is_empty() {
            s.push_str("None\n\n");
        } else {
            for comp in &results.strong_components {
                s.push_str(&format!(
                    "[{}]\n",
                    comp.iter()
                        .map(|v| v + 1)
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            s.push('\n');
        }
    } else {
        s.push_str("3. Connected components:\n");
        if results.weak_components.is_empty() {
            s.push_str("None\n\n");
        } else {
            for comp in &results.weak_components {
                s.push_str(&format!(
                    "[{}]\n",
                    comp.iter()
                        .map(|v| v + 1)
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            s.push('\n');
        }

        s.push_str("4. Weakly connected components:\nNone\n\n");
        s.push_str("5. Strongly connected components:\nNone\n\n");
    }

    s.push_str("6. Graph metrics:\n");
    if results.directed {
        s.push_str("   Diameter: None\n   Radius: None\n   Central vertices: None\n   Peripheral vertices: None\n\n");
    } else {
        let diameter_str = results
            .diameter
            .map_or("+Infinity".to_string(), |d| d.to_string());
        let radius_str = results
            .radius
            .map_or("+Infinity".to_string(), |r| r.to_string());

        let centers_str = if results.centers.is_empty() {
            "None".to_string()
        } else {
            format!(
                "[{}]",
                results
                    .centers
                    .iter()
                    .map(|v| v + 1)
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };

        let periphery_str = if results.periphery.is_empty() {
            "None".to_string()
        } else {
            format!(
                "[{}]",
                results
                    .periphery
                    .iter()
                    .map(|v| v + 1)
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };

        s.push_str(&format!(
            "   D = {}\t Z = {}\n   R = {}\t P = {}\n\n",
            diameter_str, centers_str, radius_str, periphery_str
        ));
    }

    s.push_str("7. Distances and shortest paths:\n");
    let n = results.paths.len();

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            s.push_str(&format!("from {} to {}\n", i + 1, j + 1));

            let (path_str, length_str) = match &results.paths[i][j] {
                Some(path) if !path.is_empty() => {
                    let p = path
                        .iter()
                        .map(|x| (x + 1).to_string())
                        .collect::<Vec<_>>()
                        .join(" -> ");
                    let l = results.distances[i][j].unwrap_or(f64::INFINITY);
                    (p, l.to_string())
                }
                _ => ("no path".to_string(), "+Infinity".to_string()),
            };

            s.push_str(&format!("{} ({})\n", path_str, length_str));
        }
    }

    s
}
