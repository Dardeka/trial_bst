use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T: Ord>{
    key : i32,
    left: Link<T> ,
    right: Link<T>,
    parent: Option<Weak<RefCell<Node<T>>>>
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
                    node.borrow_mut().right = Node::insert_node(right, val);
                    node.borrow_mut().parent = node.borrow();
                }else{
                    node.borrow_mut().left = Node::insert_node(left, val);
                    node.borrow_mut().parent = node.borrow().key.clone();
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

    // // to make inorder tree walk
    // fn inorder(root: Option<Rc<RefCell<Node<T>>>>){
    //     if let Some(node ) = root {
    //         Self::inorder(node.left);
    //         print!("{} ", node.key);
    //         Self::inorder(node.right);
    //     }
    // }
    
    // // To find the minimum value of the tree
    // fn minimum(root: Option<Box<Node>>){
    //     match root {
    //         Some(node) => {
    //             if node.left.is_some(){
    //                 Self::minimum(node.left);
    //             }
    //             else{ println!("The minimum is {}", node.key) }
    //         }
    //         _=>{}
    //     }
    // }

    // // To find the maximum value of the tree
    // fn maximum(root: Option<Box<Node>>){
    //     if let Some(node) = root{
    //         if node.right.is_some(){
    //             Self::maximum(node.right);
    //         }else{
    //             println!("The maximum Node is: {}", node.key);
    //         }
    //     }
    // }

    // fn search(root: Option<&Box<Node>>, goal:i32) -> Option<&Box<Node>>{
    //     match root {
    //         Some(ref node) => {
    //             let left = node.left.as_ref();
    //             let right = node.right.as_ref();
    //             if node.key == goal{
    //                 return root;
    //             }
    //             if goal < node.key{
    //                 return Self::search(left.clone(), goal);
    //             }else{
    //                 return Self::search(right.clone(), goal);
    //             }
    //         }
    //         None => {
    //             return None;
    //         }
    //     }
    // }

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
    // let destination = 18;
    // let searching = Node::search(root.as_ref(), destination);
    // println!("The position of {destination} is \n{:#?}", searching );

    // // Successor
    // Node::successor(root, 15);

}

fn main() {
    let mut array= [15, 6, 18, 3, 7, 17, 20, 2, 4, 13, 9];
    bst(&mut array);
}
