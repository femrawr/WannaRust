pub mod config;
pub mod crypto;

use rand::{Rng, thread_rng};
use rand::distributions::{Alphanumeric, Distribution};

const SPECIAL: [char; 42] = [
    'ʬ', 'ᚖ', 'Ӕ', 'ʮ', '⚆', 'ᘎ', 'ჴ',
    '҂', 'ᴥ', 'ᑯ','ȹ', 'Ɐ', 'Ꝏ', '𐋦',
    'ᓭ', 'ᖰ', 'ᘉ', '₭', 'ᴖ', 'ꙮ', '∯',
    'ʡ', 'Ѯ', '྅', 'Ⴁ', '₼', 'ᓷ', 'ʢ',
    '⸘', 'ⱷ', 'Ꙋ', 'Ƣ', 'ʠ', 'ͷ', 'ᗐ',
    'ᘀ', '߷', '҉', 'Ꚙ', 'Ѳ', 'ᘣ','₣'
];

pub fn test_entry(name: &str) { 
    println!("hello, {} from shared test function.", name);
}

pub fn gen_string(len: usize) -> String {
    let mut rng = thread_rng();

    let string = (0..len)
        .map(|_| {
            if rng.gen_ratio(1, 2) {
                SPECIAL[rng.gen_range(0..SPECIAL.len())]
            } else {
                char::from(Alphanumeric.sample(&mut rng))
            }
        })
        .collect();

    string
}