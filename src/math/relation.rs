use std::collections::HashMap;

use crate::math::Set;

#[derive(Default, Clone, Debug)]
pub struct Relation {
    base: Set,
    matrix: Vec<Vec<u64>>,                   // bit-set vectors
    index_map: Option<HashMap<char, usize>>, // if n > 128, check index_of()
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
        };

        if n > 128 {
            rel.build_index_map();
        }

        for (a, b) in pairs {
            if let (Some(i), Some(j)) = (rel.index_of(a), rel.index_of(b)) {
                rel.set_pair(i, j);
            }
        }

        rel
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
        if let (Some(i), Some(j)) = (self.index_of(a), self.index_of(b)) {
            self.get_pair(i, j)
        } else {
            false
        }
    }

    #[inline]
    pub fn is_reflexive(&self) -> bool {
        self.base.iter().all(|&c| self.contains(c, c))
    }

    #[inline]
    pub fn is_irreflexive(&self) -> bool {
        self.base.iter().all(|&c| !self.contains(c, c))
    }

    #[inline]
    pub fn is_symmetric(&self) -> bool {
        self.base.iter().all(|&a| {
            self.base
                .iter()
                .all(|&b| !self.contains(a, b) || self.contains(b, a))
        })
    }

    #[inline]
    pub fn is_antisymmetric(&self) -> bool {
        !self.base.iter().any(|&a| {
            self.base
                .iter()
                .any(|&b| a != b && self.contains(a, b) && self.contains(b, a))
        })
    }

    #[inline]
    pub fn is_asymmetric(&self) -> bool {
        self.base.iter().all(|&a| {
            self.base
                .iter()
                .all(|&b| !(self.contains(a, b) && self.contains(b, a)))
        })
    }

    #[inline]
    pub fn is_transitive(&self) -> bool {
        self.base.iter().all(|&a| {
            self.base.iter().all(|&b| {
                self.base
                    .iter()
                    .all(|&c| !(self.contains(a, b) && self.contains(b, c)) || self.contains(a, c))
            })
        })
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
            if visited.iter().any(|&x| x == a) {
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
