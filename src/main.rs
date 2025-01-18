mod practice;


fn main() {
    // practice word
    let text = "Testt très long? je le suis ici maintenant ainsi".to_string();
    practice::words::words(&text);
    practice::guessing_game::guessing_game();

}