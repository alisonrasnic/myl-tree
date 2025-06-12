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
            panic_when(self.is_dangling(), "get_value on dangling Cursor", 53);
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tree<T> {
    head:   Link<T>,
    _ghost: PhantomData<T>,
}

impl<T: std::cmp::PartialEq + std::fmt::Debug + Clone + Copy> Tree<T> {
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

    pub fn search_cursor_vlr(&mut self, ptr: Cursor<T>) -> Option<Cursor<T>> {
        println!("--myl_tree: Trying search_cursor_vlr...");
        if self.head == NonNull::dangling() {
            println!("head is dangling");
            return None;
        }

        unsafe {

            let mut stack: Vec<Link<T>> = vec![];

            stack.push(self.head);

            let mut cur_node: Link<T> = stack[0];

            while !stack.is_empty() {
                if ptr.cmp_ptr(cur_node.as_ptr()) {
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

    pub fn search_cursor_parent_vlr(&mut self, ptr: Cursor<T>) -> Option<(Cursor<T>, Dir)> {
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
                    if ptr.cmp_ptr(cur_node.read().right.as_ptr()) {
                        return Some((Cursor::from(cur_node), Dir::RIGHT));
                    }

                    stack.push(cur_node.read().right);
                }

                if cur_node.read().left != NonNull::dangling() {
                    if ptr.cmp_ptr(cur_node.read().left.as_ptr()) {
                        return Some((Cursor::from(cur_node), Dir::LEFT));
                    }

                    stack.push(cur_node.read().left);
                }

                cur_node = stack.pop().unwrap();
            }

            None
        }
    }

    /*pub fn cursor_has_child(&mut self, cursor1: Cursor<T>, cursor2: Cursor<T>) -> bool {

        // hi

        let mut stack: Vec<Link<T>> = vec![];

        stack.push(cursor1.get_ptr());

        let cur_node = stack.pop();

        while cur_node.is_some() {

            unsafe {
                let n = cur_node.unwrap();
                let right = cursor1.get_ptr().read().get_right();
                let left = cursor1.get_ptr().read().get_left();
                if cursor2.cmp_ptr() || cursor2.cmp_ptr(n.read().get_right().as_ptr()) {
                    return true; 
                }

                if let Some(r) = right {
                    stack.push(NonNull::new(r as &mut _).expect("unreachable"));
                }

                if let Some(l) = left {
                    stack.push(NonNull::new(l as &mut _).expect("unreachable")); 
                }
            }

            cur_node = stack.pop();
        }


        false
    }*/

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
    /*pub fn swap_cursors(&mut self, _cursor1: Cursor<T>, _cursor2: Cursor<T>, _opt_parent1: Option<(Cursor<T>, Dir)>, _opt_parent2: Option<(Cursor<T>, Dir)>) {
        /*
         *
         *
         *  the goal with this is to swap nodes in the tree
         *  this also means the children of those nodes swap
         *
         *  what if they lead to each other?
         *
         *               1
         *              / \
         *             2
         *            / \
         *           3   4
         *          / \
         *         5   6 
         *
         *
         *
         */

    }

    pub fn swap_cursors_with_children(&mut self, cursor1: Cursor<T>, cursor2: Cursor<T>) {
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

            self.swap_cursors(c1_l, c2_l, Some((cursor1.clone(), Dir::LEFT)), Some((cursor2.clone(), Dir::LEFT)));
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

            self.swap_cursors(c1_r, c2_r, Some((cursor1.clone(), Dir::RIGHT)), Some((cursor2.clone(), Dir::RIGHT)));
         }

        self.swap_cursors(cursor1.clone(), cursor2.clone(), None, None);
    }*/

    pub fn swap_cursors_values(&mut self, cursor1: Cursor<T>, cursor2: Cursor<T>) {

        if cursor1.is_dangling() || cursor2.is_dangling() {
            return;
        }

        unsafe {

            let node1 = cursor1.get_ptr().as_ptr();
            let node2 = cursor2.get_ptr().as_ptr();

            let val1 = (*node1).elem.clone();

            (*node1).elem = (*node2).elem.clone();

            (*node2).elem = val1;
        }
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

#[derive(PartialEq, Debug, Clone, Copy)]
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

        tree.print_vlr();
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
    fn tree_test_search_cursor() {

        let mut tree: Tree<i32> = Tree::new();

        let mut head = TreeNode::new(100);
        let mut head_l = TreeNode::new(100);
        let mut head_r = TreeNode::new(100);
        let mut head_r_r = TreeNode::new(100);
        let mut head_l_l = TreeNode::new(100);

        tree.set_head(&mut head);
        head.set_left(&mut head_l);
        head.set_right(&mut head_r);

        head_r.set_right(&mut head_r_r);
        head_l.set_left(&mut head_l_l);

        let r = tree.search_cursor_vlr(Cursor::from(Link::new(&mut head_l as &mut _).expect("Conversion to raw ptr failure"))).expect("Should not fail");

        assert_eq!(r.cmp_ptr(&mut head_l as &mut _), true);
        assert_eq!(r.cmp_ptr(&mut head_r as &mut _), false);

        let r2 = tree.search_cursor_parent_vlr(Cursor::from(Link::new(&mut head_l as &mut _).expect("Conversion to raw ptr failure"))).expect("Should not fail");
        assert_eq!(r2.0.cmp_ptr(&mut head as &mut _), true); 
        assert_eq!(r2.0.cmp_ptr(&mut head_r as &mut _), false); 

        let r3 = tree.search_cursor_parent_vlr(Cursor::from(Link::new(&mut head_r_r as &mut _).expect("Conversion to raw ptr failure"))).expect("Should not fail");
        assert_eq!(r3.0.cmp_ptr(&mut head_r as &mut _), true);
        assert_eq!(r3.0.cmp_ptr(&mut head_r_r as &mut _), false);
    }
    
    #[test]
    fn tree_test_cursors_values_swap() {

        /*
         *           100
         *          /   \
         *        200   300
         *         \    /
         *          400 500
         *
         *
         *
         *
         */

        let mut tree: Tree<i32> = Tree::new();

        let mut head = TreeNode::new(100);
        let mut head_l = TreeNode::new(200);
        let mut head_r = TreeNode::new(300);

        let mut head_l_r = TreeNode::new(400);
        let mut head_r_l = TreeNode::new(500);

        tree.set_head(&mut head);
        head.set_right(&mut head_r);
        head.set_left(&mut head_l);

        head_l.set_right(&mut head_l_r);

        head_r.set_left(&mut head_r_l);

        let curs1 = tree.search_vlr(&200).unwrap();
        let curs2 = tree.search_vlr(&300).unwrap();

        assert_eq!(head_l.get_elem() == &300, false);

        tree.swap_cursors_values(curs1, curs2);

        assert_eq!(head_l.get_elem() == &300, true);
    }

    #[test]
    fn tree_test_rehead() {
        let mut tree: Tree<i32> = Tree::new();
        let mut head = TreeNode::new(100);

        let mut head_2 = TreeNode::new(200);
        tree.rehead(&mut head, false);
        tree.rehead(&mut head_2, false);
        tree.print_vlr();
    }
}
