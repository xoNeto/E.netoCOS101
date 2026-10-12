 use std::io;

fn main() {
    let levels = vec![
        "APS 1-2", "APS 3-5", "APS 5-8",
        "EL2 8-10", "EL2 10-13", "SES"
    ];

    let professions = vec![
        "Office Admin", "Academic", "Lawyer", "Teacher" 
    ];

    println!("Choose profession: 0=Office Admin, 1=Academic, 2=Lawyer, 3=Teacher");
    let mut p = String::new();
    io::stdin().read_line(&mut p).unwrap();

    println!("Enter level (0-5):");
    let mut l = String::new();
    io::stdin().read_line(&mut l).unwrap();

    let p: usize = p.trim().parse().unwrap();
    let l: usize = l.trim().parse().unwrap();

    if p < professions.len() && l < levels.len() {
        let position = match (p, l) {
            (0, 0) => "Intern",
            (0, 1) => "Administrator",
            (0, 2) => "Senior Administrator",
            (0, 3) => "Office Manager",
            (0, 4) => "Director",
            (0, 5) => "CEO",
            (1, 1) => "Research Assistant",
            (1, 2) => "PhD Candidate",
            (1, 3) => "Post-Doc Researcher",
            (1, 4) => "Senior Lecturer",
            (1, 5) => "Dean",
            (2, 0) => "Paralegal",
            (2, 1) => "Junior Associate",
            (2, 2) => "Associate",
            (2, 3) => "Senior Associate 1-2",
            (2, 4) => "Senior Associate 3-4",
            (2, 5) => "Partner",
            (3, 0) => "Placement",
            (3, 1) => "Classroom Teacher",
            (3, 2) => "Senior Teacher",
            (3, 3) => "Leading Teacher",
            (3, 4) => "Deputy Principal",
            (3, 5) => "Principal",
            _ => "No position listed",
        };

        println!("Level: {}", levels[l]);
        println!("Position: {}", position);
    } else {
        println!("Invalid choice!");
    }
}

