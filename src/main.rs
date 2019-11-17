mod game;
mod lang;
mod messages;

use game::*;

//start a new game
fn new_game(l: &Lang, tries: &u32) -> bool {
    let secret_number = Game::generate_new_sercret_num();
    let mut game = Game::new(*l, *tries, secret_number);
    game.run()
}

//main entry
fn main() {
    //first ask the lang
    let lang = Game::ask_language();
    //second ask then number of tries
    let mut tries = Game::ask_tries(&lang);

    let m = Mess::get_message(&lang, Mess::Welcome);
    println!("{}", m);

    let mut nb_win = 0;

    // loop until users loose increasing the difficulty each turn (one tries less)
    loop {
        let res = new_game(&lang, &tries);
        if !res {
            //show number of victories
            let m = Mess::get_message(&lang, Mess::NbWins);
            println!("{} {}", m, nb_win);

            std::process::exit(0);
        } else {
            nb_win += 1;
            tries -= 1;
        }
    }
}
