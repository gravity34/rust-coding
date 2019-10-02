/* fn main() {
   
    /* let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);  */
    //----------------------------------

  /*   let x = 5;      //shadowing allow reusing same variable
    let x = x + 1;      //shadowing is diff then mut
    let x = x * 2;
    println!("The value of x is: {}", x); 
    let spaces = "   ";
    let spaces = spaces.len();*/
    //------------------------
/* 
    let _x = 2.0; // f64   //_ used for unused variables

    let _y: f32 = 3.0; // f32

    //let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;  // destructuring, because it breaks the single tuple into three parts
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z); 
 */
} */


//------------ functions-----------------


fn main() {
    another_function(5, 6);

    let y = {
        let x = 23;
        x + 1
    };

    println!("The value of y is: {}", y);
    
    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}






fn plus_one(x: i32) -> i32 {
    x + 1
}


fn five() -> i32 {
    5
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
