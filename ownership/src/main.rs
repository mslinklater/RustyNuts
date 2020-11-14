fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    s = takes_and_gives_back(s);

    println!("{}", calculate_length(&s));

    change(&mut s);

    takes_ownership(s);
    let x = 5;

    makes_copy(x);

    let _reference_to_nothing = dangle();
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("{}", a_string);
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("blah");
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
