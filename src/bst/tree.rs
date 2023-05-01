use std::fmt::Display;
use std::cmp::Ordering;

type Link<T> = Option<Box<TreeNode<T>>>;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    val: T,
    left: Link<T>,
    right: Link<T>,
}

impl <T: Ord> TreeNode<T> {
    pub fn new(val: T) -> Self {
        TreeNode { val: val, left: None, right: None }
    }
}

struct BST<T>
where
    T: Ord
{
    root: Link<T>,
}

impl <T: Ord> Default for BST<T> {
    fn default() -> Self {
        BST { root: None }
    }
}

impl <T: Ord> BST<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, val: T) {
        let mut cur = &mut self.root;
        while let Some(ref mut node) = cur {
            match node.val.cmp(&val) {
                Ordering::Less => cur = &mut node.right,
                Ordering::Equal => return,
                Ordering::Greater => cur = &mut node.left,
            }
        }
        *cur = Some(Box::new(TreeNode::new(val)));
    }
}

struct Zipper<'a, T: Ord> {
    stack: Vec<&'a TreeNode<T>>,
    node: Option<&'a TreeNode<T>>,
}

impl <'a, T: Ord> Zipper<'a, T> {
    fn push_left(&mut self) {
        if self.node.is_none() {
            return;
        }
        while let Some(ref left) = self.node.unwrap().left {
            self.stack.push(self.node.unwrap());
            self.node = Some(left.as_ref());
        }
    }
}

impl <'a, T: Ord> Iterator for Zipper<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.node.is_none() {
            None
        } else {
            let ret = &self.node.unwrap().val;
            if let Some(ref right) = self.node.unwrap().right {
                self.node = Some(right);
                self.push_left();
            } else {
                self.node = self.stack.pop();
            }
            Some(ret)
        }
    }
}

impl <'a, T: Ord> BST<T> {
    pub fn iter(&'a self) -> Zipper<'a, T> {
        Zipper { stack: vec![], node: self.root.as_deref() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator() {
        let mut t = BST::new();
        for v in 0..10 {
            t.insert(v);
        }
        let v1: Vec<i32> = t.iter().map(|v| v.to_owned()).collect();
        let v2: Vec<i32> = (0..10).collect();
        assert!(v1 == v2);
    }
}
