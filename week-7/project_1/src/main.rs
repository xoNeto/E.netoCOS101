use std::io;

fn main() {
  println!("===== SHAPE CALCULATOR =====");
    println!("1. Trapezium - Area");
    println!("2. Rhombus - Area");
    println!("3. Parallelogram - Area");
    println!("4. Cube - Surface Area");
    println!("5. Cylinder - Volume");
    println!("Enter your choice (1-5):");
let mut choice_input = String::new();

    io::stdin()
        .read_line(&mut choice_input)
        .expect("Failed to read input");

    let choice: u32 = match choice_input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid input. Please enter a number from 1 to 5.");
            return;
        }
    };

    if choice < 1 || choice > 5 {
        println!("Please choose a number from 1 to 5.");
        return;
    }

    if choice == 1 {
        let mut base1_input = String::new();
        let mut base2_input = String::new();
        let mut height_input = String::new();

        println!("Enter the first base:");
        io::stdin().read_line(&mut base1_input).expect("Failed to read input");

        println!("Enter the second base:");
        io::stdin().read_line(&mut base2_input).expect("Failed to read input");

        println!("Enter the height:");
        io::stdin().read_line(&mut height_input).expect("Failed to read input");

        let base1: f64 = base1_input.trim().parse().expect("Enter a valid number");
        let base2: f64 = base2_input.trim().parse().expect("Enter a valid number");
        let height: f64 = height_input.trim().parse().expect("Enter a valid number");

        let area = trapezium_area(base1, base2, height);

        println!("Area of trapezium: {}", area);
    }
    else if choice == 2 {
        let mut diagonal1_input = String::new();
        let mut diagonal2_input = String::new();

        println!("Enter the first diagonal:");
        io::stdin()
            .read_line(&mut diagonal1_input)
            .expect("Failed to read input");

        println!("Enter the second diagonal:");
        io::stdin()
            .read_line(&mut diagonal2_input)
            .expect("Failed to read input");

        let diagonal1: f64 = diagonal1_input.trim().parse()
            .expect("Enter a valid number");
        let diagonal2: f64 = diagonal2_input.trim().parse()
            .expect("Enter a valid number");

        let area = rhombus_area(diagonal1, diagonal2);

        println!("Area of rhombus: {}", area);
    }
    else if choice == 3 {
        let mut base_input = String::new();
        let mut height_input = String::new();

        println!("Enter the base:");
        io::stdin()
            .read_line(&mut base_input)
            .expect("Failed to read input");

        println!("Enter the height:");
        io::stdin()
            .read_line(&mut height_input)
            .expect("Failed to read input");

        let base: f64 = base_input.trim().parse()
            .expect("Enter a valid number");
        let height: f64 = height_input.trim().parse()
            .expect("Enter a valid number");

        let area = parralelogram_area(base, height);

        println!("Area of parallelogram: {}", area);
    }
    else if choice == 4 {
        let mut side_input = String::new();

        println!("Enter the side length of the cube:");
        io::stdin()
            .read_line(&mut side_input)
            .expect("Failed to read input");

        let side: f64 = side_input.trim().parse()
            .expect("Enter a valid number");

        let area = cube_surface_area(side);

        println!("Surface area of cube: {}", area);
    }
    else if choice == 5 {
    println!("Enter the radius of the cylinder:");
    let mut radius = String::new();
    std::io::stdin().read_line(&mut radius).unwrap();
    let radius: f64 = radius.trim().parse().unwrap();

    println!("Enter the height of the cylinder:");
    let mut height = String::new();
    std::io::stdin().read_line(&mut height).unwrap();
    let height: f64 = height.trim().parse().unwrap();

    let volume = 3.142 * radius * radius * height;

    println!("The volume of the cylinder is: {}", volume);
}
}
fn trapezium_area(base1: f64, base2: f64, height: f64) -> f64{
    height / 2.0 * (base1 + base2)
}
fn rhombus_area(daigonal1: f64, daigonal2: f64,) -> f64 {daigonal1 * daigonal2 / 2.0}
fn parralelogram_area(base: f64, height: f64) -> f64
{base * height}
fn cube_surface_area(side: f64) -> f64 {6.0 * side * side}
fn cylinder_volume(radius: f64, height: f64) -> f64{3.142 * radius * radius * height}