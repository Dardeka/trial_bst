fn bst(arr: &mut [i32]){
    for x in arr{
        insert_new_node(x);
    }
}

fn insert_new_node(val: &mut i32){
    let mut root: Option<i32> = None;
    check_root(root);
}

fn check_root(root:Option<i32>) -> Option<i32> {
    
}

fn main() {
    let mut array= [8,9,4,5,6,2,11,10];
    bst(&mut array);
}
