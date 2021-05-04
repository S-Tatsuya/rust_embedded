enum Type {
    Int(i64),
    Float(f64),
    Boolean(bool),
}

fn print_type(t: Type) {
    match t {
        Type::Int(i) => println!("type is integer. valeu = {}", i),
        Type::Float(f) => println!("type is floating point. value = {}", f),
        Type::Boolean(b) => println!("type is floating point. value = {}", b),
    }
}

fn main() {
    print_type(Type::Int(0));
    print_type(Type::Float(5.3));
    print_type(Type::Boolean(true));
}
