pub use super::messages::*;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

//our game
pub struct Game {
    lang: Lang,
    nb_tries: u32,
    secret_number: u32,
}

//its private functions
impl Game {
    pub fn new(lang: Lang, nb_tries: u32, secret_number: u32) -> Game {
        Game {
            lang,
            nb_tries,
            secret_number,
        }
    }

    //call global get_message with local lang
    fn get_message(&self, idx: Mess) -> String {
        Mess::get_message(&self.lang, idx)
    }

    //main part of the game
    pub fn run(&mut self) -> bool {
        loop {
            //show how many tries left
            let m = self.get_message(Mess::TriesLeft);
            println!("{} {}", m, self.nb_tries);
            let guess = self.read_guess();
            let res = self.verify(&guess);
            if res {
                //win => message send in verify
                return true;
            }
            self.nb_tries -= 1;
            //if no tries left then player loosed
            if self.nb_tries == 0 {
                let m = self.get_message(Mess::Loose);
                println!("{}", m);
                //we tell him what was the correct answer
                let m = self.get_message(Mess::GoodAnswer);
                println!("{} {}", m, self.secret_number);
                return false;
            }
        }
    }

    //ask the user and read his guess
    fn read_guess(&mut self) -> u32 {
        let m = self.get_message(Mess::YourAnswer);
        let guess = Game::read(&m);
        let u32guess: u32 = match guess.parse() {
            Err(_e) => {
                let m = self.get_message(Mess::UnsignedIntegerRequired);
                println!("{}", m);
                self.read_guess()
            }
            Ok(v) => v,
        };
        u32guess
    }

    //check if user guessed well
    fn verify(&self, guess: &u32) -> bool {
        let mut res = false; //probably loose
        let m = match guess.cmp(&self.secret_number) {
            Ordering::Less => Mess::TooSmall,
            Ordering::Greater => Mess::TooBig,
            Ordering::Equal => {
                res = true; //win!
                Mess::Win
            }
        };
        let m = self.get_message(m);
        println!("{}", &m);
        res
    }

    //ask a question and read user'answer
    pub fn read(mess: &str) -> String {
        println!("{}", mess);
        let mut res = String::new();
        io::stdin()
            .read_line(&mut res)
            .expect("Failed to read line");
        let res = res.trim();
        res.to_string()
    }

    //ask user what language he prefer until he type something we know
    pub fn ask_language() -> Lang {
        let m = Mess::get_message(&Lang::English, Mess::Lang);
        let lang = Game::read(&m);
        let lang = Lang::string_to_lang(lang);
        match lang {
            None => Game::ask_language(),
            Some(a) => a,
        }
    }

    //ask userhow many tries the user wants
    pub fn ask_tries(lang: &Lang) -> u32 {
        let m = Mess::get_message(&lang, Mess::NbTries);
        let tries = Game::read(&m);
        let tries: u32 = match tries.parse() {
            Err(_e) => {
                let m = Mess::get_message(&lang, Mess::UnsignedIntegerRequired);
                println!("{}", m);
                Game::ask_tries(&lang)
            }
            Ok(v) => v,
        };
        tries
    }

    pub fn generate_new_sercret_num() -> u32 {
        rand::thread_rng().gen_range(1, 101)
    }
}
