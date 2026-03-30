fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    println!("Hello, world!");
    println!("{} {} {}", 1.2, 2.2, add(1, 2));
    println!("{} {} {}", 1.2, 2.2, sub(1, 2));
}
