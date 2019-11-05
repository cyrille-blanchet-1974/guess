use std::io;
use rand::Rng;
use std::cmp::Ordering;

//Supported languages
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
            res.push_str("Gengo o sentaku shite kudasai (言語を選択してください): En = > Eigo (英語) fr = > Furansugo (フランス語) jp => Nihon no (日本の)\n");
        }
    }
    res
}

//check if user guessed well
fn verify(secret : &u32, guess : &u32,lang : &Lang) -> bool
{
   let mut res = false;
   match guess.cmp(secret) {
        Ordering::Less => 
        {
            let m = get_message(&lang, Mess::TooSmall);
            println!("{}",m);
        },
        Ordering::Greater => 
        {
            let m = get_message(&lang, Mess::TooBig);
            println!("{}",m);
        },        
        Ordering::Equal => 
        {
            let m = get_message(&lang, Mess::Win);
            println!("{}",m);
            res=true;
        },
    }    
    res
}

//ask a question and read user'answer
fn read(lang : &Lang,mess:Mess) -> String
{
    let m = get_message(&lang, mess);
    println!("{}",m);
    let mut res = String::new();
    io::stdin().read_line(&mut res)
        .expect("Failed to read line");
    let res = res.trim();
    res.to_string()
}

//ask the user and read his guess
fn read_guess(lang : &Lang) -> u32{
    /*let m = get_message(&lang, Mess::YourAnswer);
    println!("{}",m);
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess= guess.trim();
    */
    let guess = read(&lang,Mess::YourAnswer);
    let u32guess: u32 = guess.parse().unwrap();
    u32guess
}

//ask user what language he prefer until he type something we know
fn ask_language() -> Lang
{
    let lang = read(&Lang::English,Mess::Lang);
    let lang = string_to_lang(lang);   
    match lang
    {
        None => ask_language(),
        Some(a) => a,
    }
}


//main entry
fn main() {
    //langue par défaut
    //let mut lang = string_to_lang("fr");
    
    let lang = ask_language();

    let m = get_message(&lang, Mess::Welcome);
    println!("{}",m);
    /*
    //surchargée par paramètres si passés (le dernier gagne)
    let params: Vec<String> = std::env::args().skip(1).collect();
    for p in params
    {
        lang = string_to_lang(&p[..]);
    }*/

    let m = get_message(&lang, Mess::Welcome);
    println!("{}",m);

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut tries = 5;
    loop
    {
        let guess = read_guess(&lang);
        let res = verify(&secret_number,&guess,&lang);
        if res
        {
            std::process::exit(0);
        }
        tries = tries -1;
        let m = get_message(&lang, Mess::TriesLeft);
        println!("{} {}",m,tries);
        if tries == 0
        {
            let m = get_message(&lang, Mess::Loose);
            println!("{}",m);
            let m = get_message(&lang, Mess::GoodAnswer);
            println!("{} {}",m,secret_number);
            std::process::exit(0);
        }
    }
}