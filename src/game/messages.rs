//message managing module

//need langage managing to use messages
mod lang;
//and we make it public so that we can use it in parents
pub use lang::*;

//List of user's messages 
pub enum Mess
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

impl Mess{
    //what to write on display given a message and a language
    pub fn get_message(lang : &Lang, idx : Mess)->String 
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
}