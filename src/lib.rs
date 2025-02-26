use std::ptr::NonNull;
use std::marker::PhantomData;

type Link<T> = NonNull<TreeNode<T>>;

pub struct Tree<T> {
    head:   Link<T>,
    _ghost: PhantomData<T>,
}

impl<T: std::cmp::PartialEq> Tree<T> {
    pub fn new() -> Self {
        Self {
            head:   NonNull::dangling(),
            _ghost: PhantomData,
        }
    }

    pub fn set_head(&mut self, node: &mut TreeNode<T>) {
        self.head = Link::new(node as *mut _).expect("ptr is null");
    }

    pub fn set_left(&mut self, node: &mut TreeNode<T>) {
        unsafe {
            self.head.read().left = Link::new(node as &mut _).expect("ptr is null");
        }
    }

    pub fn set_right(&mut self, node: &mut TreeNode<T>) {
        unsafe {
            self.head.read().right = Link::new(node as &mut _).expect("ptr is null");
        }
    }

    pub fn search_lvr(&mut self, _val: T) -> Box<TreeNode<T>> {
        panic!("Unimplmented!");
    }

    pub fn search_vlr(&mut self, val: T) -> Option<Box<TreeNode<T>>> {
        if self.head == NonNull::dangling() {
            return None;
        }

        unsafe {

            let mut stack: Vec<Link<T>> = vec![];

            stack.push(self.head);

            let mut cur_node: Link<T> = stack[0];

            while !stack.is_empty() {
                if cur_node.read().elem == val {
                    break;
                } else {
                    if cur_node.read().right != NonNull::dangling() {
                        stack.push(cur_node.read().right);
                    }

                    if cur_node.read().left != NonNull::dangling() {
                        stack.push(cur_node.read().left);
                    }   
                }

                cur_node = stack.pop().unwrap();
            }

            Some(Box::new(cur_node.read()))
        }
    }

    pub fn search_vrl(&mut self, _val: T) -> Box<TreeNode<T>> {
        panic!("Unimplmented!");
    }

    pub fn search_parent_vlr(&mut self, val: T) -> Option<Box<TreeNode<T>>> {
        if self.head == NonNull::dangling() {
            return None;
        }

        unsafe {

            let mut stack: Vec<Link<T>> = vec![];

            stack.push(self.head);

            let mut cur_node: Link<T> = stack[0];

            while !stack.is_empty() {
                if cur_node.read().right != NonNull::dangling() {
                    if cur_node.read().right.read().elem == val {
                        break;
                    }

                    stack.push(cur_node.read().right);
                }

                if cur_node.read().left != NonNull::dangling() {
                    if cur_node.read().left.read().elem == val {
                        break;
                    }

                    stack.push(cur_node.read().left);
                }

                cur_node = stack.pop().unwrap();
            }

            Some(Box::new(cur_node.read()))
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct TreeNode<T> {
    elem:  T,
    left:  Link<T>,
    right: Link<T>,
}

impl<T> TreeNode<T> {
    pub fn new(val: T) -> Self {
        Self { elem: val, left: NonNull::dangling(), right: NonNull::dangling() }
    }

    pub fn set_left(&mut self, node: &mut TreeNode<T>) {
        self.left = Link::new(node as *mut _).expect("ptr is null");
    }

    pub fn set_right(&mut self, node: &mut TreeNode<T>) {
        self.right = Link::new(node as *mut _).expect("ptr is null");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_tests() {
        let mut tree = Tree::new();
        let mut head = TreeNode::new(5);
        tree.set_head(&mut head);
        let mut head_l = TreeNode::new(2);
        let mut head_r = TreeNode::new(7);
        let mut head_r_r = TreeNode::new(70);
        head_r.set_right(&mut head_r_r);
        head.set_left(&mut head_l);
        head.set_right(&mut head_r);
        let mut head_l_l = TreeNode::new(1);
        head_l.set_left(&mut head_l_l);

        let box_r = Box::new(head_r);
        let box_l = Box::new(head_l);
        let box_r_r = Box::new(head_r_r);
        assert_eq!(box_r == tree.search_vlr(7).unwrap(), true);
        assert_eq!(box_l == tree.search_vlr(2).unwrap(), true);
        assert_eq!(box_r_r== tree.search_vlr(70).unwrap(), true);
        assert_eq!(box_r == tree.search_vlr(1).unwrap(), false);
        assert_eq!(box_l == tree.search_vlr(1).unwrap(), false);
        assert_eq!(box_r_r== tree.search_vlr(10).unwrap(), false);
        assert_eq!(box_r == tree.search_parent_vlr(70).unwrap(), true);
    }
}
