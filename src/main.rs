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
        match root {
            Some(node) => {
                if node.left.is_some(){
                    Self::minimum(node.left);
                }
                else{ println!("The minimum is {}", node.key) }
            }

            _=>{}
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

    fn search(root: Option<&Box<Node>>, goal:i32) -> Option<&Box<Node>>{
        match root {
            Some(ref node) => {
                let left = node.left.as_ref();
                let right = node.right.as_ref();

                if node.key == goal{
                    return root;
                }
                if goal < node.key{
                    return Self::search(left.clone(), goal);
                }else{
                    return Self::search(right.clone(), goal);
                }
            }
            None => {
                return None;
            }
        }
    }

    fn successor(root: Option<Box<Node>>, key: i32){
        let succ = 0;
                
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
