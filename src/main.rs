use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("deviner le nombre");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("le n° secret est le {}", secret_number);
    println!("entre un entier");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    let guess: u32 = guess.trim().parse().expect("doit être un entier !!");

    println!("tu as rentré le {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("trop petit"),
        Ordering::Greater => println!("trop grand"),
        Ordering::Equal => println!("Gagné!!!"),
    }
}
