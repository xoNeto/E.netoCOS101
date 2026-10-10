use std::io;
fn main() {
    let mut experience_input = String::new();
    println!("Are you experienced? (yes/no):");
    io::stdin()
    .read_line(&mut experience_input).expect("failed to read input");
let experienced = if experience_input.trim().eq_ignore_ascii_case("yes"){
    true 
} else if experience_input.trim().eq_ignore_ascii_case("no"){
    false
}else { println!("Invalid input. please enter yes or no."); return; };

let mut age_input = String::new();
println!(" Enter your age");
io::stdin().read_line(&mut age_input).expect("failed to read input");
let age: u32 = match age_input.trim().parse() {
Ok(value) => value,
Err(_) => {
    println!("Enter valid age.");
    return;
}
};
let incentive = if !experienced { 100_000 } 
else if age >= 40 { 1_560_000 }
else if age >= 30 { 1_480_000 }
else if age < 28 { 1_300_00 }
else { println!("incentive not specified for this age");
return; };

println!(" Annual incentive: {}", incentive);
}

