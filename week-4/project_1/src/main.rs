use std::io;
fn main() {
   println!("Enter a:");
   let mut input_a = String::new();
   io::stdin().read_line(&mut input_a).expect("Failed to read input");
   let a:f32 = input_a.trim().parse().expect("Not a valid number");

println!("Enter b:");
   let mut input_b = String::new();
   io::stdin().read_line(&mut input_b).expect("Failed to read input");
   let b:f32 = input_b.trim().parse().expect("Not a valid number");

   println!("Enter c:");
   let mut input_c = String::new();
   io::stdin().read_line(&mut input_c).expect("Failed to read input");
   let c:f32 = input_c.trim().parse().expect("Not a valid number");

   let discriminant:f32 = (b * b) - (4.0 * a * c);
   println!("Discriminant = {}", discriminant );
   
   if discriminant > 0.0 {
    let root1 = ( -b + discriminant.sqrt() ) / (2.0 *a);
    let root2 = ( -b - discriminant.sqrt() ) / (2.0 * a);
   println!("Root 1 = {}",root1 );
   println!("Root 2 = {}",root2 );
   } 

   else if discriminant == 0.0 {
    let root = -b / (2.0 * a);
    println!("There is one real root.");
    println!("Root = {}", root);
   }    

   
    else if discriminant < 0.0 {
        println!("There are no real roots");
    }
   
}
