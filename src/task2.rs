use crate::math::{Relation, Set};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run_task_2(file_path: &str, is_silent: bool) -> Result<(), String> {
    let file = File::open(Path::new(file_path))
        .map_err(|e| format!("Failed to open file '{}': {}", file_path, e))?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let bytes_read = reader
        .read_line(&mut line)
        .map_err(|e| format!("Failed to read the first line: {}", e))?;
    if bytes_read == 0 {
        return Err("Error: File is empty — expected base set on the first line.".to_string());
    }

    let base_line = line.trim();
    if base_line.is_empty() {
        return Err("Error: Base set line is empty.".to_string());
    }

    let mut base = Set::new();
    for token in base_line.split_ascii_whitespace() {
        if let Some(ch) = token.chars().next() {
            base.add(ch);
        } else {
            return Err(format!("Error: Invalid token in base set: '{}'", token));
        }
    }
    if base.is_empty() {
        return Err("Error: Base set must contain at least one element.".to_string());
    }

    let mut pairs = Vec::new();
    let mut line_no = 1;
    line.clear();

    while reader
        .read_line(&mut line)
        .map_err(|e| format!("I/O error: {}", e))?
        > 0
    {
        line_no += 1;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            line.clear();
            continue;
        }

        let mut parts = trimmed.split_ascii_whitespace();
        let a_token = parts.next();
        let b_token = parts.next();
        if parts.next().is_some() || a_token.is_none() || b_token.is_none() {
            return Err(format!(
                "Error: Expected 2 elements per line, got '{}', line {}",
                trimmed, line_no
            ));
        }

        let (a, b) = (
            a_token.unwrap().chars().next(),
            b_token.unwrap().chars().next(),
        );
        if let (Some(a), Some(b)) = (a, b) {
            if !base.contains(a) {
                return Err(format!(
                    "Error: Element '{}' on line {} not found in base set.",
                    a, line_no
                ));
            }
            if !base.contains(b) {
                return Err(format!(
                    "Error: Element '{}' on line {} not found in base set.",
                    b, line_no
                ));
            }
            pairs.push((a, b));
        } else {
            return Err(format!("Error: Invalid characters on line {}", line_no));
        }

        line.clear();
    }

    if pairs.is_empty() {
        return Err("Error: No relation pairs were provided.".to_string());
    }

    if !is_silent {
        println!("Base set: {}", base);
        println!(
            "Relation: {{{}}}",
            pairs
                .iter()
                .map(|(a, b)| format!("({}, {})", a, b))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    let rel = Relation::new(base, pairs);

    let reflexive = rel.is_reflexive();
    let irreflexive = rel.is_irreflexive();
    let symmetric = rel.is_symmetric();
    let antisymmetric = rel.is_antisymmetric();
    let asymmetric = rel.is_asymmetric();
    let transitive = rel.is_transitive();

    let equivalence = reflexive && symmetric && transitive;
    let partial_order = reflexive && antisymmetric && transitive;

    let props = [
        ("Reflexive", reflexive),
        ("Irreflexive", irreflexive),
        ("Symmetric", symmetric),
        ("Antisymmetric", antisymmetric),
        ("Asymmetric", asymmetric),
        ("Transitive", transitive),
        ("Equivalence", equivalence),
        ("Partial order", partial_order),
    ];

    println!("\nRelation properties:");
    for (name, value) in props {
        println!("{:<15}: {}", name, if value { "+" } else { "-" });
    }

    if !is_silent {
        if equivalence {
            let classes = rel.equivalence_classes();
            println!("\nEquivalence classes:");
            for (i, class) in classes.iter().enumerate() {
                println!("  {}: {}", i + 1, class);
            }
            println!("Index of partition: {}", classes.len());
        }

        if partial_order {
            println!("\nMinimal elements: {}", rel.minimal_elements());
            println!("Maximal elements: {}", rel.maximal_elements());
        }
    }

    Ok(())
}
