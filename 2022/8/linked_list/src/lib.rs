#![allow(dead_code)]

use std::{
    fmt::{Debug, Display},
    ops::Index,
};

struct List {
    value: u8,
    next: Option<Box<List>>,
}

impl List {
    fn new(value: u8) -> Self {
        let next = None;
        Self { value, next }
    }

    fn append(&mut self, value: u8) {
        let new = List::new(value);
        let last = self.get_last_node_mut();
        last.next = Some(Box::new(new));
    }

    fn get_last_node_mut(&mut self) -> &mut Self {
        if let Some(ref mut next) = self.next {
            next.get_last_node_mut()
        } else {
            self
        }
    }

    fn get_last_node(&self) -> &Self {
        if let Some(ref next) = self.next {
            next.get_last_node()
        } else {
            self
        }
    }

    fn len(&self) -> usize {
        self.next.as_ref().map_or(1, |next| 1 + next.len())
    }

    fn get_mut(&mut self, index: usize) -> &mut Self {
        if index == 0 {
            self
        } else {
            self.next.as_mut().unwrap().get_mut(index - 1)
        }
    }
}

impl Index<usize> for List {
    type Output = List;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            self
        } else {
            if let Some(ref next) = self.next {
                &next[index - 1]
            } else {
                panic!("list should have {index} more elements")
            }
        }
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.next == other.next
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.next.is_none() {
            f.write_fmt(format_args!("{}", self.value))
        } else {
            f.write_fmt(format_args!("{} -> ", self.value))?;
            std::fmt::Display::fmt(&self.next.as_ref().unwrap(), f)
        }
    }
}

impl Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("List")
            .field("value", &self.value)
            .field("next", &self.next)
            .finish()
    }
}

impl From<Vec<u8>> for List {
    fn from(vec: Vec<u8>) -> Self {
        let mut result = List::new(vec[0]);
        for item in &vec[1..] {
            result.append(*item);
        }
        result
    }
}

fn main() {
    let mut l = List::new(5);
    l.append(8);
    l.append(1);
    println!("{}", l[0]);
    println!("{}", l[1]);
    println!("{}", l[2]);
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn append() {
        let mut l = List::new(5);

        assert_eq!(l.len(), 1);
        assert_eq!(l.get_last_node().value, 5);

        l.append(8);

        assert_eq!(l.len(), 2);
        assert_eq!(l.get_last_node().value, 8);
    }

    #[test]
    fn mutable() {
        let mut l = List::new(0);
        l.append(1);
        assert_eq!(l.get_last_node().value, 1);
        assert_eq!(l.len(), 2);

        l.get_last_node_mut().value = 5;
        assert_eq!(l.get_last_node().value, 5);
        assert_eq!(l.len(), 2);
    }

    #[test]
    fn representation() {
        let mut l = List::new(1);
        l.append(2);
        l.append(3);
        let repr = format!("{}", l);
        assert_eq!(repr, "1 -> 2 -> 3");
    }

    #[test]
    #[should_panic(expected = "list should have 1 more elements")]
    fn out_of_bounds() {
        let mut l = List::new(1);
        l.append(2);
        let _ = l[l.len()];
    }

    #[test]
    fn from_vec() {
        let input = vec![0, 3, 2, 1, 4];
        let list = List::from(input);
        let repr = format!("{}", list);
        assert_eq!(repr, "0 -> 3 -> 2 -> 1 -> 4");
    }
}