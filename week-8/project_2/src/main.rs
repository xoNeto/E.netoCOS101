use std::io;

fn main() {
    let mut developers: Vec<(String, u32)> = Vec::new();

    println!("How many developers?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let count: usize = input.trim().parse().unwrap();

    for i in 0..count {
        let mut name = String::new();

        println!("Enter developer {}'s name:", i + 1);
        io::stdin().read_line(&mut name).unwrap();

        println!("Enter years of experience:");
        let mut years = String::new();
        io::stdin().read_line(&mut years).unwrap();

        let years: u32 = years.trim().parse().unwrap();
        developers.push((name.trim().to_string(), years));
    }

    let mut highest_name = String::new();
    let mut highest_years = 0;

    for (name, years) in developers {
        if years > highest_years || highest_name.is_empty() {
            highest_years = years;
            highest_name = name;
        }
    }

    println!("Most experienced developer: {}", highest_name);
    println!("Years of experience: {}", highest_years);
}
