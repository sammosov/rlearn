// Tuples can be used as dunction arguments and as return values.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 'ket' can be used ti bind the members od a tuple to variables
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

// The following struct is for the activity 
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // A tuple with a bunch of different types.
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    // Values can be extracted from the tuple using tuple indesing
    println!("Long tuple first values: {}", long_tuple.0);
    println!("Long tuple second values: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("Tuple if tuples: {:?}", tuple_of_tuples);

    // Bit long tuples (more than 12 elements) cannot be printed.
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7);//, 8, 9, 10, 11, 12, 13);
    // println!("Too Long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair= (1, true);
    println!("Pair is {:?}", pair); 
    println!("The reversed pair is {:?}", reverse(pair));

    // To create one element tuplesm the comma is required to tell them apart 
    // from a literal surrounded by paretntheses
    println!("One element tuple: {:?}", (5u32,));
    println!("Just and integer: {:?}", (5u32));

    // Tuples can be sestructed to create bindings.
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple; 
    println!("[:?], {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Mtrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);  
}
