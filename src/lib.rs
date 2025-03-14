use std::ptr::NonNull;
use std::marker::PhantomData;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Dir {
    LEFT,
    RIGHT,
}

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

    pub fn get_ptr(&self) -> NonNull<TreeNode<T>> {
        return self.curr;
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

    pub fn set_left_ptr(&mut self, ptr: Link<T>) {
        unsafe {
            self.curr.as_mut().set_left_ptr(ptr);
        }
    }

    pub fn set_right_ptr(&mut self, ptr: Link<T>) {
        unsafe {
            self.curr.as_mut().set_right_ptr(ptr);
        }
    }

    pub fn set_node_left(&mut self, node: &mut TreeNode<T>) {
        unsafe {
            self.curr.as_mut().set_left(node);
        }
    } 

    pub fn set_node_right(&mut self, node: &mut TreeNode<T>) {
        unsafe {
            self.curr.as_mut().set_right(node);
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

    pub fn search_lvr(&mut self, _val: T) -> Cursor<T> {
        panic!("Unimplmented!");
    }

    pub fn search_vlr(&mut self, val: &T) -> Option<Cursor<T>> {
        println!("--myl_tree: Trying search_vlr...");
        if self.head == NonNull::dangling() {
            println!("head is dangling");
            return None;
        }

        unsafe {

            let mut stack: Vec<Link<T>> = vec![];

            stack.push(self.head);

            let mut cur_node: Link<T> = stack[0];

            while !stack.is_empty() {
                if cur_node.read().elem == *val {
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

    pub fn search_parent_vlr(&mut self, val: &T) -> Option<(Cursor<T>, Dir)> {
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
                    if cur_node.read().right.read().elem == *val {
                        return Some((Cursor::from(cur_node), Dir::RIGHT));
                    }

                    stack.push(cur_node.read().right);
                }

                if cur_node.read().left != NonNull::dangling() {
                    if cur_node.read().left.read().elem == *val {
                        return Some((Cursor::from(cur_node), Dir::LEFT));
                    }

                    stack.push(cur_node.read().left);
                }

                cur_node = stack.pop().unwrap();
            }

            None
        }
    }

    // SETTER

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

    pub fn swap_cursors(&mut self, cursor1: Cursor<T>, cursor2: Cursor<T>) {
        // Swaps the nodes at two cursors in-place without changing children

        //panic!("Currently malfunctioning line 227");

        let mut parent_1 = self.search_parent_vlr(cursor1.get_value());
        let mut parent_2 = self.search_parent_vlr(cursor2.get_value());

        /*let mut curs_1_l = cursor1.get_left();
        let mut curs_1_r = cursor1.get_right();

        let mut curs_2_l = cursor2.get_left();
        let mut curs_2_r = cursor2.get_right();*/

        //      1
        //     / \
        //    2   3
        //   / \ / \
        //   4 5 6 7
        //
        //   if we wanted to swap 5 and 3
        //
        //      1
        //     / \
        //    2   5
        //   / \ / \
        //   4 3 6 7
        //
        //   swapped in-place
        //
        //   so what happened?
        //
        //   the parent of 5 is our head
        //   the parent of 3 is 2
        //
        //   parent_2.set_right(node_1);
        //   parent_1.set_right(node_2);
        //
        //   okay so now what
        //   however, simply this would end up with our solution looking more like
        //
        //      1
        //     / \
        //    2   5
        //   / \
        //  4   3
        //     / \
        //    6   7

        if parent_1.is_none() || parent_2.is_none() {
            panic!("Either one ptr is null or TODO for swapping head");
        } else {

            let mut parent_1 = parent_1.as_mut().unwrap();
            let mut parent_2 = parent_2.as_mut().unwrap();

            if parent_1.1 == Dir::LEFT {
                parent_1.0.set_left_ptr(cursor2.get_ptr());
                println!("parent_1: LEFT");
            } else {
                parent_1.0.set_right_ptr(cursor2.get_ptr());
                println!("parent_1: RIGHT");
            }

            if parent_2.1 == Dir::LEFT {
                parent_2.0.set_left_ptr(cursor1.get_ptr());
                println!("parent_2: Left");
            } else {
                parent_2.0.set_right_ptr(cursor1.get_ptr());
                println!("parent_2: RIGHT");
            }    
        }

        
    }

    pub fn swap_cursors_with_children(&mut self, cursor1: Cursor<T>, cursor2: Cursor<T>) {
       // if we want children to come with, we need a ptr to each child node
       //  and then reparent each of them
    }

    pub fn rehead(&mut self, node: &mut TreeNode<T>, left: bool) {
        // Changes the head to a new node, which then points to the old head
        
        unsafe {
            if left {
                node.set_left(&mut self.head.as_mut());
            } else {
                node.set_right(&mut self.head.as_mut());
            }

            self.set_head(node);
        }
    }

    pub fn reparent(&mut self, cursor: Cursor<T>, node: &mut TreeNode<T>, left: bool) {
        let mut res = self.search_parent_vlr(cursor.get_value());

        if cursor.cmp_ptr(self.head.as_ptr()) {
            self.rehead(node, left);   
            return;
        }

        if res.is_some() {
            let res_1 = res.as_ref().unwrap().1;
            unsafe {
                if res_1 == Dir::LEFT {
                    res.as_mut().unwrap().0.set_node_left(node);
                } else {
                    res.as_mut().unwrap().0.set_node_right(node);
                }

                if left {
                    node.set_left_ptr(cursor.get_ptr());
                } else {
                    node.set_right_ptr(cursor.get_ptr());
                }
            }
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

    pub fn set_left_ptr(&mut self, ptr: NonNull<TreeNode<T>>) {
        self.left = ptr;
    }

    pub fn set_right_ptr(&mut self, ptr: NonNull<TreeNode<T>>) {
        self.right = ptr;
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

        assert_eq!(tree.search_vlr(&5).unwrap().cmp_ptr(&head), true);
        
        assert_eq!(tree.search_vlr(&7).unwrap().cmp_ptr(&head_r), true);
        assert_eq!(tree.search_vlr(&2).unwrap().cmp_ptr(&head_l), true);
        assert_eq!(*tree.search_vlr(&2).unwrap().get_value() == 2, true);
        assert_eq!(tree.search_vlr(&70).unwrap().cmp_ptr(&head_r_r), true);
        assert_eq!(tree.search_vlr(&1).unwrap().cmp_ptr(&head_l), false);
        assert_eq!(tree.search_vlr(&1).unwrap().cmp_ptr(&head_r), false);
        assert_eq!(tree.search_vlr(&19810).is_some(), false);


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

        assert_eq!(tree.search_parent_vlr(&70).unwrap().0.cmp_ptr(&head_r), true);       
    }

    #[test]
    fn tree_cursor_tests() {
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

        let mut seven = tree.search_vlr(&7).unwrap();

        seven.set_node_right(&mut TreeNode::new(10));

        let ten = tree.search_vlr(&10).unwrap();

    }

    #[test]
    fn tree_test_replacer_functions() {
        let mut tree = Tree::new();
        let mut head = TreeNode::new(5);
        let mut head_r = TreeNode::new(6);
        let mut head_r_r = TreeNode::new(7);
        let mut head_r_r_r = TreeNode::new(8);

        tree.set_head(&mut head);
        let old_res = tree.get_head().expect("WHY IS HEAD DANGLING?!?!?");

        head.set_right(&mut head_r);
        head_r.set_right(&mut head_r_r);
        head_r_r.set_right(&mut head_r_r_r);

        let mut new_node = TreeNode::new(100);

        let mut cursor_1 = tree.search_vlr(&7);

        // 5 
        //  \
        //   6
        //    \
        //     7
        //      \
        //       8
        //
        // 5
        //  \
        //   6
        //    \
        //     100
        //        \
        //         7
        //          \
        //           8

        tree.reparent(cursor_1.unwrap(), &mut new_node, false);

        let box_r_r = Box::new(head_r_r);
        assert_eq!(new_node.get_right().unwrap() == box_r_r, true);

        let res = tree.search_vlr(&100).unwrap();

        let box_new_node = Box::new(new_node);
        assert_eq!(head_r.get_right().unwrap() == box_new_node, true);

        let mut tree2: Tree<i32> = Tree::new();
        let mut head2 = TreeNode::new(1);
        let mut new_head2 = TreeNode::new(300);
        
        tree2.set_head(&mut head2);
        let res = tree2.search_vlr(&1).unwrap();
        tree2.reparent(res, &mut new_head2, false);

        assert_eq!(*tree2.get_head().unwrap().get_elem() == 300, true)
    }

    #[test]
    fn tree_test_cursor_swap() {
        let mut tree: Tree<i32> = Tree::new();

        let mut head = TreeNode::new(100);
        let mut head_r = TreeNode::new(200);
        let mut head_l = TreeNode::new(50);

        let mut head_r_r = TreeNode::new(300);
        let mut head_l_l = TreeNode::new(25);

        tree.set_head(&mut head);

        head.set_right(&mut head_r);
        head.set_left(&mut head_l);

        head_r.set_right(&mut head_r_r);

        head_l.set_left(&mut head_l_l);

        let curs1 = tree.search_vlr(&50).unwrap();
        let curs2 = tree.search_vlr(&200).unwrap();

        assert_eq!(*head.get_left().unwrap().get_elem() == 200, false);

        tree.swap_cursors(curs1, curs2);

        assert_eq!(*head.get_right().unwrap().get_elem() == 50, true);
        assert_eq!(*head.get_left().unwrap().get_elem() == 200, true);
    }
}
