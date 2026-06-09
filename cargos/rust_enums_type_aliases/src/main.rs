enum VeryVernoseEnumOfThingsToDoWithNumbers {
    Add, 
    Substract
}

// Creates a type alias 
type Operations = VeryVernoseEnumOfThingsToDoWithNumbers; 


// The nmost common place you will see this is in impl blocks using the 'self' alias
impl VeryVernoseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y, 
            Self::Substract => x - y,  
        }
    }
}




fn main() {
    // We can refer  to each vatiant via its alias, not its long and inconvinent names.
    let _x = Operations::Add;
}

