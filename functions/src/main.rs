/* fn main() {
    println!("Hello, world!");
    another_function();
}
fn another_function() {
    println!("Another function.");
} */


/* fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
} */

// Statements perform some action and do not return a value
//Expressions evaluate to a resulting value
//let keyword is a statement
//Function definitions are also statements;
//The block that we use to create new scopes, {}, is an expression,
//Rust is an expression-based language

/* fn main() {
    let x = (let y = 6);        //Statements do not return values. so error will occur
} */


/* fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
} */




// function returning value
/* fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
} */


fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1      //adding ; will give error it will convert expression into statement which return nothing
}



