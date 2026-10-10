use std::io;
fn main() {
  println!("===== RESTAURANT MENU =====");
    println!("P. Pounded Yam / Edikaikong Soup - ₦3200");
    println!("F. Fried Rice & Chicken - ₦3000");
    println!("A. Amala & Ewedu Soup - ₦2500");
    println!("E. Eba & Egusi Soup - ₦2000");
    println!("W. White Rice & Stew - ₦2500");
    
    let mut food_choice = String::new();
    println!(" Enter the letter of your food choice: ");
    io::stdin().read_line(&mut food_choice).expect("failed to read input");
    let food = food_choice.trim().to_uppercase();
    let price = if food == "P" {
        3200
    }  else if food == "F" {
        3000
    } else if food == "A" {
        2500
    } else if food == "E" {
        2000
    } else if food == "W" {
        2500
    } else { println!("Not a valid food choice"); 
    return;
};
println!("Price per plate: {}", price);

let mut quantity_input = String::new();

    println!("Enter the quantity:");

    io::stdin()
        .read_line(&mut quantity_input)
        .expect("Failed to read input");

    let quantity: u32 = match quantity_input.trim().parse() {
        Ok(value) if value > 0 => value,
        _ => {
            println!("Please enter a valid quantity greater than zero.");
            return;
        }

    };
    let total = price * quantity;
    println!("Quantity: {}", quantity);
    println!("total charge: {}", total );

    let discount = if total >= 10_000 {
        total * 5 / 100
    } else { 0 };
    let final_total = total - discount;

    println!("Discount: {}", discount);
    println!("Total amount to be paid: {}", final_total );
}