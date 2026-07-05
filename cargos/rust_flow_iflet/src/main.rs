// For som use cases, when matching enum, march is awkward. For exmple: 

/*
// Make 'optional' of type 'Option<i32>'
let optional = Some(7);

match optional {
    Some(i) => orintln!("This is a really long string and '{:?}'", i),
    _ -> {}, //Required because 'march' is exhaustive. Doesn`t it seem  like wasted space?
};

*/

// 'if let' is cleaner dir this use xase and in addition allows various faulure options to be specified:

/* 
fn main() {
    // All have type 'Option<i32>
    let number = Some(7); 
    let letter: Option<i32> = None; 
    let emotion: Option<i32> = None; 

    // The 'if let' construct reads: "if 'let' destructures 'number' into"
    // 'Some(i)', evaluate the block ('{}').
    if let Some(i) = number {
        println!("Matched {:?}", i);
    } else {
        // Destructure failed. Change to the faulrure case. 
        println!("Didn`t match a number. Let`s go with letter!");
    }

    // provide and altered failing combination. 
    let i_like_letters = false; 

    if let Some(i) = emotion {
        println!("Matched {:?}", i);
        // destructure failed. Evaluate an 'else if' condition to see if the 
        // alternate failrure branch should be taken:
    } else if i_like_letters {
        println!("Didn`t match a numbber. Let`s go with letter!");
    } else {
        // The construction evaluated false. This branch os the defaults: 
        println!("I don`t like letters. Let`s go with emotion :) !!");
    }
}
*/

// In the same way 'if let' can be used to match any enum value: 

// Our example enum 
/*
enum Foo {
    Bar, 
    Baz, 
    Qux(u32)
}

fn main() {
    // create example wariables 
    let a = Foo::Bar; 
    let b = Foo::Baz; 
    let c = Foo::Qux(100); 

    // Variable a matches Foo:Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // Variable b doesnot match Foo:Bar
    // So this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also work with 'if let'
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
}
*/

enum Foo {Bar}

fn main() {
    let a = Foo::Bar; 

    // Variable matches Foo::Bar
    if let Foo::Bar = a {  // This cause a compile error. Try to use 'if let' instead
        println!("a is foobar");
    }
}
