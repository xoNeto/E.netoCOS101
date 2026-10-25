use std::io::Write;
fn main() {
   let names = [
    "Oluchi Mordi",
    "Adams Aliyu",
    "Shaniya Bolade",
    "Adekunle Gold",
    "Blanca Edemoh",
];
let matric_numbers = [
"ACC10211111",
"ECO10110101",
"CSC10328828",
"EEE11020202",
"MEE10202001",
];
let departments = ["Accounting", "Economics", "Computer", "Electrical", "Mechanical",];
let levels = [ 300, 100, 200, 200, 100];

    let mut file = std::fs::File::create("PAU-SMIS.txt").expect("create failed");


    println!("{:<17} | {:<15} | {:<15} | {:<5}",
        "Student Name", "Matric. Number", "Department", "Level");

    writeln!(file, "{:<17} | {:<15} | {:<15} | {:<5}",
        "Student Name", "Matric. Number", "Department", "Level")
        .expect("Write failed");

    println!("----------------------------------------------------------------");

    writeln!(file, "----------------------------------------------------------------")
        .expect("Write failed");

    for index in 0..5 {
        println!("{:<18} | {:<15} | {:<15} | {:<5}",
            names[index],
            matric_numbers[index],
            departments[index],
            levels[index]);

        writeln!(file, "{:<18} | {:<15} | {:<15} | {:<5}",
            names[index],
            matric_numbers[index],
            departments[index],
            levels[index])
            .expect("Write failed");
    }

    println!("\nStudent records saved successfully.");
}

