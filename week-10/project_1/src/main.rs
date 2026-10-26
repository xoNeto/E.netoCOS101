struct Laptop {
    brand: String,
    quantity: u32,
    price: f64,
}
impl Laptop {
    fn calculate_total_cost(&self, number_purchased: u32) -> f64 {
    self.price * number_purchased as f64
}
}
fn main() {
    

let hp = Laptop {
    brand: String::from("HP"),
    quantity: 10,
    price: 650000.0,
};
let ibm = Laptop {
    brand: String::from("IBM"),
    quantity: 6,
    price: 755000.0,
};

let toshiba = Laptop {
        brand: String::from("Toshiba"),
        quantity: 10,
        price: 550000.0,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        quantity: 4,
        price: 850000.0,
    };
let hp_cost = hp.calculate_total_cost(3);
    let ibm_cost = ibm.calculate_total_cost(3);
    let toshiba_cost = toshiba.calculate_total_cost(3);
    let dell_cost = dell.calculate_total_cost(3);

    println!("ALABA ELECTRONICS");
    println!("----------------------");

    println!("Brand: {}", hp.brand);
    println!("Quantity available: {}", hp.quantity);
    println!("Price per laptop: ₦{}", hp.price);
    println!("Cost of 3 laptops: ₦{}\n", hp_cost);

    println!("Brand: {}", ibm.brand);
    println!("Quantity available: {}", ibm.quantity);
    println!("Price per laptop: ₦{}", ibm.price);
    println!("Cost of 3 laptops: ₦{}\n", ibm_cost);

    println!("Brand: {}", toshiba.brand);
    println!("Quantity available: {}", toshiba.quantity);
    println!("Price per laptop: ₦{}", toshiba.price);
    println!("Cost of 3 laptops: ₦{}\n", toshiba_cost);

    println!("Brand: {}", dell.brand);
    println!("Quantity available: {}", dell.quantity);
    println!("Price per laptop: ₦{}", dell.price);
    println!("Cost of 3 laptops: ₦{}\n", dell_cost);

    let total_cost = hp_cost + ibm_cost + toshiba_cost + dell_cost;

    println!("==============================");
    println!("Total cost of the 12 laptops chosen: ₦{}", total_cost);
}

