#[allow(unused_variables)]
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    // tuples

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // arrays

    let a = [1, 2, 3, 4, 5];

    let months = ["January", "Febraury", "March", "April", "May", "June", "July", "August", "September", "Ovtober", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

	//let build_time_error = a[6];

	//let index = 6;
	//let runtime_error = a[index];

	
}
