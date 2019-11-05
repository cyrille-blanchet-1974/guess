use std::io;
use rand::Rng;
use std::cmp::Ordering;

//Supported languages
#[derive(Copy, Clone)]
 enum Lang
{
    French,
    English,
    Japanese,
}

//found the enul value from a string
fn string_to_lang(val : String) -> Option<Lang>
{
    match &val[..]
    {
      "fr" => Some(Lang::French),
      "en" => Some(Lang::English),
      "jp" => Some(Lang::Japanese),
      _    => None,
    }
}

//List of user's messages 
enum Mess
{
    Welcome,
    TriesLeft,
    Loose,
    GoodAnswer,
    YourAnswer,
    TooSmall,
    TooBig,
    Win,
    Lang,
    NbTries,
    UnsignedIntegerRequired,
    NbWins,
}

//what to write on display given a message and a language
fn get_message(lang : &Lang, idx : Mess)->String 
{
    let mut res;
    match idx  
    {
        Mess::Welcome=> 
        {
            match lang 
            {
                Lang::English  => res="Guess the number!".to_string(),
                Lang::French   => res="Devinez le nombre!".to_string(),
                Lang::Japanese => res="Sūji o suisoku shite kudasai (数字を推測してください)！".to_string(),
            }
        },
        Mess::TriesLeft=> 
        {
            match lang 
            {
                Lang::English  => res="Tries left ".to_string(),
                Lang::French   => res="Essais restant ".to_string(),
                Lang::Japanese => res="Nokori no tesuto (残りのテスト) ".to_string(),
            }
        },
        Mess::Loose=> 
        {
            match lang 
            {
                Lang::English  => res="No more tries : you Loose".to_string(),
                Lang::French   => res="Plus d'essais: vous avez perdu".to_string(),
                Lang::Japanese => res="Sonohoka no tesuto: Maketa (その他のテスト：負けた) ".to_string(),
            }
        },
        Mess::GoodAnswer=> 
        {
            match lang 
            {
                Lang::English  => res="Correct answer was ".to_string(),
                Lang::French   => res="La bonne réponse était ".to_string(),
                Lang::Japanese => res="Seikai wa (正解は)".to_string(),
            }
        },
        Mess::YourAnswer=> 
        {
            match lang 
            {
                Lang::English  => res="Please input your guess.".to_string(),
                Lang::French   => res="Merci de saisir le nombre de votre choix.".to_string(),
                Lang::Japanese => res="Sentaku shita bangō o nyūryoku shite kudasai (選択した番号を入力してください。).".to_string(),
            }
        },
        Mess::TooSmall=> 
        {
            match lang 
            {
                Lang::English  => res="Too small!".to_string(),
                Lang::French   => res="Le nombre est plus grand!".to_string(),
                Lang::Japanese => res="Kazu ga ōi (数が多い)!".to_string(),
            }
        },
        Mess::TooBig=> 
        {
            match lang 
            {
                Lang::English  => res="Too big!".to_string(),
                Lang::French   => res="Le nombre est plus petit!".to_string(),
                Lang::Japanese => res="Kazu wa motto sukunai (数はもっと少ない)!".to_string(),
            }
        },
        Mess::Win=> 
        {
            match lang 
            {
                Lang::English  => res="Congratulation: You win!".to_string(),
                Lang::French   => res="Bravo: vous avez deviné!".to_string(),
                Lang::Japanese => res="Burabō: Suisoku shita (ブラボー：推測した)!".to_string(),
            }
        },
        Mess::Lang=>
        {
            res="Choose your language: en=> English fr=>French jp=>japanese\n".to_string();
            res.push_str("Choisissez votre langue: en=> Anglais fr=>Français jp=>Japonais\n");
            res.push_str("Gengo o sentaku shite kudasai (言語を選択してください): En = > Eigo (英語) fr = > Furansugo (フランス語) jp => Nihon no (日本の)");
        },
        Mess::NbTries=>
        {
            match lang 
            {
                Lang::English  => res="How many tries do you want to guess?".to_string(),
                Lang::French   => res="En combien d'essais pensez vous trouver ?".to_string(),
                Lang::Japanese => res="Nankai tameshite mitaidesu ka (何回試してみたいですか)?".to_string(),
            }
        },
        Mess::UnsignedIntegerRequired=>
        {
            match lang 
            {
                Lang::English  => res="You must enter a number between 1 and 100!".to_string(),
                Lang::French   => res="Vous devez saisir un nombre entre 1 et 100!".to_string(),
                Lang::Japanese => res="1 〜 100 No sūji o nyūryoku suru hitsuyō ga arimasu (1〜100の数字を入力する必要があります。)".to_string(),
            }
        },
        Mess::NbWins=>
        {
            match lang 
            {
                Lang::English  => res="Number of wins:".to_string(),
                Lang::French   => res="Nombre de victoires:".to_string(),
                Lang::Japanese => res="Kachi-sū (勝ち数):".to_string(),
            }
        }
    }
    res
}

//our game
struct Game{
    lang : Lang,
    nb_tries : u32,
    secret_number : u32
}

//its private functions
impl Game{
    //call global get_message with local lang
    fn get_message(&self, idx : Mess)->String 
    {
        get_message(&self.lang, idx)
    }

    //main part of the game
    fn run(&mut self)-> bool
    {
        loop
        {
            //show how many tries left
            let m = self.get_message(Mess::TriesLeft);
            println!("{} {}",m,self.nb_tries);
            let guess =self.read_guess();
            let res = self.verify(&guess);
            if res
            {
                //win => message send in verify
                return true;
            }
            self.nb_tries -= 1;
            //if no tries left then player loosed
            if self.nb_tries == 0
            {
                let m = self.get_message(Mess::Loose);
                println!("{}",m);
                //we tell him what was the correct answer
                let m = self.get_message(Mess::GoodAnswer);
                println!("{} {}",m,self.secret_number);
                return false;
            }
        }        
    }

    //ask the user and read his guess
    fn read_guess(&mut self) -> u32{
        let m = self.get_message(Mess::YourAnswer);
        let guess = read(&m);
        let u32guess: u32 =
        match guess.parse()
        {
            Err(_e) => {
                let m = self.get_message(Mess::UnsignedIntegerRequired);
                println!("{}",m);
                self.read_guess()
            },
            Ok(v) => v,
        };
        u32guess
    }

    //check if user guessed well
    fn verify(&self, guess : &u32) -> bool
    {
        let mut res = false;//probably loose
        let m = match guess.cmp(&self.secret_number) {
                Ordering::Less => 
                {
                    Mess::TooSmall
                },
                Ordering::Greater => 
                {
                    Mess::TooBig
                },        
                Ordering::Equal => 
                {
                    res=true;//win!
                    Mess::Win
                },
        };
        let m = self.get_message(m);
        println!("{}",&m);
        res
    }
}

//ask a question and read user'answer
fn read(mess :&str) -> String
{
    println!("{}",mess);
    let mut res = String::new();
    io::stdin().read_line(&mut res)
        .expect("Failed to read line");
    let res = res.trim();
    res.to_string()
}


//ask user what language he prefer until he type something we know
fn ask_language() -> Lang
{
    let m = get_message(&Lang::English,Mess::Lang);
    let lang = read(&m);
    let lang = string_to_lang(lang);   
    match lang
    {
        None => ask_language(),
        Some(a) => a,
    }
}

//ask userhow many tries the user wants
fn ask_tries(lang : &Lang) -> u32
{
    let m = get_message(&lang,Mess::NbTries);
    let tries = read(&m);
    let tries: u32 = 
        match tries.parse()
        {
            Err(_e) => {
                let m = get_message(&lang,Mess::UnsignedIntegerRequired);
                println!("{}",m);
                ask_tries(&lang)
            },
            Ok(v) => v,
        };
    tries
}

//start a new game 
fn new_game(l : &Lang, tries :&u32) -> bool
{
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut game = Game{
        lang: *l,
        nb_tries: *tries,
        secret_number : secret_number};
    game.run()
}

//main entry
fn main() {
    //first ask the lang   
    let lang = ask_language();
    //second ask then number of tries
    let mut tries = ask_tries(&lang);

    let m = get_message(&lang, Mess::Welcome);
    println!("{}",m);

    let mut nb_win=0;

    // loop until users loose increasing the difficulty each turn (one tries less)
    loop
    {
        let res = new_game(&lang,&tries);
        if !res
        {
            //show number of victories
            let m = get_message(&lang, Mess::NbWins);
            println!("{} {}",m,nb_win);

            std::process::exit(0);
        }
        else
        {
            nb_win+=1;
            tries-=1;
        }
    }

}