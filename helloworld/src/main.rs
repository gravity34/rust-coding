
// /* fn main() {
//     println!("Hello, world, its my first program in RUST");

//     let x=5;
//     let y=2.3;
//     let condition=true;
//     let z='c';

//     println!("{},{},{},{}",x,y,condition,z);
//     println!("The sum of x and y is: {}",(x+y as i32));
//     println!("The sum of x and y is: {}",(x as f32+y));

//     //Tuple:collection of hetrogenous elements of any data type and mutable
//     let mixdata=(x,y,condition,z);
//     //Array:collection of homogenous elements of any data type and mutable
//     let samedata=[1,2,5,13,6,23];
//     println!("{:?}",mixdata);
//     println!("{:?}",mixdata.0);
//     println!("{:?}",samedata);
//     println!("{}",samedata[0]);

// } */


use std::io;
use std::cmp::Ordering;
use rand::Rng;

// my git me
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

