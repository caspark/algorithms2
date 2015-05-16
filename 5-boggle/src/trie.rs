/// Trie branch factor (hardcoded to allow using fixed size array)
const R: usize = 26; // 26 letters in the alphabet

#[derive(Debug)]
struct Node {
    is_present: bool,
    next: [Option<Box<Node>>; R],
}

impl Node {
    fn get<'n>(&'n self, key: &[u8], d: usize) -> Option<&'n Node> {
        if key.len() == d {
            Some(self)
        } else {
            let c = key[d] as usize;
            self.next[c].as_ref().and_then(|next_node| next_node.get(key, d + 1))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Ord)]
pub enum Presence {
    /// neither the key nor any longer variants of the key are present in the trie
    Missing,
    /// key is a prefix for one or more values which are in the trie
    Prefix,
    /// key is present in the trie
    Present,
}

#[derive(Debug)]
pub struct Trie {
    root: Option<Box<Node>>,
}

impl Trie {
    pub fn new() -> Trie {
        Trie { root: None }
    }

    pub fn contains(&self, key: &[u8]) -> Presence {
        self.root.as_ref().and_then(|r|
            r.get(key, 0).map(|x| if x.is_present { Presence::Present } else { Presence::Prefix })
        ).unwrap_or(Presence::Missing)
    }

    pub fn add(&mut self, key: &[u8]) {
        debug_assert!(!key.iter().any(|l| *l as usize >= R), format!("attempted to add key '{:?}' with letters outside R!", key));
        let mut old_root = self.root.take();
        self.root = Some(Box::new(Trie::add_node(&mut old_root, key, 0)));
    }

    fn add_node(existing: &mut Option<Box<Node>>, key: &[u8], d: usize) -> Node {
        let mut real_x = existing.take().unwrap_or_else(|| Box::new(Node {
            // wtf. Apparently I can't use [None; R] because Node isn't Copy
            next: [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
            is_present: false,
        }));
        if key.len() == d {
            real_x.is_present = true;
        } else {
            let c = key[d] as usize;
            real_x.next[c] = Some(Box::new(Trie::add_node(&mut real_x.next[c], key, d + 1)))
        }
        return *real_x;
    }
}

#[cfg(test)]
mod tests {
    use super::{Trie, Presence};

    #[test]
    fn works_properly() {
        let mut t = Trie::new();
        assert_eq!(t.contains(&[1, 2]), Presence::Missing);
        assert_eq!(t.contains(&[1, 2, 3]), Presence::Missing);
        t.add(&[1, 2, 3]);
        assert_eq!(t.contains(&[1, 2]), Presence::Prefix);
        assert_eq!(t.contains(&[1, 2, 3]), Presence::Present);
        t.add(&[1, 2]);
        assert_eq!(t.contains(&[1, 2]), Presence::Present);
    }

}
