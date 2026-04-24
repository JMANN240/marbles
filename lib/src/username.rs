use fake::{
    Fake,
    faker::{
        name::en::{FirstName, LastName},
        number::en::Digit,
    },
};
use heck::{AsPascalCase, AsSnekCase};
use rand::{Rng, RngExt, seq::IndexedRandom};

pub fn generate_username<R: Rng>(rng: &mut R) -> String {
    let mut username = String::new();

    let separator = ["", ".", "_"].choose(rng).unwrap();

    let mut first_was_short = false;

    if rng.random_bool(0.9) {
        let first_name = randomly_cased(rng, &FirstName().fake::<String>());

        let varied = vary(rng, first_name);

        if varied.len() == 1 {
            first_was_short = true;
        }

        username.push_str(&varied);
    } else if rng.random_bool(0.2) {
        let last_name = randomly_cased(rng, &LastName().fake::<String>());

        let varied = vary(rng, last_name);

        if varied.len() == 1 {
            first_was_short = true;
        }

        username.push_str(&varied);
    }

    if !username.is_empty() && rng.random_bool(0.8) {
        username.push_str(separator);
    }

    if username.is_empty() || first_was_short || rng.random_bool(0.9) {
        let last_name = randomly_cased(rng, &LastName().fake::<String>());

        username.push_str(&vary(rng, last_name));
    }

    if !username.is_empty() && rng.random_bool(0.8) {
        username.push_str(separator);
    }

    if rng.random_bool(0.4) {
        username.push_str(&Digit().fake::<String>());
    }

    if rng.random_bool(0.4) {
        username.push_str(&Digit().fake::<String>());
    }

    if rng.random_bool(0.4) {
        username.push_str(&Digit().fake::<String>());
    }

    if rng.random_bool(0.05) {
        username.push('_');
    }

    if username.len() < 6 {
        generate_username(rng)
    } else {
        username
    }
}

pub fn randomly_cased<R: Rng>(rng: &mut R, text: &str) -> String {
    match rng.random_range(0..2) {
        0 => {
            format!("{}", AsPascalCase(text))
        }
        1 => {
            format!("{}", AsSnekCase(text))
        }
        _ => unreachable!()
    }
}

pub fn vary<R: Rng>(rng: &mut R, text: String) -> String {
    if rng.random_bool(0.4) {
        text[0..1].to_string()
    } else if text.len() > 2 && rng.random_bool(0.4) {
        let (left, right) = text.split_at(rng.random_range(1..(text.len() - 1)));
        let extended = format!("{}{}{}", left, right[0..1].repeat(2), right);

        extended
    } else {
        text
    }
}