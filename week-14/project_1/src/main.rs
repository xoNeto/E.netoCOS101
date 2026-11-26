use std::io;
use std::fs;

fn display_table(file_name: &str) {
    match fs::read_to_string(file_name){
        Ok(contents) => {
            println!("\n{}", contents);
        }
        Err(error)=> {
            println!("error reading file: {}", error);
        }
    }
}
fn administrator() { 
println! ("\n--- COMPLETE DATABASE STRUCTURE ---");
display_table("customer_tb.sql");
display_table("staff_tb.sql");
display_table("project_tb.sql");
display_table("dataplan_tb.sql");
}
fn project_manager() {
    display_table("project_tb.sql");
}
fn employee(){
    display_table("staff_tb.sql");
}
fn customer() {
    display_table("customer_tb.sql");
}
fn vendor() {
    display_table("dataplan_tb.sql");
} 
fn main(){
    println!("WELCOME TO THE GLOBACOM DATABASE");
    println!("\nEnter your position:");
    println!("Administrator");
    println!("Project Manager");
    println!("Employee");
    println!("Customer");
    println!("Vendor");
    let mut role = String::new();

    io::stdin()
    .read_line(&mut role) .expect("Failed to read input");
    match role.trim().to_lowercase().as_str() {
        "administrator" => administrator(),
        "project manager" => project_manager(),
        "employee" => employee(),
        "customer" => customer(),
        "vendor" => vendor(),
        _ => println!("Invalid position entered"),
    }
}