use std::io;

fn l4_easy_stdin() {
	let mut input = String::new();
	let reader = io::stdin();
	reader.read_line(&mut input);
	println!("You entered: {}", input);
}

fn l3_change_variable_value() {
	/*
		Lesson 3:
		This function will change value of variable x
	*/
	let mut x = 9; // mut keyword makes x can be changed
	println!("x = {}", x);
	x += 5; // x = x + 5;
	println!("x = {}", x);
}

fn l2_var_hello_word() {
	/*
		Lesson 2:
		This function print hello world string from variable to screen
	*/
	let str_hello_world = "Hello everyone"; // this variable can't be changed
	println!("{}", str_hello_world);
}

fn l1_hello_world() {
	/*
		Lesson 1:
		This function will print a string into screen
	*/
	println!("Hello world");
}

fn main() {
	l4_easy_stdin();
}