const FOURTY_TWO: i32 = 42;
fn main() {
    println!("変数");
    let x: i32 = 42;
    println!("x = {}", x);

    println!("変数:mut");
    let mut y: i32 = 42;
    println!("y = {}", y);
    y = 24;
    println!("y = {}", y);

    println!("定数");
    println!("FOURTY_TWO = {}", FOURTY_TWO);
}
