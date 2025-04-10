// #[derive(Debug)]
struct Node{
    key : i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    // fn new_node(val: i32)-> Option<Box<Node>>{
    //     Some(
    //         Box::new(
    //             Node { 
    //                 key: val, 
    //                 left: None, 
    //                 right: None, 
    //             }
    //         )
    //     )
    // }

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

    fn inorder(){
        
    }
}

fn bst(arr: &mut [i32]){
    let mut root: Option<Box<Node>> = None;
    let mut index = 0;
    while index < arr.len(){
        root = Node::insert_node(root, arr[index]);
        index += 1;
    }

    println!("{:?}", root)
}

fn main() {
    let mut array= [8,9,4,5,6,2,11,10];
    bst(&mut array);
}
