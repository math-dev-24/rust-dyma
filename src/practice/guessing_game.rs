use std::io;
use rand::Rng;

pub fn guessing_game() {
    let maximum: u32 = 1000;
    let mut guesses = vec![];
    let random_number = rand::thread_rng().gen_range(1..=maximum);

    println!("Devinez le nombre entre 1 et {}", maximum);

    loop {
        let mut guess = String::new();
        println!("Enter un nombre :");

        if !guesses.is_empty() {
            println!("Tentative précédente : {}", guesses[guesses.len() - 1]);
        }

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                guesses.push(num);
                num
            },
            Err(_) => {
                println!("Veuillez entrer un nombre !");
                continue;
            },
        };
        if guess > random_number {
            println!("Plus bas !");
        }else if guess < random_number {
            println!("Plus haut !");
        }else if guess == random_number {
            println!("Vous avez gagné en {} round", guesses.len());
            break;
        }
    }
}
