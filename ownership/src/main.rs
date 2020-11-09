fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    takes_ownership(s);

    println!("{}", s);

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String)
{
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32)
{
    println!("{}", some_integer);
}
