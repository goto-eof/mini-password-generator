use rand::distributions::Uniform;
use rand::prelude::Distribution;

pub fn generate_password(length: i32) -> String {
    let low_case = "abcdefghijklmnopqrstuvxyz";
    let up_case = "ABCDEFGHIJKLMNOPQRSTUVXYZ";
    let numbers = "0123456789";
    let chars = "\\!\"£$%&/()=?^*°:;.,";
    let all = format!("{}{}{}{}", low_case, up_case, numbers, chars);
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..all.len() - 2);
    let mut password = "".to_owned();
    for _n in 1..length {
        let throw = die.sample(&mut rng);
        let mut char = all.chars();
        let char = char.nth(throw).unwrap();
        password = format!("{}{}", password, char);
    }
    return password;
}
