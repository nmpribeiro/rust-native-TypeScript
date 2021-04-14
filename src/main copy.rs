use std::env;

struct Number {
    odd: bool,
    value: i32,
}

// note: `Copy` requires that `Clone` is implemented too
impl std::clone::Clone for Number {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl std::marker::Copy for Number {}

fn print_number(n: Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}

fn invert(n: &mut Number) {
    n.value = -n.value;
}

struct Pair<T> {
    a: T,
    b: T,
}

fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn check() -> Result<String, std::str::Utf8Error> {
    let s = std::str::from_utf8(&[240, 159, 141, 137])?;
    println!("{}", s);
    Ok(String::from(s))
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // let query = &args[1];
    // let filename = &args[2];

    // println!("Searching for {}", query);
    // println!("In file {}", filename);

    let n = Number {
        odd: true,
        value: 51,
    };
    let mut m = n.clone();
    m.value += 100;
    print_number(n);
    print_number(m);

    // this time, `n` is mutable
    let mut n2 = Number {
        odd: true,
        value: 51,
    };
    print_number(n2);
    invert(&mut n2); // `n is borrowed mutably - everything is explicit
    print_number(n2);

    let p1 = Pair { a: 3, b: 9 };
    let p2 = Pair { a: true, b: false };
    print_type_name(&p1); // prints "Pair<i32>"
    print_type_name(&p2); // prints "Pair<bool>"

    let mut v1 = Vec::new();
    v1.push(1);
    let mut v2 = Vec::new();
    v2.push(false);
    print_type_name(&v1); // prints "Vec<i32>"
    print_type_name(&v2); // prints "Vec<bool>"

    match check() {
        Ok(s) => println!("done: {}!", s),
        Err(e) => panic!("Error: {}", e),
    };

    return ();
}
