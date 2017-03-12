fn main() {
	println!("{} is my name.", "Sumit");
	println!("{1}, this is {0}. {1}, this is {0}.","Sumit","Deadpool");
	println!("{name} is {age} years old.", name="Sumit", age=27);

	// Special formatting can be specified after a `:`.
	println!("{} of {:b} people know binary, the other half don't", 1, 2);

	// You can pad numbers with extra zeroes. This will output "000001".
	println!("{number:>0width$}", number=1, width=6);

	println!("{0}, {1}","Happy", "Birthday");

	#[allow(dead_code)]
	struct Structure(i32);

	//Cannot print structs like this
	//println!("This struct `{}` won't print...", Structure(3));

	let pi = 3.141592;
	println!("Pi is roughly {:.*}", 3, pi);
}
