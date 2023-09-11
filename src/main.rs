use std::io;
use rand::Rng;

fn main(){
    println!("Welcome to RicerollRs");
    print!(" ");
    println!("Rolling Dice.....");
    let mut stop :u32 = 0;
    while stop != 1{

    let landed_face :u32 = rand::thread_rng().gen_range(1..=6);
    match landed_face{
        1 => println!("Tough.....move only one box"),
        6 => println!("Lucky throw...move 6 boxes"),
        other => println!("Player moves {} boxes",other),
    }
    println!("Do you want to roll the dice again...(y or n)");

    let mut cont = String::new();
    io::stdin()
    .read_line(&mut cont)
    .expect("Failed to readline");
    
    if cont.trim() == "n"{
        stop = 1;
        println!("Byee!")
    }else{
        continue;
    }
    }
}