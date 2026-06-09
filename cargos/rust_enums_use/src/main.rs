// An attribute to hide warning for unused code.
#![allow(dead_code)]

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student, 
    Teacher, 
}

fn main() {
    // Explicitly 'use' each name so they are available 
    // without scoping. 
    use Stage::{Beginner, Advanced};
    // Automatically 'use' each name inside 'Role". 
    use Role::*;

    // Equivalent ti 'Stages::beginner'.
    let stage = Beginner;
    // Equivalent to 'Roles::Student'.
    let role = Student; 

    match stage {
        // Note the lack of scoping because of the explicit 'use' above
        Beginner => println!("Beginners are starting their learing journey!"),
        Advanced => println!("Advanced learner are msatering their subjects..."), 
    }

    match role {
        // Note again the lack of scoping. 
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }
}

