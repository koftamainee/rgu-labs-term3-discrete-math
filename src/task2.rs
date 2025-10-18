use crate::math::{Relation, Set};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const PARALLEL_THRESHOLD: usize = 100;

pub fn run_task_2(file_path: &str, is_silent: bool) -> Result<(), String> {
    let path = Path::new(file_path);
    let file =
        File::open(path).map_err(|e| format!("Failed to open file '{}': {}", file_path, e))?;
    let mut lines = io::BufReader::new(file).lines();

    let base_line = lines
        .next()
        .ok_or("Error: File is empty — expected base set on the first line.".to_string())?
        .map_err(|e| format!("Failed to read the first line: {}", e))?;

    if base_line.trim().is_empty() {
        return Err("Error: Base set line is empty.".to_string());
    }

    let mut base = Set::new();
    for token in base_line.split_whitespace() {
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
    for (line_no, line) in lines.enumerate() {
        let line = line.map_err(|e| format!("Failed to read line {}: {}", line_no + 2, e))?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let tokens: Vec<_> = trimmed.split_whitespace().collect();
        if tokens.len() != 2 {
            return Err(format!(
                "Error: Expected 2 elements per line, got {} on line {} ('{}')",
                tokens.len(),
                line_no + 2,
                trimmed
            ));
        }

        let (a, b) = (tokens[0].chars().next(), tokens[1].chars().next());
        if let (Some(a), Some(b)) = (a, b) {
            if !base.contains(a) {
                return Err(format!(
                    "Error: Element '{}' on line {} not found in base set.",
                    a,
                    line_no + 2
                ));
            }
            if !base.contains(b) {
                return Err(format!(
                    "Error: Element '{}' on line {} not found in base set.",
                    b,
                    line_no + 2
                ));
            }
            pairs.push((a, b));
        } else {
            return Err(format!("Error: Invalid characters on line {}", line_no + 2));
        }
    }

    if pairs.is_empty() {
        return Err("Error: No relation pairs were provided.".to_string());
    }

    if !is_silent {
        println!("Base set: {}", base);

        print!("Relation: {{");
        for (i, (a, b)) in pairs.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("({}, {})", a, b);
        }
        println!("}}");
    }

    let pairs_len = pairs.len();

    let rel = Relation::new(base, pairs);

    let (reflexive, irreflexive, symmetric, antisymmetric, asymmetric, transitive) =
        if pairs_len > PARALLEL_THRESHOLD {
            use rayon::join;

            let (reflexive, irreflexive) = join(|| rel.is_reflexive(), || rel.is_irreflexive());
            let (symmetric, antisymmetric) = join(|| rel.is_symmetric(), || rel.is_antisymmetric());
            let (asymmetric, transitive) = join(|| rel.is_asymmetric(), || rel.is_transitive());

            (
                reflexive,
                irreflexive,
                symmetric,
                antisymmetric,
                asymmetric,
                transitive,
            )
        } else {
            let reflexive = rel.is_reflexive();
            let irreflexive = rel.is_irreflexive();
            let symmetric = rel.is_symmetric();
            let antisymmetric = rel.is_antisymmetric();
            let asymmetric = rel.is_asymmetric();
            let transitive = rel.is_transitive();

            (
                reflexive,
                irreflexive,
                symmetric,
                antisymmetric,
                asymmetric,
                transitive,
            )
        };

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

    Ok(())
}
