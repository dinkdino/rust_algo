mod binary_search;

fn main() {

    let arr = [1,2,3,4,5,6,7,8,9];

    match binary_search::binary_search(&arr, 3) {
        None => println!("Could not find element"),
        Some(index) => println!("Element found at {}", index)
    }

    match binary_search::binary_search_rec(&arr, 6, 0, arr.len()) {
        None => println!("Could not find element"),
        Some(index) => println!("Element found at {}", index)
    }
}
