// // Adding new node
// fn insert_data(data: i32, mut &root){
//     if root == None{
//         &root = data;
//     }   
// }

// Creating new tree 
// fn new_tree(){

// }

// BST function
fn bst(arr: &mut[i32], idx: usize) {
    // print!("{:?}\n", arr);

    // let root = arr[idx];

    // State parent, left, and right index
    let parent = idx;
    let left = idx*2+1;
    let right = idx*2+2;

    println!("Array has been accepted! parent: {} ; left: {}; right: {}", arr[idx], arr[left], arr[right]);

    if arr[left]>arr[parent]{
        let temp = arr[right];
        arr[right] = arr[left];
        arr[left] = temp;
    }
    if arr[right]<arr[parent] {
        let temp = arr[left];
        arr[left] = arr[right];
        arr[right] = temp;
    }
    println!(" ");
    println!("Array has been accepted! parent: {} ; left: {}; right: {}", arr[idx], arr[left], arr[right]);
}

fn main() {
    let mut array= [8,9,4,5,6,2,11,10];
    bst(&mut array, 0);
}
