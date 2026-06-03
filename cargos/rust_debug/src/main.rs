//This structure cannot be printed either woth 'fmt::Display' ot 
// with 'fmt::Denug'.

//struct UnPrintable(i32);

//The 'derive' attribute automatically creates the implementation 
// required to make this 'struct' printable woth 'fmt::Debug'.

//#[derive(Debug)]
//struct DebugPrinyable(i32);

/*#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    //Printing with '{:2}' is similar wiy {} 
    println!("{:2} month in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name", "Slater", "Christian", actor="actor`s");
    println!("Now {:?} will print", Structure(3));

    //the problem with 'derive' is there is no contol ober how 
    // the  results look. What id I want this ti just sjow a '7'?
    println!("Now {:?} will print!", Deep(Structure(7)));
}
*/


#[derive(Debug)]
struct Person<'a>{
    name: &'a str, 
    age: u8
}

fn main(){
    let name = "Peter"; 
    let age = 27; 
    let peter = Person {name, age};

    //Pretty print
    println!("{:#?}", peter);

}