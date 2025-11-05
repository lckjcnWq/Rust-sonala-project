fn main() {
    println!("Hello, world!");

    let a = 10;
    let b: i32 = 20;
    let mut c = 30;
    let d = 30_i32;
    let e = add(1, 2);
    println!("{}", e);

}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
