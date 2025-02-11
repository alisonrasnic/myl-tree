use std::ptr::NonNull;
use std::marker::PhantomData;

enum SearchType {
    LVR,
    VLR,
    VRL,
}

type Link<T> = NonNull<TreeNode<T>>;

pub struct Tree<T> {
    head:   Link<T>,
    depth:  u8,
    length: u8,
    _ghost: PhantomData<T>,
}

impl Tree<T> {
    pub fn new() -> Self {
        Self {
            head:   NonNull::dangling(),
            depth:  0,
            length: 0,
            _ghost: PhantomData,
        }
    }

    pub fn search_node(&mut self, order: SearchType, val: T) -> Box<Node<T>> {
        unsafe {
            match order {
                SearchType::LVR => sub_search_lvr(val),
                SearchType::VLR => sub_search_vlr(val),
                SearchType::VRL => sub_search_vrl(val),
            }
        }
    }

    fn sub_search_lvr(&mut self, val: T) -> Box<Node<T>> {
        panic!("Unimplmented!");
    }

    fn sub_search_vlr(&mut self, val: T) -> Box<Node<T>> {
        unsafe {
            if self.head.elem == val {
                return Box::new(self.head);
            }

            let rax: Box<Node<T>> = ;
            
        }
    }

    fn sub_search_vrl(&mut self, val: T) -> Box<Node<T>> {
        panic!("Unimplmented!");
    }
}

pub struct TreeNode<T> {
    elem:  T,
    left:  Link<T>,
    right: Link<T>,
}

impl TreeNode<T> {
    
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
