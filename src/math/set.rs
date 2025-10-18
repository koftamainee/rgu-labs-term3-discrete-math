use std::fmt;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Set {
    elements: Vec<char>,
}

impl Set {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, x: char) -> &mut Self {
        match self.elements.binary_search(&x) {
            Ok(_) => {}
            Err(pos) => self.elements.insert(pos, x),
        }
        self
    }

    pub fn remove(&mut self, x: char) -> &mut Self {
        if let Ok(pos) = self.elements.binary_search(&x) {
            self.elements.remove(pos);
        }
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.elements.clear();
        self
    }

    pub fn power(&self) -> Vec<Set> {
        let n = self.elements.len();
        if n > 64 {
            panic!("Elements count in set is too big");
        }

        let mut result = Vec::with_capacity(1 << n);
        for mask in 0..(1 << n) {
            let mut subset = Set::new();
            for i in 0..n {
                if (mask & (1 << i)) != 0 {
                    subset.elements.push(self.elements[i]);
                }
            }
            result.push(subset);
        }
        result
    }

    pub fn union(&self, other: &Set) -> Set {
        let mut result = Vec::new();
        let mut i = 0;
        let mut j = 0;
        while i < self.elements.len() && j < other.elements.len() {
            if self.elements[i] < other.elements[j] {
                result.push(self.elements[i]);
                i += 1;
            } else if self.elements[i] > other.elements[j] {
                result.push(other.elements[j]);
                j += 1;
            } else {
                result.push(self.elements[i]);
                i += 1;
                j += 1;
            }
        }
        result.extend_from_slice(&self.elements[i..]);
        result.extend_from_slice(&other.elements[j..]);
        Set { elements: result }
    }

    pub fn intersection(&self, other: &Set) -> Set {
        let mut result = Vec::new();
        let mut i = 0;
        let mut j = 0;
        while i < self.elements.len() && j < other.elements.len() {
            if self.elements[i] < other.elements[j] {
                i += 1;
            } else if self.elements[i] > other.elements[j] {
                j += 1;
            } else {
                result.push(self.elements[i]);
                i += 1;
                j += 1;
            }
        }
        Set { elements: result }
    }

    pub fn difference(&self, other: &Set) -> Set {
        let mut result = Vec::new();
        let mut i = 0;
        let mut j = 0;
        while i < self.elements.len() && j < other.elements.len() {
            if self.elements[i] < other.elements[j] {
                result.push(self.elements[i]);
                i += 1;
            } else if self.elements[i] > other.elements[j] {
                j += 1;
            } else {
                i += 1;
                j += 1;
            }
        }
        result.extend_from_slice(&self.elements[i..]);
        Set { elements: result }
    }

    pub fn is_subset(&self, other: &Set) -> bool {
        let mut i = 0;
        let mut j = 0;
        while i < self.elements.len() && j < other.elements.len() {
            if self.elements[i] == other.elements[j] {
                i += 1;
                j += 1;
            } else if self.elements[i] > other.elements[j] {
                j += 1;
            } else {
                return false;
            }
        }
        i == self.elements.len()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn index_of(&self, element: char) -> Option<usize> {
        self.elements.binary_search(&element).ok()
    }

    #[inline]
    pub fn contains(&self, element: char) -> bool {
        self.index_of(element).is_some()
    }
}

impl std::ops::Add<&Set> for &Set {
    type Output = Set;
    fn add(self, other: &Set) -> Set {
        self.union(other)
    }
}

impl std::ops::BitAnd<&Set> for &Set {
    type Output = Set;
    fn bitand(self, other: &Set) -> Set {
        self.intersection(other)
    }
}

impl std::ops::Sub<&Set> for &Set {
    type Output = Set;
    fn sub(self, other: &Set) -> Set {
        self.difference(other)
    }
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!(
            "{{{}}}",
            self.elements
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        write!(f, "{}", s)
    }
}

impl<'a> IntoIterator for &'a Set {
    type Item = &'a char;
    type IntoIter = std::slice::Iter<'a, char>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<'a> IntoIterator for &'a mut Set {
    type Item = &'a mut char;
    type IntoIter = std::slice::IterMut<'a, char>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter_mut()
    }
}

impl IntoIterator for Set {
    type Item = char;
    type IntoIter = std::vec::IntoIter<char>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl Set {
    pub fn iter(&self) -> std::slice::Iter<'_, char> {
        self.elements.iter()
    }
}
