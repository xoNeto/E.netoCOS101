use std::io::Write;
fn main() {
  let announce = "Categories of drinks availabe\n";
  let category_1 = "Lager : 33 export, desparados, Goldberg, Golder, Heineken, Star.\n";
  let category_2 = "Stout : Legend, Turbo king, Williams\n";
  let category_3 = "Non-alchoholic : Maltina, Amstel malta, Malta Gold, Fayrouz\n";
  let company = "Nigerian Breweries Plc\n";
  let mut file = std::fs::File::create("Nigerian breweries Plc categories of drinks.txt")
  .expect("create failed");
  file.write_all(company.as_bytes()).expect("write failed");
  file.write_all(announce.as_bytes()).expect("write failed");
  file.write_all(category_1.as_bytes()).expect("write failed");
  file.write_all(category_2.as_bytes()).expect("write failed");
  file.write_all(category_3.as_bytes()).expect("write failed");
  println!("\nData written in file");
}