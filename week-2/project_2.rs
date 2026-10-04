fn main() {
	let toshiba_total: f64 = 450000.00 * 2.0;
	let mac_total: f64 = 1500000.00 * 1.0;
	let hp_total: f64 = 750000.00 * 3.0;
	let dell_total: f64 = 2850000.00 * 3.0;
	let acer_total: f64 = 250000.00 * 1.0;

	// calculate sum
	let sum = toshiba_total + mac_total + hp_total + dell_total + acer_total;
	println!("Sum of sales record is {}",sum);

		// calculate average
let average = sum / 5.0;
println!("average of sales recrd is {}",average);
}