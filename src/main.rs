use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{self, Rc, Weak};

type Link<Tree> = Option<Rc<RefCell<Node<Tree>>>>;

#[derive(Debug)]
struct Node<Tree>{
    key : i32,
    left: Link<Tree> ,
    right: Link<Tree>,
    parent: Option<Weak<RefCell<Node<Tree>>>>
}

impl<T:Ord> Node<T> {
    // To insert new node into the tree by checking the current tree either have a root or not
    fn insert_node(root: Option<Rc<RefCell<Node<T>>>>, val: i32) -> Option<Rc<RefCell<Node<T>>>>{
        // let node = root.clone();
        match root {
            Some(node) => {
                let key = node.borrow().key;
                let right = node.borrow().right.clone();
                let left = node.borrow().left.clone();
                if val > key{
                    let new_right = Node::insert_node(right, val);
                    if let Some(ref child) = new_right{
                        child.borrow_mut().parent = Some(Rc::downgrade(&node));
                        // child.borrow_mut().parent = Some(Rc::new(RefCell::new()));
                    }
                    node.borrow_mut().right = new_right;
                }else{
                    let new_left = Node::insert_node(left, val);
                    if let Some(ref child) = new_left{
                        child.borrow_mut().parent = Some(Rc::downgrade(&node));
                    } 
                    node.borrow_mut().left = new_left;
                    
                }
                Some(node)
            }
            None => {
                Some(
                    Rc::new(
                        RefCell::new(
                            Node { 
                                key: val, 
                                left: None, 
                                right: None, 
                                parent: None, 
                            }
                        )
                    )
                )
            }
        }
    }

    // to make inorder tree walk
    fn inorder(root: Option<Rc<RefCell<Node<T>>>>){
        if let Some(node ) = root {
            Self::inorder(node.borrow().left.clone());
            print!("{} ", node.borrow().key);
            Self::inorder(node.borrow().right.clone());
        }
    }
    
    // To find the minimum value of the tree
    fn minimum(root: Option<Rc<RefCell<Node<T>>>>) -> i32{
        if let Some(node)=root{
            if node.borrow().left.is_some(){
                return Self::minimum(node.borrow().left.clone());
                
            }else{
                return node.borrow().key;
                // println!("The minimum is {}", node.borrow().key);
            }
        }
        else{
            -1
        } 
    }

    // To find the maximum value of the tree
    fn maximum(root: Option<Rc<RefCell<Node<T>>>>) -> i32{
        if let Some(node) = root{
            if node.borrow().right.is_some(){
                return Self::maximum(node.borrow().right.clone());
            }else{
                return node.borrow().key;
            }
        }else{
            -1
        }
    }

    fn search(root: Option<Rc<RefCell<Node<T>>>>, goal:i32) -> Option<Rc<RefCell<Node<T>>>>{
        match root {
            Some(ref node) => {
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                if node.borrow().key == goal{
                    return root;
                }
                if goal < node.borrow().key{
                    return Self::search(left, goal);
                }else{
                    return Self::search(right, goal);
                }
            }
            None => {
                return None;
            }
        }
    }

    fn successor(root: Option<Rc<RefCell<Node<T>>>>, key: i32) -> i32 {
        let node = Node::search(root, key);
        if node.is_none(){
            return -1;
        }
        let node = node.unwrap();
        if node.borrow().right.is_some(){
            return Node::minimum(node.borrow().right.clone());
        }else{
            let mut prev_parent = node.borrow().parent.clone();
            while let Some(weak_par) = prev_parent {
                if let Some(parent) = weak_par.upgrade(){
                    if parent.borrow().key > key{
                        return parent.borrow().key;
                    }
                    node = parent.clone();
                    prev_parent = node.borrow().parent.clone();
                }else{
                    break;
                }
            }
        }
        -1
    }

}

fn bst(arr: &mut [i32]) -> Option<Rc<RefCell<Node<i32>>>>{
    let mut root: Option<Rc<RefCell<Node<i32>>>> = None;
    let mut index = 0;
    while index < arr.len(){
        root = Node::insert_node(root, arr[index]);
        index += 1;
    }
    root
}

fn main() {
    let mut array= [15, 6, 18, 3, 7, 17, 20, 2, 4, 13, 9];
    let result = bst(&mut array);
    // println!("{:#?}", result);
        
    // Inorder Tree Walk
    print!("The in-order tree walk is: ", );
    Node::inorder(result.clone());
    print!("\n");
    print!("\n");

    // Minimum Tree
    let minim = Node::minimum(result.clone());
    println!("The minimum node is : {minim}");
    print!("\n");print!("\n");

    // Maximum Tree
    let max = Node::maximum(result.clone());
    println!("The maximum node is : {max}");
    print!("\n");print!("\n");

    // Node Searching
    let destination = 2;
    let searching = Node::search(result.clone(), destination);
    println!("The position of {destination} is \n{:#?}", searching );
    print!("\n");print!("\n");

    // Successor
    let goal = 13;
    let succ = Node::successor(result.clone(), goal);
    if succ == -1{
        println!("There is no successor for the given node");
    }else{
        println!("The successor of {goal} is {succ}");
    }

}
