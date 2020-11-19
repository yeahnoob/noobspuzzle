use std::convert::TryInto;

fn main() {
    println!("Hello, world!");
    let v1: Vec<u32> = vec![1, 2, 3];
    let a1: [u32; 3] = v1.try_into().expect("wrong length");
    let v2: Vec<u32> = vec![1, 2, 3, 4, 5];
    let a2: [u32; 3] = v2.try_into().expect("wrong length");
}
