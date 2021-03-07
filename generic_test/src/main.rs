use std::cmp::*;

fn largest<T>(list: &[T]) -> &T
        where T: PartialOrd {
    let mut large = &list[0];
    for elem in list {
        if elem > large {
            large = elem;
        }
    }

    &large
}

fn main() {
    println!("Hello, world!");

    let i_vec: Vec<i32> = vec![1, 2, 3, 4];
    let c_vec: Vec<char> = vec!['a', 'b', 'c', 'd'];
    let s_vec: Vec<String> = vec![
        String::from("abcd"),
        String::from("efgh"),
        String::from("ijkl"),
        String::from("mnop")
    ];

    println!("i_vec : {}", largest(&i_vec));
    println!("c_vec : {}", largest(&c_vec));
    println!("s_vec : {}", largest(&s_vec));
}
