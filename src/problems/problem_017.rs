use std::collections::HashMap;

pub fn solve(limit: u32) -> u32{
    let word_mapping = get_words_for_nb();
    let mut total_letters = 0;

    for mut i in 1..(limit + 1) {
        let mut word = String::new();
        let thousands = i / 1000;
        i -= thousands * 1000;
        let hundreds = i / 100;
        i -= hundreds * 100;
        let tens = i / 10;
        i -= tens * 10;
        let unit = i;

        if thousands > 0 {
            let corresponding_word = word_mapping.get(&thousands).unwrap();
            let correspondung_unit = word_mapping.get(&1000).unwrap();
            word.push_str(&format!("{}{}", corresponding_word, correspondung_unit));
        }

        if hundreds > 0 {
            let corresponding_word = word_mapping.get(&hundreds).unwrap();
            let correspondung_unit = word_mapping.get(&100).unwrap();
            word.push_str(&format!("{}{}", corresponding_word, correspondung_unit));
        }

        if (tens != 0 || unit != 0) && (hundreds != 0 || thousands != 0) {
            word.push_str("and");
        }

        if tens == 1 {
            let corresponding_word = word_mapping.get(&(tens * 10 + unit)).unwrap();
            word.push_str(&format!("{}", corresponding_word));
        } else {
            if tens > 1 {
                let corresponding_word = word_mapping.get(&(tens * 10)).unwrap();
                word.push_str(&format!("{}", corresponding_word)); 
            }

            if unit > 0 {
                let corresponding_word = word_mapping.get(&unit).unwrap();
                word.push_str(&format!("{}", corresponding_word));
            }
        }
        total_letters += word.len() as u32;
        
    }
    total_letters
}

fn get_words_for_nb() -> HashMap<u32, &'static str> {
    let mut map: HashMap<u32, &str> = HashMap::new();

    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");
    map.insert(4, "four");
    map.insert(5, "five");
    map.insert(6, "six");
    map.insert(7, "seven");
    map.insert(8, "eight");
    map.insert(9, "nine");
    map.insert(10, "ten");
    map.insert(11, "eleven");
    map.insert(12, "twelve");
    map.insert(13, "thirteen");
    map.insert(14, "fourteen");
    map.insert(15, "fifteen");
    map.insert(16, "sixteen");
    map.insert(17, "seventeen");
    map.insert(18, "eighteen");
    map.insert(19, "nineteen");
    map.insert(20, "twenty");
    map.insert(30, "thirty");
    map.insert(40, "forty");
    map.insert(50, "fifty");
    map.insert(60, "sixty");
    map.insert(70, "seventy");
    map.insert(80, "eighty");
    map.insert(90, "ninety");
    map.insert(100, "hundred");
    map.insert(1000, "thousand");

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_correct_result() {
        assert_eq!(solve(1000), 21124);
    }
}