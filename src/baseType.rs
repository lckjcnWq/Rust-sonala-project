pub fn main1(){
    let a : u8 = 255;
    let b = a.wrapping_add(20);
    println!("a={} b={}", a, b);
}