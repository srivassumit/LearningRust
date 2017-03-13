use std::io;

fn main() {

	let mut meal_cost = String::new();
	io::stdin().read_line(&mut meal_cost)
		.expect("Unable to read line");
	let meal_cost: f32 = meal_cost.trim().parse()
		.expect("Please enter a number");

	let mut tip_percent = String::new();
	io::stdin().read_line(&mut tip_percent)
		.expect("Unable to read line");
	let tip_percent: f32 = tip_percent.trim().parse()
		.expect("Please enter a number");

	let mut tax_percent = String::new();
	io::stdin().read_line(&mut tax_percent)
		.expect("Unable to read line");
	let tax_percent: f32 = tax_percent.trim().parse()
		.expect("Please enter a number");

	let total_cost: f32;
	total_cost = meal_cost + (tip_percent + tax_percent)*meal_cost/100.0;
	
	println!("The total meal cost is {} dollars.", total_cost.round());
}
