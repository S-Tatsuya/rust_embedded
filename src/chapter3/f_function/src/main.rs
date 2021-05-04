fn main() {
    let x = double(5);
    println!("5 × 2 = {}", x);
}

fn double(number: u32) -> u32 {
    if number > u32::MAX / 2 {
        return u32::MAX;
    }
    number * 2
}