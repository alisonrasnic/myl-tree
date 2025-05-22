use std::ptr::NonNull;
use std::marker::PhantomData;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Dir {
    LEFT,
    RIGHT,
}

fn panic_when(cond: bool, msg: &str, ln: i32) {
    if cond {
        panic!("\n[ERR] {} at {}\n", msg, ln);
    }
}

/*
 *
 *  Credit to https://rust-unofficial.github.io/too-many-lists
 *
 *  HEAVILY referenced to create this tree library so I can make my own lang in Rust C:
 *
 */

type Link<T> = NonNull<TreeNode<T>>;

#[derive(Debug)]
pub struct Cursor<T> {
    curr: Link<T>,
}

impl<T> Clone for Cursor<T> {
    fn clone(&self) -> Self {
        Cursor { curr: self.curr.clone() }
    }
}

impl<T: std::cmp::PartialEq + std::fmt::Debug> Cursor<T> {

    pub fn from(ptr: Link<T>) -> Self {
        Cursor { curr: ptr }
    }

    pub fn dangling() -> Self {
        Cursor { curr: NonNull::dangling() }
    }

    pub fn is_dangling(&self) -> bool {
        self.curr == NonNull::dangling()
    }

    pub fn get_value(&self) -> &T {
        unsafe {
            panic_when(self.is_dangling(), "get_value on dangling Cursor", 51);
            return self.curr.as_ref().get_elem();
        }
    }

   pub fn cmp_ptr(&self, ptr: *const TreeNode<T>) -> bool {
        if !self.is_dangling() {
            return std::ptr::eq(self.curr.as_ptr(), ptr);
        }

        false
    }

    pub fn get_ptr(&self) -> NonNull<TreeNode<T>> {
        return self.curr;
    }

    // left & right cause seg faults but why?
    pub fn left(&mut self) {
        unsafe {
            panic_when(self.is_dangling(), "left on dangling Cursor", 71);
            panic_when(self.curr.read().left == NonNull::dangling(), "left is dangling", 72);

            self.curr.replace(self.curr.read().left.read());
        }
    }

    pub fn right(&mut self) {
        unsafe {
            panic_when(self.is_dangling(), "right on dangling Cursor", 80);
            panic_when(self.curr.read().right == NonNull::dangling(), "right is dangling", 81);

            self.curr.replace(self.curr.read().right.read());
        }
    }

    pub fn left_exists(&self) -> bool {
        unsafe {
            if self.curr == NonNull::dangling() {
                return false;
            } else if self.curr.read().left == NonNull::dangling() {
                return false;
            } else {
                return true;
            }
        }
    }

    pub fn right_exists(&self) -> bool {
        unsafe {
            if self.curr == NonNull::dangling() {
                return false;
            } else if self.curr.read().right == NonNull::dangling() {
                return false;
            } else {
                return true;
            }
        }
    }

    pub fn set_left_ptr(&mut self, ptr: Link<T>) {
        unsafe {
            panic_when(self.is_dangling(), "set_left_ptr on dangling Cursor", 113);
            self.curr.as_mut().set_left_ptr(ptr);
        }
    }

    pub fn set_right_ptr(&mut self, ptr: Link<T>) {
        unsafe {
            panic_when(self.is_dangling(), "set_right_ptr on dangling Cursor", 120);
            self.curr.as_mut().set_right_ptr(ptr);
        }
    }

    pub fn set_node_left(&mut self, node: &mut TreeNode<T>) {
        unsafe {
            panic_when(self.is_dangling(), "set_node_ptr on dangling Cursor", 127);
            self.curr.as_mut().set_left(node);
        }
    } 

    pub fn set_node_right(&mut self, node: &mut TreeNode<T>) {
        unsafe {
            panic_when(self.is_dangling(), "set_node_ptr on dangling Cursor", 134);
            self.curr.as_mut().set_right(node);
        }
    }   
}   

pub struct Tree<T> {
    head:   Link<T>,
    _ghost: PhantomData<T>,
}

impl<T: std::cmp::PartialEq + std::fmt::Debug> Tree<T> {
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

    pub fn print_vlr(&self) {
        if self.head == NonNull::dangling() {
            println!("empty tree");
            return;
        }

        unsafe {

            let mut stack: Vec<(&str, Link<T>)> = vec![];

            stack.push(("----------------------------------------", self.head));

            let mut cur_node: (&str, Link<T>) = stack[0];
            let mut rdx = 40;
            let mut ldx = 40;

            while !stack.is_empty() {
                println!("{}{:?}", cur_node.0, cur_node.1.read().elem);
                if cur_node.1.read().right != NonNull::dangling() {
                    rdx += 5;
                    let mut s = String::new();
                    for _ in 0..rdx {
                        s += " ";
                    }
                    s += "R";
                    stack.push((s.leak(), cur_node.1.read().right));
                }

                if cur_node.1.read().left != NonNull::dangling() {
                    ldx -= 5;
                    let mut s = String::new();
                    for _ in 0..ldx {
                        s += " ";
                    }
                    s+="L";
                    stack.push((s.leak(), cur_node.1.read().left));
                }   

                cur_node = stack.pop().unwrap();
            }
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

    // dangles children as well
    pub fn swap_cursors(&mut self, cursor1: Cursor<T>, cursor2: Cursor<T>, opt_parent1: Option<(Cursor<T>, Dir)>, opt_parent2: Option<(Cursor<T>, Dir)>) {
        // Swaps the nodes at two cursors in-place without changing children
        if cursor1.is_dangling() && cursor2.is_dangling() {
            return;
        }

        if cursor1.cmp_ptr(self.head.as_ptr()) && cursor2.cmp_ptr(self.head.as_ptr()) {
            return;
        }

        if cursor1.is_dangling() {
            // in this case, we want to

            if cursor2.cmp_ptr(self.head.as_ptr()) {
                self.head = NonNull::dangling();
            }

            let mut _nparent_2 = (Cursor::<T>::dangling(), Dir::LEFT);
            if opt_parent2.is_some() {
                println!("Found optional parent");
                _nparent_2 = opt_parent2.unwrap();
            } else {
                // potentially a problem if the parent doesn't exist but somehow isn't head?
                _nparent_2 = self.search_parent_vlr(cursor2.get_value()).unwrap().clone();
            }

            if !_nparent_2.0.is_dangling() && _nparent_2.1 == Dir::LEFT {
                _nparent_2.0.set_left_ptr(NonNull::dangling());
            } else if !_nparent_2.0.is_dangling() {
                _nparent_2.0.set_right_ptr(NonNull::dangling());
            } else if _nparent_2.0.is_dangling() {
                panic!("WTF?!");
            }

            let nparent_1 = opt_parent1.as_mut().unwrap();

            if nparent_1.1 == Dir::LEFT {
                (*nparent_1).0.set_left_ptr(cursor2.get_ptr());
            } else {
                (*nparent_1).0.set_right_ptr(cursor2.get_ptr());
            }

            return;
            
        } else if cursor2.is_dangling() {

            if cursor1.cmp_ptr(self.head.as_ptr()) {
                self.head = NonNull::dangling();
            }

            /*
             *
             *  we have two paths
             *
             *  1. either we use the original parent
             *  2. or opt_parent1 is Some and we use that in-place
             *
             *  if opt_parent1 is some
             *  assign our var to it
             *
             *  otherwise assign it to the parent
             *
             */

            let mut _nparent_1 = (Cursor::<T>::dangling(), Dir::LEFT);
            if opt_parent1.is_some() {
                println!("Found optional parent");
                _nparent_1 = opt_parent1.unwrap();
                if _nparent_1.0.is_dangling() {
                    panic!("Okay this is ridiculous");
                }
            } else {
                _nparent_1 = self.search_parent_vlr(cursor1.get_value()).unwrap().clone();
            }                

            println!("Reached point to delete");
            if !_nparent_1.0.is_dangling() && _nparent_1.1 == Dir::LEFT {
                println!("DELETING LEFT PTR");
                _nparent_1.0.set_left_ptr(NonNull::dangling());
            } else if !_nparent_1.0.is_dangling() {
                println!("DELETING RIGHT PTR");
                _nparent_1.0.set_right_ptr(NonNull::dangling());
            } else {
                panic!("parent1 is dangling");
            }

            let parent_2 = opt_parent2.unwrap();

            if parent_2.1 == Dir::LEFT {
                parent_2.0.set_left_ptr(cursor1.get_ptr());
            } else {
                parent_2.0.set_right_ptr(cursor1.get_ptr());
            }

            return;
        }

        let mut parent_1 = self.search_parent_vlr(cursor1.get_value());
        let mut parent_2 = self.search_parent_vlr(cursor2.get_value());

        // dangles
        if cursor1.cmp_ptr(self.head.as_ptr()) {
            let mut par2 = parent_2.expect("Second cursor parent is NULL");

            if par2.1 == Dir::LEFT {
                par2.0.set_left_ptr(NonNull::dangling());
            } else {
                par2.0.set_right_ptr(NonNull::dangling());
            }

            self.head = cursor2.get_ptr();

            return;
        } else if cursor2.cmp_ptr(self.head.as_ptr()) {
            let mut par1 = parent_1.expect("First cursor parent is NULL");

            if par1.1 == Dir::LEFT {
                par1.0.set_left_ptr(NonNull::dangling());
            } else {
                par1.0.set_right_ptr(NonNull::dangling());
            }

            self.head = cursor1.get_ptr();

            return;
        }
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
            panic!("[ERR] p1: {:?} | p2: {:?}", parent_1, parent_2);
        } else {

            let parent_1 = parent_1.as_mut().unwrap();
            let parent_2 = parent_2.as_mut().unwrap();

            if parent_1.1 == Dir::LEFT {
                parent_1.0.set_left_ptr(cursor2.get_ptr());
            } else {
                parent_1.0.set_right_ptr(cursor2.get_ptr());
            }

            if parent_2.1 == Dir::LEFT {
                parent_2.0.set_left_ptr(cursor1.get_ptr());
            } else {
                parent_2.0.set_right_ptr(cursor1.get_ptr());
            }    
        }
    }

    pub fn swap_cursors_with_children(&mut self, cursor1: &mut Cursor<T>, cursor2: &mut Cursor<T>) {
        if cursor1.is_dangling() && cursor2.is_dangling() {
            return;
        }

        {
            let mut c1_l = cursor1.clone();
            let mut c2_l = cursor2.clone();

            if c1_l.left_exists() {
                c1_l.left();
            } else {
                c1_l = Cursor::dangling();
            }

            if c2_l.left_exists() {
                c2_l.left();
            } else {
                c2_l = Cursor::dangling();
            }

            self.swap_cursors(c1_l, c2_l, Some((cursor1, Dir::LEFT)), Some((cursor2, Dir::LEFT)));
        }

        {
            let mut c1_r = cursor1.clone();
            let mut c2_r = cursor2.clone();

            if c1_r.right_exists() {
                c1_r.right();
            } else {
                c1_r = Cursor::dangling();
            }

            if c2_r.right_exists() {
                c2_r.right();
            } else {
                c2_r = Cursor::dangling();
            }

            self.swap_cursors(c1_r, c2_r, Some((cursor1, Dir::RIGHT)), Some((cursor2, Dir::RIGHT)));
         }

        self.swap_cursors(cursor1.clone(), cursor2.clone(), None, None);
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
        //let mut head_r_clone = TreeNode::new(7);
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


        let box_l = Box::new(head_l);
        let box_l_l = Box::new(head_l_l);
        let box_r = Box::new(head_r);
        let box_r_r = Box::new(head_r_r);

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

        //let ten = tree.search_vlr(&10).unwrap();

    }

    #[test]
    fn tree_test_replacer_functions() {
        let mut tree = Tree::new();
        let mut head = TreeNode::new(5);
        let mut head_r = TreeNode::new(6);
        let mut head_r_r = TreeNode::new(7);
        let mut head_r_r_r = TreeNode::new(8);

        tree.set_head(&mut head);
        //let old_res = tree.get_head().expect("WHY IS HEAD DANGLING?!?!?");

        head.set_right(&mut head_r);
        head_r.set_right(&mut head_r_r);
        head_r_r.set_right(&mut head_r_r_r);

        let mut new_node = TreeNode::new(100);

        let cursor_1 = tree.search_vlr(&7);

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

        let _res = tree.search_vlr(&100).unwrap();

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

        {
            print!("\n\n---\n");
            tree.print_vlr();
            print!("\n----\n\n");
            let curs1 = tree.search_vlr(&50).unwrap();
            let curs2 = tree.search_vlr(&200).unwrap();

            assert_eq!(*head.get_left().unwrap().get_elem() == 200, false);

            tree.swap_cursors(curs1, curs2, None, None);

            assert_eq!(*head.get_right().unwrap().get_elem() == 50, true);
            assert_eq!(*head.get_left().unwrap().get_elem() == 200, true);
        }

        {
            
            print!("\n\n---\n");
            tree.print_vlr();
            print!("\n----\n\n");
            let curs1 = tree.search_vlr(&100).unwrap();
            let curs2 = tree.search_vlr(&100).unwrap();

            assert_eq!(curs1.cmp_ptr(&head), true);
            assert_eq!(curs2.cmp_ptr(&head), true);
            tree.swap_cursors(curs1, curs2, None, None);

            assert_eq!(*tree.get_head().unwrap().get_elem() == 100, true);

        }

        {
            
            print!("\n\n---\n");
            tree.print_vlr();
            print!("\n----\n\n");
            let curs1 = tree.search_vlr(&50).unwrap();
            let curs2 = tree.search_vlr(&100).unwrap();
            tree.swap_cursors(curs1, curs2, None, None);

            assert_eq!(*tree.get_head().unwrap().get_elem() == 50, true);

        }
    }

    #[test]
    pub fn tree_test_cursor_swap_optparent() {
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

        print!("\n\n---\n");
        tree.print_vlr();
        print!("\n----\n\n");

        let curs1 = tree.search_vlr(&25).unwrap();
        let curs2 = Cursor::dangling();
        
        let mut par1 = tree.search_vlr(&50).unwrap();
        let mut par2 = tree.search_vlr(&200).unwrap();

        if par1.is_dangling() || par2.is_dangling() {
            panic!("no");
        }

        tree.swap_cursors(curs1, curs2, Some((&mut par1, Dir::LEFT)), Some((&mut par2, Dir::LEFT)));
        print!("\n\n---\n");
        tree.print_vlr();
        print!("\n----\n\n");
        assert_eq!(head_l.get_left().is_none(), true);

    }

    #[test]
    pub fn tree_test_cursor_swap_with_children() {
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

        let mut curs1 = tree.search_vlr(&50).unwrap();
        let mut curs2 = tree.search_vlr(&200).unwrap();

        tree.swap_cursors_with_children(&mut curs1, &mut curs2);

        /*
         *      100
         *     /   \
         *    200   50
         *   /  \ /   \
         *  25        300
         *
         *
         *
         *
         *
         */
        println!("{}", head.get_left().unwrap().get_elem()); 
        assert_eq!(head.get_left().unwrap().get_elem() == &300, true);
    }
}
