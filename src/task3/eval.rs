use super::ast::Ast;
use std::collections::{BTreeMap, BTreeSet};

pub fn collect_vars(ast: &Ast, set: &mut BTreeSet<String>) {
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

pub fn eval_ast(ast: &Ast, env: &BTreeMap<String, bool>) -> bool {
    match ast {
        Ast::Var(v) => *env.get(v).unwrap_or(&false),
        Ast::Not(x) => !eval_ast(x, env),
        Ast::BinOp(op, l, r) => {
            let a = eval_ast(l, env);
            let b = eval_ast(r, env);
            match op {
                super::ast::BinOp::Or => a | b,
                super::ast::BinOp::And => a & b,
                super::ast::BinOp::Xor => a ^ b,
                super::ast::BinOp::Equiv => a == b,
                super::ast::BinOp::Impl => (!a) | b,
                super::ast::BinOp::Nand => !(a & b),
                super::ast::BinOp::Nor => !(a | b),
            }
        }
    }
}

pub fn truth_table_from_ast(ast: &Ast, vars: &[String]) -> Vec<u8> {
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
