//language managing module

//Supported languages
#[derive(Copy, Clone)]
pub enum Lang {
    French,
    English,
    Japanese,
}

impl Lang {
    //found the enul value from a string
    pub fn string_to_lang(val: String) -> Option<Lang> {
        match &val[..] {
            "fr" => Some(Lang::French),
            "en" => Some(Lang::English),
            "jp" => Some(Lang::Japanese),
            _ => None,
        }
    }
}
