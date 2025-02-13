use std::ptr::NonNull;
use std::marker::PhantomData;

type Link<T> = NonNull<TreeNode<T>>;

pub struct Tree<T> {
    head:   Link<T>,
    depth:  u8,
    length: u8,
    _ghost: PhantomData<T>,
}

impl<T: std::cmp::PartialEq> Tree<T> {
    pub fn new() -> Self {
        Self {
            head:   NonNull::dangling(),
            depth:  0,
            length: 0,
            _ghost: PhantomData,
        }
    }

    pub fn search_lvr(&mut self, val: T) -> Box<TreeNode<T>> {
        panic!("Unimplmented!");
    }

    pub fn search_vlr(&mut self, val: T) -> Option<Box<TreeNode<T>>> {
        unsafe {
            if self.head.read().elem == val {
                return Some(Box::new(self.head.read()));
            }

            let mut stack: Vec<Link<T>> = vec![];
            let mut cur_node: Link<T> = self.head.read().left;

            stack.push(cur_node);

            /*
             *
             *      what's the algorithm
             *
             *      first we check if our only node on the stack is the one
             *          if not, we push its left node and process that one
             *          if we encounter a dangling node, we peek the stack
             *          check if it has a right node and process that one
             *          if it dangles again, then we pop because we're done with that node
             *          if the pop is none, then we are done
             *
             */

            while cur_node.read().elem != val {
                if cur_node == NonNull::dangling() {
                    return None;
                }
                
                

            }

            Some(Box::new(cur_node.read()))
        }
    }

    pub fn search_vrl(&mut self, val: T) -> Box<TreeNode<T>> {
        panic!("Unimplmented!");
    }
}

pub struct TreeNode<T> {
    elem:  T,
    left:  Link<T>,
    right: Link<T>,
}

impl<T> TreeNode<T> {
    
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
