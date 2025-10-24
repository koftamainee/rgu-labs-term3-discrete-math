use std::collections::HashMap;

use crate::math::Set;

#[derive(Default, Clone, Debug)]
pub struct Relation {
    base: Set,
    matrix: Vec<Vec<u64>>,                   // bit-set vectors
    index_map: Option<HashMap<char, usize>>, // if n > 128, check index_of()

    is_full: bool,
    is_empty: bool,
}

impl Relation {
    pub fn new(base: Set, pairs: impl IntoIterator<Item = (char, char)>) -> Self {
        let n = base.len();
        let words_per_row = (n + 63).div_ceil(64);
        let matrix = vec![vec![0u64; words_per_row]; n];

        let mut rel = Self {
            base,
            matrix,
            index_map: None,
            is_full: false,
            is_empty: false,
        };

        if n > 128 {
            rel.build_index_map();
        }

        for (a, b) in pairs {
            if let (Some(i), Some(j)) = (rel.index_of(a), rel.index_of(b)) {
                rel.set_pair(i, j);
            }
        }

        rel.update_flags();

        rel
    }

    fn update_flags(&mut self) {
        let n = self.matrix.len();
        if n == 0 {
            self.is_empty = true;
            self.is_full = false;
            return;
        }

        let bits_per_row = n;
        let full_chunks = bits_per_row / 64;
        let leftover_bits = bits_per_row % 64;

        let mut is_full = true;
        let mut is_empty = true;

        for row in &self.matrix {
            for &chunk in row.iter().take(full_chunks) {
                if chunk != u64::MAX {
                    is_full = false;
                }
                if chunk != 0 {
                    is_empty = false;
                }
            }

            if leftover_bits > 0 {
                if let Some(&last_chunk) = row.get(full_chunks) {
                    let mask = if leftover_bits == 64 {
                        u64::MAX
                    } else {
                        (1u64 << leftover_bits) - 1
                    };
                    if last_chunk & mask != mask {
                        is_full = false;
                    }
                    if last_chunk & mask != 0 {
                        is_empty = false;
                    }
                }
            }

            if !is_full && !is_empty {
                break;
            }
        }

        self.is_full = is_full;
        self.is_empty = is_empty;
    }

    #[inline]
    fn index_of(&self, c: char) -> Option<usize> {
        if let Some(map) = &self.index_map {
            map.get(&c).copied() // O(1) avg, O(n) worst
        } else {
            self.base.index_of(c) // O(logn)
        }
    }

    fn build_index_map(&mut self) {
        let mut map = HashMap::with_capacity(self.base.len());
        for (i, &c) in self.base.iter().enumerate() {
            map.insert(c, i);
        }
        self.index_map = Some(map);
    }

    #[inline]
    fn set_pair(&mut self, i: usize, j: usize) {
        let (word, bit) = (j / 64, j % 64);
        self.matrix[i][word] |= 1 << bit;
    }

    #[inline]
    fn get_pair(&self, i: usize, j: usize) -> bool {
        let (word, bit) = (j / 64, j % 64);
        (self.matrix[i][word] >> bit) & 1 == 1
    }

    #[inline]
    pub fn contains(&self, a: char, b: char) -> bool {
        if self.is_empty {
            return false;
        }

        if let (Some(i), Some(j)) = (self.index_of(a), self.index_of(b)) {
            if self.is_full {
                return true;
            }
            self.get_pair(i, j)
        } else {
            false
        }
    }

    #[inline]
    pub fn is_reflexive(&self) -> bool {
        if self.is_full {
            return true;
        }
        if self.is_empty {
            return false;
        }

        self.base.iter().all(|&c| self.contains(c, c))
    }

    #[inline]
    pub fn is_irreflexive(&self) -> bool {
        if self.is_empty {
            return true;
        }
        if self.is_full {
            return false;
        }

        self.base.iter().all(|&c| !self.contains(c, c))
    }

    pub fn is_symmetric(&self) -> bool {
        if self.is_full || self.is_empty {
            return true;
        }

        let n = self.matrix.len();
        for i in 0..n {
            for j in i + 1..n {
                if self.get_pair(i, j) != self.get_pair(j, i) {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_antisymmetric(&self) -> bool {
        if self.is_empty {
            return true;
        }
        if self.is_full {
            return false;
        }

        let n = self.matrix.len();
        for i in 0..n {
            for j in i + 1..n {
                if self.get_pair(i, j) && self.get_pair(j, i) {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_asymmetric(&self) -> bool {
        if self.is_empty {
            return true;
        }
        if self.is_full {
            return false;
        }

        self.is_irreflexive() && self.is_antisymmetric()
    }

    #[inline]
    pub fn is_transitive(&self) -> bool {
        if self.is_empty || self.is_full {
            return true;
        }

        let n = self.matrix.len();
        let words_per_row = self.matrix[0].len();

        for i in 0..n {
            for j in 0..n {
                if self.get_pair(i, j) {
                    for w in 0..words_per_row {
                        let aj = self.matrix[j][w];
                        let ai = self.matrix[i][w];
                        if (aj & !ai) != 0 {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    #[inline]
    pub fn is_equivalence(&self) -> bool {
        self.is_reflexive() && self.is_symmetric() && self.is_transitive()
    }

    #[inline]
    pub fn is_partial_order(&self) -> bool {
        self.is_reflexive() && self.is_antisymmetric() && self.is_transitive()
    }

    pub fn minimal_elements(&self) -> Set {
        let mut result = Set::new();

        for &a in &self.base {
            let has_incoming = self.base.iter().any(|&b| b != a && self.contains(b, a));
            if !has_incoming {
                result.add(a);
            }
        }

        result
    }

    pub fn maximal_elements(&self) -> Set {
        let mut result = Set::new();

        for &a in &self.base {
            let has_outgoing = self.base.iter().any(|&b| b != a && self.contains(a, b));
            if !has_outgoing {
                result.add(a);
            }
        }

        result
    }

    pub fn equivalence_classes(&self) -> Vec<Set> {
        let mut classes = Vec::new();
        let mut visited = Set::new();

        for &a in &self.base {
            if visited.contains(a) {
                continue;
            }

            let mut class = Set::new();
            for &b in &self.base {
                if self.contains(a, b) && self.contains(b, a) {
                    class.add(b);
                }
            }

            for &b in &class {
                visited.add(b);
            }

            classes.push(class);
        }

        classes
    }
}
