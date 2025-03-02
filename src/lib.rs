use std::ptr::NonNull;
use std::marker::PhantomData;

/*
 *
 *  Credit to https://rust-unofficial.github.io/too-many-lists
 *
 *  HEAVILY referenced to create this tree library so I can make my own lang in Rust C:
 *
 */

type Link<T> = NonNull<TreeNode<T>>;

pub struct Cursor<T> {
    curr: Link<T>,
}

impl<T: std::cmp::PartialEq> Cursor<T> {

    pub fn from(ptr: Link<T>) -> Self {
        Cursor { curr: ptr }
    }

    pub fn get_value(&self) -> &T {
        unsafe {
            return self.curr.as_ref().get_elem();
        }
    }

    pub fn cmp_ptr(&self, ptr: *const TreeNode<T>) -> bool {
        return std::ptr::eq(self.curr.as_ptr(), ptr);
    }

    pub fn left(&mut self) {
        unsafe {
            self.curr = self.curr.read().left;
        }
    }

    pub fn right(&mut self) {
        unsafe {
            self.curr = self.curr.read().right;
        }
    }
}   

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

    // GETTER
    
    pub fn get_head(&self) -> Option<Box<TreeNode<T>>> {
        unsafe {
            if self.head != NonNull::dangling() {
                return Some(Box::new(self.head.read()));
            } else {
                return None;
            }
        }
    }

    pub fn get_left(&self) -> Option<Box<TreeNode<T>>> {
        if self.get_head().is_some() {
            let head = self.get_head().unwrap();
            return head.get_left();
        } else {
            return None;
        }
    }

    pub fn get_right(&self) -> Option<Box<TreeNode<T>>> {
        if self.get_head().is_some() {
            let head = self.get_head().unwrap();
            return head.get_right();
        } else {
            return None;
        }
    }

    pub fn set_head(&mut self, node: &mut TreeNode<T>) {
        self.head = Link::new(node as &mut _).expect("ptr is null");
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

    pub fn search_lvr(&mut self, _val: T) -> Cursor<T> {
        panic!("Unimplmented!");
    }

    pub fn search_vlr(&mut self, val: T) -> Option<Cursor<T>> {
        println!("--myl_tree: Trying search_vlr...");
        if self.head == NonNull::dangling() {
            return None;
        }

        println!("--myl_tree: Not dangling head...");

        unsafe {

            println!("--myl_tree: Starting unsafe...");
            let mut stack: Vec<Link<T>> = vec![];

            stack.push(self.head);

            let mut cur_node: Link<T> = stack[0];

            while !stack.is_empty() {
                println!("--myl_tree: checking stack...");
                if cur_node.read().elem == val {
                    println!("--myl_tree: Detecting equality...");
                    return Some(Cursor::from(cur_node));
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

            None
        }
    }

    pub fn search_vrl(&mut self, _val: T) -> Cursor<T> {
        panic!("Unimplmented!");
    }

    pub fn search_parent_vlr(&mut self, val: T) -> Option<Cursor<T>> {
        if self.head == NonNull::dangling() {
            return None;
        }

        unsafe {

            let mut stack: Vec<Link<T>> = vec![];

            stack.push(self.head);

            let mut cur_node: Link<T> = stack[0];

            while !stack.is_empty() {
                println!("--myl_tree: checking stack...");
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

            Some(Cursor::from(cur_node))
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

    pub fn get_elem(&self) -> &T {
        &self.elem
    }

    pub fn get_left(&self) -> Option<Box<TreeNode<T>>> {
        unsafe {
            if self.left != NonNull::dangling() {
                return Some(Box::new(self.left.read()));
            } else {
                return None;
            }
        }
    }

    pub fn get_right(&self) -> Option<Box<TreeNode<T>>> {
        unsafe {
            if self.right != NonNull::dangling() {
                return Some(Box::new(self.right.read()));
            } else {
                return None;
            }
        }
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
        let mut head_r_clone = TreeNode::new(7);
        let mut head_r_r = TreeNode::new(70);
        head_r.set_right(&mut head_r_r);
        head.set_left(&mut head_l);
        head.set_right(&mut head_r);
        let mut head_l_l = TreeNode::new(1);
        head_l.set_left(&mut head_l_l);

        assert_eq!(tree.search_vlr(5).unwrap().cmp_ptr(&head), true);
        
        assert_eq!(tree.search_vlr(7).unwrap().cmp_ptr(&head_r), true);
        assert_eq!(tree.search_vlr(2).unwrap().cmp_ptr(&head_l), true);
        assert_eq!(*tree.search_vlr(2).unwrap().get_value() == 2, true);
        assert_eq!(tree.search_vlr(70).unwrap().cmp_ptr(&head_r_r), true);
        assert_eq!(tree.search_vlr(1).unwrap().cmp_ptr(&head_l), false);
        assert_eq!(tree.search_vlr(1).unwrap().cmp_ptr(&head_r), false);
        assert_eq!(tree.search_vlr(19810).is_some(), false);


        let mut box_l = Box::new(head_l);
        let mut box_l_l = Box::new(head_l_l);
        let mut box_r = Box::new(head_r);
        let mut box_r_r = Box::new(head_r_r);

        assert_eq!(head.get_left() == Some(box_l), true);
        assert_eq!(head.get_right() == Some(box_r), true);
        assert_eq!(head.get_left() == Some(box_l_l), false);

        assert_eq!(head.get_right().unwrap().get_right() == Some(box_r_r), true);
    }

    #[test]
    fn tree_test_parent_search() {
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

        assert_eq!(tree.search_parent_vlr(70).unwrap().cmp_ptr(&head_r), true);       
    }
}
