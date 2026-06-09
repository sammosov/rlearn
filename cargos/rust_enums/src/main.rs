// Crete the enum to classify a web event. Note how both 
// names and type information together specify the variat: 
// 'Pageload != PageUnload' and 'keyPress(char) !=Paste(String)'.
// each is different and independent.

enum WebEvent{
    // An 'enum' variant may eother the 'unit-like', 
    PageLoad,
    PageUnload,
    // like tuple structs, 
    KeyPress(char), 
    Paste(String), 
    // or c-like structures.
    Click {x: i64, y: i64}
}

// a function which takes a 'WebEvent' enum aa an argument and 
// return warning. 

fn inspect(event: WebEvent){
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"), 
        // Destructure 'c' from inside the 'enum' variant. 
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure 'Click' into 'x' and 'y'.
        WebEvent::Click {x, y}  => {
            println!("clicked at x={}, y={}.", x, y); 
        },
    }
} 


fn main() {
    let pressed = WebEvent::KeyPress('x');
    // 'tp_owned()' creates an owned 'String' from a string slice 
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80};
    let load    = WebEvent::PageLoad; 
    let unload  = WebEvent::PageUnload; 

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload); 
}
