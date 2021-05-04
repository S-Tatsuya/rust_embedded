fn main() {
    println!("if文");
    let x = -1;
    if x == 42 {
        println!("x is 42");
    } else if x < 0 {
        println!("x is negative");
    } else {
        println!("x is not 42 but positive");
    }

    println!("");
    println!("if文は式である。");
    let x = -42;
    let abs = if x < 0 {-x} else {x};
    println!("absolute value of x = {}", abs);

    println!("");
    println!("loop文");
    let mut count = 0;
    println!("count = {}", count);
    loop {
        count += 1;
        if count % 2 == 1 {
            println!("odd");
            continue;
        }

        println!("even");

        if count == 4 {
            break;
        }
    }
    println!("count = {}", count);

    println!("");
    println!("while文");
    let mut counter = 0;
    while counter < 10 {
        counter += 1;
    }
    println!("counter = {}", counter);

    println!("");
    println!("for文");
    for i in 1..5 {
        println!("i = {}", i);
    }

    println!("");
    println!("イテレータ");
    let a = [1, 2, 3, 4];
    for element in a.iter() {
        println!("a[i] = {}", element);
    }
}