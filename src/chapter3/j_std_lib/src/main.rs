#[derive(Debug)]
enum Error {
    Overflow,
    Zero,
}

fn main() {
    println!("Option正常完了");
    let doubled = double(10).unwrap();
    println!("doubled = {}", doubled);

    println!("");
    println!("Option異常終了");
    match double(u32::MAX) {
        Some(x) => println!("doubled = {}", x),
        None => println!("double failed!"),

    }

    println!("");
    println!("if let");
    if let Some(x) = double(10) {
        println!("doubled = {}", x);
    }

    if let Some(x) = double(u32::MAX) {
        println!("doubled = {}", x);
    }

    println!("");
    println!("Result型");
    let doubled = double_result(10).unwrap();
    println!("doubled = {}", doubled);

    println!("");
    println!("Result型エラー処理");
    match double_result(u32::MAX) {
        Ok(x) => println!("double = {}", x),
        Err(_e) => println!("double failed"),
    }

    println!("");
    println!("Result型：if let");
    if let Ok(x) = double_result(u32::MAX) {
        println!("double = {}", x);
    }
    println!("処理のあと");

    println!("");
    println!("Result型：error伝搬");
    if let Err(e) = func() {
        println!("func failed with {:?}", e);
    }
}

fn double(number: u32) -> Option<u32> {
    if number > (u32::MAX / 2) {
        return None
    }

    Some(number * 2)
}

fn double_result(number: u32) -> Result<u32, Error> {
    if number == 0 {
        Err(Error::Zero)
    } else if number > (u32::MAX / 2) {
        Err(Error::Overflow)
    } else {
        Ok(number * 2)
    }
}

fn func() -> Result<u32, Error> {
    let doubled = double_result(u32::MAX)?;
    Ok(doubled)
}