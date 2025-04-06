// Adding new node
fn insert_data(data: i32, mut &root){
    if root == None{
        &root = data;
    }   
}

// Creating new tree 
fn new_tree(){

}

// BST function
fn bst(arr: [i32; 8]) {
    let mut root: Option<i32> = None;
    for x in arr{
        insert_data(x, &root);
    }
}

fn main() {
    let array = [8,4,9,5,6,2,11,10];
    bst(array);
}
