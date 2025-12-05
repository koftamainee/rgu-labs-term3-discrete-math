fn minterm_str(vars: &[String], bits: usize, mask: usize) -> String {
    let mut parts = Vec::new();
    (0..bits).for_each(|i| {
        let v = &vars[i];
        let bit = ((mask >> (bits - 1 - i)) & 1) != 0;
        parts.push(if bit { v.clone() } else { format!("-{}", v) });
    });
    if parts.is_empty() {
        "1".to_string()
    } else {
        parts.join(" & ")
    }
}

fn maxterm_str(vars: &[String], bits: usize, mask: usize) -> String {
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

pub fn anf_from_truth(values: &[u8], nvars: usize) -> Vec<u8> {
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

pub fn anf_to_str(coefs: &[u8], vars: &[String]) -> String {
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

pub fn find_fictitious(vars: &[String], table: &[u8]) -> Vec<bool> {
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

pub fn remove_fictitious(
    vars: &[String],
    table: &[u8],
    is_fict: &[bool],
) -> (Vec<String>, Vec<u8>) {
    let n = vars.len();
    let mut new_vars = Vec::new();
    let mut idx_map = Vec::new();
    for i in 0..n {
        if !is_fict[i] {
            idx_map.push(new_vars.len() as isize);
            new_vars.push(vars[i].clone());
        } else {
            idx_map.push(-1);
        }
    }
    if new_vars.is_empty() {
        let val = if table.contains(&1) { 1 } else { 0 };
        return (vec![], vec![val]);
    }
    let m = new_vars.len();
    let new_size = 1 << m;
    let mut new_table = vec![0u8; new_size];
    (0..(1 << n)).for_each(|mask| {
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

pub fn dual_from_truth(table: &[u8], nvars: usize) -> Vec<u8> {
    let size = 1 << nvars;
    let mut out = vec![0u8; size];
    (0..size).for_each(|mask| {
        let inv_mask = (!mask) & (size - 1);
        out[mask] = 1 - table[inv_mask];
    });
    out
}

pub fn sdnf_from_truth(vars: &[String], table: &[u8]) -> String {
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

pub fn sknf_from_truth(vars: &[String], table: &[u8]) -> String {
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
