use std::cell::RefCell;
use std::rc::{Rc, Weak};

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
    fn minimum(root: Option<Rc<RefCell<Node<T>>>>){
        match root {
            Some(node) => {
                if node.borrow().left.is_some(){
                    Self::minimum(node.borrow().left.clone());
                }
                else{ println!("The minimum is {}", node.borrow().key) }
            }
            _=>{}
        }
    }

    // To find the maximum value of the tree
    fn maximum(root: Option<Rc<RefCell<Node<T>>>>){
        if let Some(node) = root{
            if node.borrow().right.is_some(){
                Self::maximum(node.borrow().right.clone());
            }else{
                println!("The maximum Node is: {}", node.borrow().key);
            }
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

    // fn successor(root: Option<Box<Node>>, key: i32){
    //     let succ = 0;                
    // }
}

fn bst(arr: &mut [i32]){
    let mut root: Option<Rc<RefCell<Node<i32>>>> = None;
    let mut index = 0;
    while index < arr.len(){
        root = Node::insert_node(root, arr[index]);
        index += 1;
    }

    // // To see the tree structure
    // println!("{:#?}", root);

    
    // // Inorder Tree Walk
    // print!("The in-order tree walk is: ", );
    // Node::inorder(root);
    // print!("\n");

    // // Minimum Tree
    // Node::minimum(root);

    // // Maximum Tree
    // Node::maximum(root);

    // Node Searching
    let destination = 2;
    let searching = Node::search(root.clone(), destination);
    println!("The position of {destination} is \n{:#?}", searching );

    // // Successor
    // Node::successor(root, 15);

}

fn main() {
    let mut array= [15, 6, 18, 3, 7, 17, 20, 2, 4, 13, 9];
    bst(&mut array);
}
