use std::collections::LinkedList;
#[allow(dead_code)]
pub fn create_list(){
    let mut list_a = LinkedList::<u32>::new();
    list_a.extend(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    for value in list_a.iter_mut() {
        if *value % 3 == 0 {
            *value *= 3;
        }
    }
    println!("list_a = {:?}", &list_a);
}