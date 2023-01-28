use rand::distributions::Uniform;
use rand::prelude::Distribution;

pub fn generate_password(length: i32, uc: bool, lc: bool, num: bool, sym: bool) -> (String, f64) {
    if !uc && !lc && !num && !sym {
        return ("".to_owned(), 0.0);
    }

    let low_case = "abcdefghijklmnopqrstuvxyz";
    let up_case = "ABCDEFGHIJKLMNOPQRSTUVXYZ";
    let numbers = "0123456789";
    let symbols = "!£$%&/()=?^*°:;.,";

    let mut all = "".to_owned();

    if uc {
        all = format!("{}{}", all, up_case);
    }

    if lc {
        all = format!("{}{}", all, low_case);
    }

    if num {
        all = format!("{}{}", all, numbers);
    }

    if sym {
        all = format!("{}{}", all, symbols);
    }

    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..all.len() - 2);
    let mut password = "".to_owned();
    for _n in 1..length + 1 {
        let throw = die.sample(&mut rng);
        let mut char = all.chars();
        let char = char.nth(throw).unwrap();
        password = format!("{}{}", password, char);
    }
    return (password, calculate_entropy(length, all.len().into()));
}

pub fn generate_mnemonic_password(length: i32) -> (String, f64) {
    let consonants_lc = "bcdfghjklmnpqrstuvxyz";
    let vowels_lc = "aeiou";

    let mut password = "".to_owned();

    for i in 1..length + 1 {
        if i % 2 == 0 {
            let mut rng = rand::thread_rng();
            let die = Uniform::from(0..vowels_lc.len() - 1);
            let throw = die.sample(&mut rng);
            let mut char = vowels_lc.chars();
            let mut char = char.nth(throw).unwrap();
            if throw % 2 == 1 {
                char = char.to_ascii_uppercase();
            }

            password = format!("{}{}", password, char);
        } else {
            let mut rng = rand::thread_rng();
            let die = Uniform::from(0..consonants_lc.len() - 1);
            let throw = die.sample(&mut rng);
            let mut char = consonants_lc.chars();
            let mut char = char.nth(throw).unwrap();
            if throw % 2 == 1 {
                char = char.to_ascii_uppercase();
            }
            password = format!("{}{}", password, char);
        }
    }
    return (
        password,
        calculate_entropy(length, consonants_lc.len() * 2 + vowels_lc.len() * 2),
    );
}

pub fn calculate_entropy(password_length: i32, charset_length: usize) -> f64 {
    let charset_length: f64 = charset_length as f64;
    let password_length: f64 = password_length.into();
    return password_length * charset_length.log2() / 2.0_f64.log2();
}
