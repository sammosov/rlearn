/* Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
Floating point: f32, f64
char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
bool either true or false
The unit type (), whose only possible value is an empty tuple: ()
Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.
*/

fn main() {
    // Variables can be type annotated.
    let logical: bool = true; 
    
    let a_float: f64 = 1.0;   // Regular annotaion
    let an_integer   = 5i32;  // Suffix annotation

    // Or the default will be used.
    let default_float = 3.0; // 'f64'
    let default_integer = 7; // 'i32' 

    // A type can also be inferred from a context.
    let mut inferred_type = 12; // type i64 is inferrred from another line; 
    inferred_type = 4294967296i64;

    // A mutable variable`s value can be changed.
    let mut mutable = 12; // Mutable 'i32'
    mutable = 22;

    // Error! The type of variable can`t be changed.
    //mutable = true; 
    
    //Varibales can be overwritten withi shadowing
    let muyable = true;

    /* ------------------ Compound types - Arra and Tuple ------------------------*/

    // Array signature consists of Type T and length as [T; length]
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // Tuple is a collection 'kortezh' of values of different types 
    // and its constructed using parentheses ().

    let my_tuple = (5u32, 1u8, true, -5.04f32);




}
