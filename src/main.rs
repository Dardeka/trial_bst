#[derive(Debug)]
struct Node{
    key : i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {

    // To insert new node into the tree by checking the current tree either have a root or not
    fn insert_node(root: Option<Box<Node>>, val: i32) -> Option<Box<Node>>{
        match root {
            Some(mut node) => {
                if val > node.key{
                    node.right = Node::insert_node(node.right.take(), val);
                }else{
                    node.left = Node::insert_node(node.left.take(), val);
                }
                Some(node)
            }
            None => {
                Some(Box::new(
                    Node { 
                        key: val, 
                        left: None, 
                        right: None,
                    }
                ))
            }
        }
    }

    // to make inorder tree walk
    fn inorder(root: Option<Box<Node>>){
        if let Some(node ) = root {
            Self::inorder(node.left);
            print!("{} ", node.key);
            Self::inorder(node.right);
        }
    }
    
    // To find the minimum value of the tree
    fn minimum(root: Option<Box<Node>>){
        if let Some(node) = root{
            if node.left.is_some(){
                Self::minimum(node.left);
            }else{
                println!("The minimum Node is: {}", node.key);
            }
        }
    }

    // To find the maximum value of the tree
    fn maximum(root: Option<Box<Node>>){
        if let Some(node) = root{
            if node.right.is_some(){
                Self::maximum(node.right);
            }else{
                println!("The maximum Node is: {}", node.key);
            }
        }
    }

    fn successor(root: Option<Box<Node>>, key: i32){
        if let Some(node) = root{
            if node.key == key {
                if node.right.is_some(){
                    Node::minimum(node.right);
                    // println!("The successor of {} is {:?}", key, );
                }else{
                    println!("The successor of {} is {}", key, node.key);
                }
            }
        }
    }
}

fn bst(arr: &mut [i32]){
    let mut root: Option<Box<Node>> = None;
    let mut index = 0;
    while index < arr.len(){
        root = Node::insert_node(root, arr[index]);
        index += 1;
    }
    
    // // Inorder Tree Walk
    // print!("The in-order tree walk is: ", );
    // Node::inorder(root);
    // print!("\n");

    // // Minimum Tree
    // Node::minimum(root);

    // // Maximum Tree
    // Node::maximum(root);

    // Successor
    Node::successor(root, 15);

}

fn main() {
    let mut array= [15, 6, 18, 3, 7, 17, 20, 2, 4, 13, 9];
    bst(&mut array);
}
