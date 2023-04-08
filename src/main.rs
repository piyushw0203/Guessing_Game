#![allow(non_snake_case)]
use std::io;
use rand::Rng;

use std::time::{Instant,Duration};
fn main() 
{
    println!("GUESS THE NUMBER GAME!!");
    let secret_number=rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is {}",secret_number);
    let time_limit=Duration::from_secs(40);
    let start_time=Instant::now();
    loop 
    {
        println!("Guess the Number");
        println!("You have 40 seconds to make your guess");
       // let start_time=Instant::now();
        
        let mut guess=String::new();

        

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32=match guess.trim().parse()
        {
            Ok(num)=>num,
            Err(_)=>continue,

        };
    if start_time.elapsed()>=time_limit
        {
            println!("Time's up!");
            break;
        }  

    println!("You guessed {guess}");
    
    {
        
        if guess == secret_number
        {   
            println!("You Win!!");
            break;
        }
        else if guess<secret_number
        {
            println!("Too Small");
        }  
        else
        {
            println!("Too Big");
        } 
          
    }
    
    }
    
}