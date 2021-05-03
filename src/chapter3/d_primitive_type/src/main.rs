fn main() {
    println!("浮動小数点");
    let x = 4.24242;
    let y: f32 = 2.42424;

    println!("x = {}, y = {:.2}", x, y);

    println!("");
    println!("bool型");
    let x = true;
    let y: bool = false;

    println!("x = {}, y = {}", x, y);

    println!("");
    println!("配列型");
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array = {:?}" ,array);
    println!("array[0] = {}" ,array[0]);

    println!("");
    println!("スライス型");
    let array = [1, 2, 3, 4, 5];
    let slice: &[i32] = &array[1..3];
    println!("slice = {:?}", slice);
    println!("slice[0] = {}", slice[0]);

    println!("");
    println!("スライス型:mut");
    let mut array = [1, 2, 3, 4, 5];
    let slice = &mut array[1..3];
    slice[0] = 6;
    println!("slice = {:?}", slice);
    println!("array = {:?}", array);

    println!("");
    println!("タプル型");
    let t: (u8, i32, usize) = (1, -42, 1_000);
    println!("i32 = {}", t.1);
    
    println!("");
    println!("文字列型");
    let greeting = "hello";
    println!("{}", greeting);

    println!("");
    println!("文字列型:バイト列");
    let greeting = b"hello";
    println!("{:?}", greeting)
}
