pub mod ast;
pub mod eval;
pub mod forms;
pub mod parser;

pub use ast::{Ast, BinOp};
pub use eval::{collect_vars, eval_ast, truth_table_from_ast};
pub use forms::{
    anf_from_truth, anf_to_str, dual_from_truth, find_fictitious, remove_fictitious,
    sdnf_from_truth, sknf_from_truth,
};
pub use parser::{parse_expr, tokenize};

pub fn run_task_3(file_path: &str) -> Result<(), String> {
    use eval::{collect_vars, truth_table_from_ast};
    use forms::{
        anf_from_truth, anf_to_str, dual_from_truth, find_fictitious, remove_fictitious,
        sdnf_from_truth, sknf_from_truth,
    };
    use parser::{parse_expr, tokenize};
    use std::fs;

    let s = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    let formulas: Vec<&str> = s
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    if formulas.is_empty() {
        return Err("Input file empty or contains only whitespace".to_string());
    }

    for (formula_index, formula) in formulas.iter().enumerate() {
        println!(
            "\n{}=== Processing Formula {}: '{}' ==={}",
            "=".repeat(20),
            formula_index + 1,
            formula,
            "=".repeat(20)
        );

        let tokens = match tokenize(formula) {
            Ok(t) => t,
            Err(e) => {
                println!("Skipping formula due to tokenization error: {}", e);
                continue;
            }
        };

        let (ast, pos) = match parse_expr(&tokens) {
            Ok(result) => result,
            Err(e) => {
                println!("Skipping formula due to parsing error: {}", e);
                continue;
            }
        };

        if pos != tokens.len() {
            println!("Skipping formula due to extra tokens after full parse — invalid formula");
            continue;
        }

        let mut varset = std::collections::BTreeSet::new();
        collect_vars(&ast, &mut varset);

        if varset.is_empty() {
            let table = truth_table_from_ast(&ast, &[]);
            println!("No variables (constant function). Value = {}", table[0]);
            continue;
        }

        let vars: Vec<String> = varset.into_iter().collect();

        println!("Variables (sorted): {:?}", vars);

        let table = truth_table_from_ast(&ast, &vars);
        println!("Truth table ({} rows):", table.len());
        for (i, &v) in table.iter().enumerate() {
            println!("{:0width$b} -> {}", i, v, width = vars.len());
        }

        let fict = find_fictitious(&vars, &table);
        let mut fict_list = Vec::new();
        for (i, &isf) in fict.iter().enumerate() {
            if isf {
                fict_list.push(vars[i].clone());
            }
        }
        println!("Fictitious variables: {:?}", fict_list);

        let (new_vars, new_table) = remove_fictitious(&vars, &table, &fict);
        println!("After removing fictitious variables: vars = {:?}", new_vars);
        println!("Reduced truth table ({} rows):", new_table.len());
        for (i, &v) in new_table.iter().enumerate() {
            println!("{:0width$b} -> {}", i, v, width = new_vars.len());
        }

        let n_new = new_vars.len();
        let dual = dual_from_truth(&new_table, n_new);
        let sdnf = sdnf_from_truth(&new_vars, &new_table);
        let sknf = sknf_from_truth(&new_vars, &new_table);
        let anf_coefs = anf_from_truth(&new_table, n_new);
        let anf = anf_to_str(&anf_coefs, &new_vars);

        let sdnf_dual = sdnf_from_truth(&new_vars, &dual);
        let sknf_dual = sknf_from_truth(&new_vars, &dual);

        println!("\n--- Results for function without fictitious variables ---");
        println!("Variables: {:?}", new_vars);
        println!("SDNF: {}", sdnf);
        println!("SKNF: {}", sknf);
        println!("ANF: {}", anf);
        println!("Dual function truth table:");
        for (i, &v) in dual.iter().enumerate() {
            println!("{:0width$b} -> {}", i, v, width = n_new);
        }
        println!("SDNF of dual: {}", sdnf_dual);
        println!("SKNF of dual: {}", sknf_dual);
    }

    Ok(())
}
