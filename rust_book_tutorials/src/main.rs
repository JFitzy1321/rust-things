#![allow(dead_code)]

fn main() {
    // println!("Hello, cargo! Again...");
    // guessing_game::main();
    // println!("Chapter 3: Variables!");
    // chapter_3::variables();

    // println!("Chapter 3: Shadowing!");
    // chapter_3::shadowing();

    println!("Chapter 3: Array indexing");
    chapter_3::out_of_bounds_array();
}

// // Chapter 2 : Guessing Game
// mod guessing_game {
//     use rand::Rng;
//     use std::cmp::Ordering;
//     use std::io::Write;

//     pub fn main() {
//         println!("Guess the number!");
//         let secret_num: u32 = rand::thread_rng().gen_range(1, 101);
//         println!("The secret number is: {}", secret_num);
//         loop {
//             print("Please input your guess: ");
//             let mut guess = String::new();
//             std::io::stdin()
//                 .read_line(&mut guess)
//                 .expect("Failed to read line");
//             let guess = guess.trim();
//             if ["quit", "exit"].contains(&guess.to_lowercase().as_ref()) {
//                 println!("Quitting game!");
//                 break;
//             }
//             let guess: u32 = match guess.parse() {
//                 Ok(num) => num,
//                 Err(_) => {
//                     println!("Please enter a whole number!");
//                     continue;
//                 }
//             };
//             println!("You guessed {}", guess);
//             match guess.cmp(&secret_num) {
//                 Ordering::Less => println!("Too small!"),
//                 Ordering::Greater => println!("Too big!"),
//                 Ordering::Equal => {
//                     println!("You win!");
//                     break;
//                 }
//             }
//         }
//     }

//     fn print(message: &str) {
//         print!("{}", message);
//         std::io::stdout().flush().unwrap();
//     }
// }

// Common Programming Concepts
mod chapter_3 {
    pub fn variables() {
        let x = 5;
        println!("X is: {}", x);
        let x = 6;
        println!("X is now {} after shadowing!", x);
        const MAX_INT: u32 = 100_000;
    }

    pub fn shadowing() {
        let text = "I am some text";
        println!("{}", text);
        let text = 32_876;
        println!("{}", text);
    }

    pub fn out_of_bounds_array() {
        let numArray = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let index = 20;

        println!("The {} element is: {}", index, numArray[index]);
    }
}
