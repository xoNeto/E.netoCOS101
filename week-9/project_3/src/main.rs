use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    // Read the three CSV files
    let commissioners_data = fs::read_to_string("commissioners.csv")
        .expect("Could not read commissioners.csv");

    let ministries_data = fs::read_to_string("ministries.csv")
        .expect("Could not read ministries.csv");

    let zones_data = fs::read_to_string("zones.csv")
        .expect("Could not read zones.csv");

    // Split each file into individual lines and collect them into vectors
    let commissioners: Vec<&str> = commissioners_data.lines().collect();
    let ministries: Vec<&str> = ministries_data.lines().collect();
    let zones: Vec<&str> = zones_data.lines().collect();

    // Create the new merged CSV file
    let mut new_csv = File::create("merged.csv")
        .expect("Could not create merged.csv");

    // Merge the matching records by their positions
    for i in 0..5 {
        let row = format!(
            "{},{},{}\n",
            commissioners[i],
            ministries[i],
            zones[i]
        );

        new_csv.write_all(row.as_bytes())
            .expect("Could not write to merged.csv");
    }

    println!("The three files have been merged successfully!");
}
