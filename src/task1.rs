use crate::math::Set;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn find_set(sets: &mut HashMap<char, Set>, name: char) -> Option<&mut Set> {
    sets.get_mut(&name)
}

fn handle_command(sets: &mut HashMap<char, Set>, line: &str) -> Result<(), String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty command".to_string());
    }

    match parts[0] {
        "new" => {
            if parts.len() < 2 {
                return Err("'new' requires set name".to_string());
            }
            let name = parts[1].chars().next().unwrap();
            if sets.contains_key(&name) {
                return Err(format!("Set {} already exists", name));
            }
            sets.insert(name, Set::new());
            println!("New set {}", name);
        }
        "del" => {
            if parts.len() < 2 {
                return Err("'del' requires set name".to_string());
            }
            let name = parts[1].chars().next().unwrap();
            if sets.remove(&name).is_some() {
                println!("Deleted set {}", name);
            } else {
                return Err(format!("Set {} not found", name));
            }
        }
        "add" => {
            if parts.len() < 3 {
                return Err("'add' requires set name and element".to_string());
            }
            let name = parts[1].chars().next().unwrap();
            let x = parts[2].chars().next().unwrap();
            if let Some(s) = find_set(sets, name) {
                s.add(x);
                println!("Added '{}' to {}", x, name);
            } else {
                return Err(format!("Set {} not found", name));
            }
        }
        "rem" => {
            if parts.len() < 3 {
                return Err("'rem' requires set name and element".to_string());
            }
            let name = parts[1].chars().next().unwrap();
            let x = parts[2].chars().next().unwrap();
            if let Some(s) = find_set(sets, name) {
                s.remove(x);
                println!("Removed '{}' from {}", x, name);
            } else {
                return Err(format!("Set {} not found", name));
            }
        }
        "pow" => {
            if parts.len() < 2 {
                return Err("'pow' requires set name".to_string());
            }
            let name = parts[1].chars().next().unwrap();
            if let Some(s) = find_set(sets, name) {
                let power_set = s.power();
                println!("Power set of {}: {{", name);
                for subset in power_set {
                    println!("  {},", subset);
                }
                println!("}}");
            } else {
                return Err(format!("Set {} not found", name));
            }
        }
        "see" => {
            if parts.len() == 2 {
                let name = parts[1].chars().next().unwrap();
                if let Some(s) = find_set(sets, name) {
                    println!("{}: {}", name, s);
                } else {
                    return Err(format!("Set {} not found", name));
                }
            } else {
                for (name, s) in sets.iter() {
                    println!("{}: {}", name, s);
                }
            }
        }
        _ => {
            if parts.len() != 3 {
                return Err("Invalid binary operation format".to_string());
            }
            let name1 = parts[0].chars().next().unwrap();
            let op = parts[1];
            let name2 = parts[2].chars().next().unwrap();

            let s1 = sets.get(&name1).ok_or(format!("Set {} not found", name1))?;
            let s2 = sets.get(&name2).ok_or(format!("Set {} not found", name2))?;

            match op {
                "+" => println!("{} + {} = {}", name1, name2, s1 + s2),
                "&" => println!("{} & {} = {}", name1, name2, s1 & s2),
                "-" => println!("{} - {} = {}", name1, name2, s1 - s2),
                "<" => println!(
                    "{} < {} ? {}",
                    name1,
                    name2,
                    if s1.is_subset(s2) { "true" } else { "false" }
                ),
                "=" => println!(
                    "{} = {} ? {}",
                    name1,
                    name2,
                    if s1 == s2 { "true" } else { "false" }
                ),
                _ => return Err(format!("Unknown operator '{}'", op)),
            }
        }
    }
    Ok(())
}

pub fn run_task_1(file_path: &str) -> Result<(), String> {
    println!("Running task 1 on {}", file_path);

    let file = File::open(file_path).map_err(|_| format!("Cannot open file: {}", file_path))?;
    let reader = io::BufReader::new(file);

    let mut sets: HashMap<char, Set> = HashMap::new();

    for line in reader.lines() {
        let line = line.map_err(|_| "Failed to read line".to_string())?;
        if !line.trim().is_empty() {
            handle_command(&mut sets, &line)?;
        }
    }

    Ok(())
}
