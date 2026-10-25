use std::io::Write;
fn main() {
   let commissioners = ["Aigbogun Alamba Daudu",
"Murtala Afeez Bendu",
"Okorocha Calistus Ogbona",
"Adewale Jimoh Akanbi",
"Osazuwa Faith Etieye",];
let ministries = ["Internal Affairs",
"Justice",
"Defense",
"power & steel",
"Petroleum",];
let zone = ["South West",
"North East",
"South South",
"South West",
"South East",];
let mut file = std::fs::File::create("The EFCC Merge.txt").expect("create failed");
println!("{:<25} | {:<15} | {:<15}",
        "Commisioner", "Ministry", "Zone");
for index in 0..5 {
    println!(
        "{:<25} | {:<15} | {:<15}",
        commissioners[index],
        ministries[index],
        zone[index]
    );

    writeln!(
        file,
        "{:<25} | {:<15} | {:<15}",
        commissioners[index],
        ministries[index],
        zone[index]
    )
    .expect("Write failed");
}
}