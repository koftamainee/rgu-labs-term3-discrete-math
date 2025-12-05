use std::collections::{BTreeMap, BTreeSet};
use std::fs;

#[derive(Debug, Clone)]
pub enum Ast {
    Var(String),
    Not(Box<Ast>),
    BinOp(BinOp, Box<Ast>, Box<Ast>),
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Or,
    And,
    Xor,
    Equiv,
    Impl,
    Nand,
    Nor,
}

impl BinOp {
    pub fn from_char(c: char) -> Option<BinOp> {
        match c {
            '+' => Some(BinOp::Or),
            '&' => Some(BinOp::And),
            '@' => Some(BinOp::Xor),
            '~' => Some(BinOp::Equiv),
            '>' => Some(BinOp::Impl),
            '|' => Some(BinOp::Nand),
            '!' => Some(BinOp::Nor),
            _ => None,
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            BinOp::Or => "+",
            BinOp::And => "&",
            BinOp::Xor => "@",
            BinOp::Equiv => "~",
            BinOp::Impl => ">",
            BinOp::Nand => "|",
            BinOp::Nor => "!",
        }
    }
}

#[derive(Debug, Clone)]
enum Token {
    LParen,
    RParen,
    Op(char),
    Var(String),
}

fn is_var_start(ch: char) -> bool {
    ch.is_ascii_alphabetic()
}

fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }
        match ch {
            '(' | '{' | '[' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' | '}' | ']' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '+' | '&' | '@' | '~' | '>' | '|' | '!' | '-' => {
                // '-' is unary NOT token; others are binary ops
                tokens.push(Token::Op(ch));
                chars.next();
            }
            c if is_var_start(c) => {
                let mut name = String::new();
                name.push(c);
                chars.next();
                while let Some(&nc) = chars.peek() {
                    if nc.is_ascii_alphanumeric() || nc == '_' {
                        name.push(nc);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Var(name));
            }
            other => {
                return Err(format!("Unexpected character in input: '{}'", other));
            }
        }
    }
    Ok(tokens)
}

// Parser: grammar (with restriction that binary ops are parenthesized):
// expr := Var | '-' expr | '(' expr <binop> expr ')'
fn parse_expr(tokens: &[Token]) -> Result<(Ast, usize), String> {
    parse_at(tokens, 0)
}

fn parse_at(tokens: &[Token], pos: usize) -> Result<(Ast, usize), String> {
    if pos >= tokens.len() {
        return Err("Unexpected end of tokens".to_string());
    }
    match &tokens[pos] {
        Token::Var(name) => Ok((Ast::Var(name.clone()), pos + 1)),
        Token::Op('-') => {
            // unary not
            let (sub, np) = parse_at(tokens, pos + 1)?;
            Ok((Ast::Not(Box::new(sub)), np))
        }
        Token::LParen => {
            // expect ( expr op expr )
            let (left, p1) = parse_at(tokens, pos + 1)?;
            if p1 >= tokens.len() {
                return Err("Unexpected end, expected operator after left expr".to_string());
            }
            let op = match &tokens[p1] {
                Token::Op(c) => *c,
                _ => return Err("Expected binary operator after left expression".to_string()),
            };
            if BinOp::from_char(op).is_none() {
                return Err(format!("Unknown binary operator '{}'", op));
            }
            let (right, p2) = parse_at(tokens, p1 + 1)?;
            if p2 >= tokens.len() {
                return Err("Unexpected end, expected ')'".to_string());
            }
            match &tokens[p2] {
                Token::RParen => Ok((
                    Ast::BinOp(
                        BinOp::from_char(op).unwrap(),
                        Box::new(left),
                        Box::new(right),
                    ),
                    p2 + 1,
                )),
                _ => Err("Expected closing ')' after binary expression".to_string()),
            }
        }
        Token::Op(c) => Err(format!("Unexpected operator token '{}'", c)),
        Token::RParen => Err("Unexpected closing parenthesis".to_string()),
    }
}

fn collect_vars(ast: &Ast, set: &mut BTreeSet<String>) {
    match ast {
        Ast::Var(v) => {
            set.insert(v.clone());
        }
        Ast::Not(x) => collect_vars(x, set),
        Ast::BinOp(_, l, r) => {
            collect_vars(l, set);
            collect_vars(r, set);
        }
    }
}

fn eval_ast(ast: &Ast, env: &BTreeMap<String, bool>) -> bool {
    match ast {
        Ast::Var(v) => *env.get(v).unwrap_or(&false),
        Ast::Not(x) => !eval_ast(x, env),
        Ast::BinOp(op, l, r) => {
            let a = eval_ast(l, env);
            let b = eval_ast(r, env);
            match op {
                BinOp::Or => a | b,
                BinOp::And => a & b,
                BinOp::Xor => a ^ b,
                BinOp::Equiv => a == b,
                BinOp::Impl => (!a) | b,
                BinOp::Nand => !(a & b),
                BinOp::Nor => !(a | b),
            }
        }
    }
}

fn var_literal(name: &str, value: bool) -> String {
    if value {
        name.to_string()
    } else {
        format!("-{}", name)
    }
}

fn minterm_str(vars: &[String], bits: usize, mask: usize) -> String {
    let mut parts = Vec::new();
    (0..bits).for_each(|i| {
        let v = &vars[i];
        let bit = ((mask >> (bits - 1 - i)) & 1) != 0;
        // minterm: xi if bit==1 else -xi
        parts.push(if bit { v.clone() } else { format!("-{}", v) });
    });
    if parts.is_empty() {
        "1".to_string()
    } else {
        parts.join(" & ")
    }
}

fn maxterm_str(vars: &[String], bits: usize, mask: usize) -> String {
    // maxterm clause: OR of literals, literal is xi if bit==0 else -xi
    let mut parts = Vec::new();
    (0..bits).for_each(|i| {
        let v = &vars[i];
        let bit = ((mask >> (bits - 1 - i)) & 1) != 0;
        parts.push(if !bit { v.clone() } else { format!("-{}", v) });
    });
    if parts.is_empty() {
        "0".to_string()
    } else {
        parts.join(" + ")
    }
}

// Zhegalkin (ANF) via Möbius transform (XOR basis)
fn anf_from_truth(values: &[u8], nvars: usize) -> Vec<u8> {
    let mut a = values.to_owned();
    let size = 1 << nvars;
    for i in 0..nvars {
        for mask in 0..size {
            if (mask & (1 << i)) != 0 {
                a[mask] ^= a[mask ^ (1 << i)];
            }
        }
    }
    a
}

fn anf_to_str(coefs: &[u8], vars: &[String]) -> String {
    let n = vars.len();
    let size = 1 << n;

    let terms: Vec<String> = (0..size)
        .filter_map(|mask| {
            if coefs[mask] == 0 {
                return None;
            }
            if mask == 0 {
                Some("1".to_string())
            } else {
                let part: Vec<String> = (0..n)
                    .filter(|&i| (mask & (1 << (n - 1 - i))) != 0)
                    .map(|i| vars[i].clone())
                    .collect();
                Some(part.join(" & "))
            }
        })
        .collect();

    if terms.is_empty() {
        "0".to_string()
    } else {
        terms.join(" @ ")
    }
}

fn truth_table_from_ast(ast: &Ast, vars: &[String]) -> Vec<u8> {
    let n = vars.len();
    let size = 1 << n;
    let mut out = vec![0u8; size];
    let mut env = BTreeMap::new();
    for v in vars.iter() {
        env.insert(v.clone(), false);
    }
    (0..size).for_each(|mask| {
        (0..n).for_each(|i| {
            let bit = ((mask >> (n - 1 - i)) & 1) != 0;
            env.insert(vars[i].clone(), bit);
        });
        out[mask] = if eval_ast(ast, &env) { 1 } else { 0 };
    });
    out
}

// Determine fictitious variables: variable i is fictitious if for all assignments,
// flipping that bit doesn't change function output.
fn find_fictitious(vars: &[String], table: &[u8]) -> Vec<bool> {
    let n = vars.len();
    let size = 1 << n;
    let mut res = vec![false; n];
    (0..n).for_each(|i| {
        let mut ok = true;
        for mask in 0..size {
            let flipped = mask ^ (1 << (n - 1 - i));
            if table[mask] != table[flipped] {
                ok = false;
                break;
            }
        }
        res[i] = ok;
    });
    res
}

// Remove fictitious vars: builds new vars list and new truth table by projecting
fn remove_fictitious(vars: &[String], table: &[u8], is_fict: &[bool]) -> (Vec<String>, Vec<u8>) {
    let n = vars.len();
    let mut new_vars = Vec::new();
    let mut idx_map = Vec::new(); // maps old index -> either new index or -1
    for i in 0..n {
        if !is_fict[i] {
            idx_map.push(new_vars.len() as isize);
            new_vars.push(vars[i].clone());
        } else {
            idx_map.push(-1);
        }
    }
    if new_vars.is_empty() {
        // constant function; return single-var 0-length table of size 1
        let val = if table.contains(&1) { 1 } else { 0 };
        return (vec![], vec![val]);
    }
    let m = new_vars.len();
    let new_size = 1 << m;
    let mut new_table = vec![0u8; new_size];
    (0..(1 << n)).for_each(|mask| {
        // build reduced mask
        let mut new_mask = 0usize;
        (0..n).for_each(|i| {
            if idx_map[i] >= 0 {
                let bit = ((mask >> (n - 1 - i)) & 1) != 0;
                if bit {
                    new_mask |= 1 << (m - 1 - idx_map[i] as usize);
                }
            }
        });
        new_table[new_mask] = table[mask];
    });
    (new_vars, new_table)
}

fn dual_from_truth(table: &[u8], nvars: usize) -> Vec<u8> {
    // f^d(x) = !f(!x)
    let size = 1 << nvars;
    let mut out = vec![0u8; size];
    (0..size).for_each(|mask| {
        let inv_mask = (!mask) & (size - 1);
        out[mask] = 1 - table[inv_mask];
    });
    out
}

fn sdnf_from_truth(vars: &[String], table: &[u8]) -> String {
    let n = vars.len();
    let size = 1 << n;
    let mut terms = Vec::new();
    (0..size).for_each(|mask| {
        if table[mask] == 1 {
            let t = minterm_str(vars, n, mask);
            terms.push(format!("({})", t));
        }
    });
    if terms.is_empty() {
        "0".to_string()
    } else {
        terms.join(" + ")
    }
}

fn sknf_from_truth(vars: &[String], table: &[u8]) -> String {
    let n = vars.len();
    let size = 1 << n;
    let mut clauses = Vec::new();
    (0..size).for_each(|mask| {
        if table[mask] == 0 {
            let c = maxterm_str(vars, n, mask);
            clauses.push(format!("({})", c));
        }
    });
    if clauses.is_empty() {
        "1".to_string()
    } else {
        clauses.join(" & ")
    }
}

pub fn run_task_3(file_path: &str) -> Result<(), String> {
    let s = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;
    // file may contain many lines; treat first non-empty line as formula
    let formula = s
        .lines()
        .find(|l| !l.trim().is_empty())
        .unwrap_or("")
        .trim();
    if formula.is_empty() {
        return Err("Input file empty or contains only whitespace".to_string());
    }

    let tokens = tokenize(formula)?;
    let (ast, pos) = parse_expr(&tokens)?;
    if pos != tokens.len() {
        return Err("Extra tokens after full parse — invalid formula".to_string());
    }

    // collect variables
    let mut varset = BTreeSet::new();
    collect_vars(&ast, &mut varset);
    if varset.is_empty() {
        // constant function
        let table = truth_table_from_ast(&ast, &[]);
        println!("Parsed formula: {}", formula);
        println!("No variables (constant function). Value = {}", table[0]);
        return Ok(());
    }
    let vars: Vec<String> = varset.into_iter().collect();

    println!("Parsed formula: {}", formula);
    println!("Variables (sorted): {:?}", vars);

    let table = truth_table_from_ast(&ast, &vars);
    println!("Truth table ({} rows):", table.len());
    for (i, &v) in table.iter().enumerate() {
        println!("{:0width$b} -> {}", i, v, width = vars.len());
    }

    // find fictitious
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

    // compute dual
    let n_new = new_vars.len();
    let dual = dual_from_truth(&new_table, n_new);
    // SDNF, SKNF, ANF
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

    Ok(())
}
