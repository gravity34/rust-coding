fn main() {
    
     let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }



   /*  let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    } */

     //Using if in a let Statement

/* let condition = true;
    let number = if condition {
        35
    } else {
        6
    };

    println!("The value of number is: {}", number); */
//--------------------- Loop-----------------
let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);


//------------------while----------
 let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

//Looping Through a Collection 
let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

//Looping Through a Collection with FOR LOOP
let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }


for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    
}
