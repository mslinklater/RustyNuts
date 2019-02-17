#[allow(unused_variables)]
fn main() {
    println!("Hello, world!");

	another_function();
	parameter_function(5);
	two_parameters(10, 20);

	// error - let is a statement, not an expression, so it does not return anything
	//let x = (let y = 6);

	// A scope IS an expression

	let x = 5;

	let y = {
		let x = 3;
		x + 1
	};
	println!("The value of y is {}", y);

	let x = five();
	println!("The value of x is: {}", x);
}

fn five() -> i32 {
	5
}

fn another_function() {
	println!("Another function!");
}

fn parameter_function(x: i32) {
	println!("THe value of x is {}", x);
}

fn two_parameters(x: i32, y: i32) {
	println!("The value of x is {}", x);
	println!("The value of y is {}", y);
}