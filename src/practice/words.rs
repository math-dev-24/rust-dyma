struct Sentence {
    sentence: String,
    words: Vec<(usize, String)>,
}

impl Sentence {
    fn new(sentence: String) -> Sentence {
        let mut words = vec![];
        for word in sentence.split_whitespace() {
            let count_char = word.chars().count();
            words.push((count_char, word.to_string()));
        }
        Sentence { sentence, words }
    }

    fn get_acronym(&self) -> String {
        self.words.iter()
            .filter_map(|(_, word)| word.chars().next())
            .collect()
    }

    fn is_palindrome(&self) -> Option<bool> {
        if self.words.len() > 1 {
            return None
        }
        let word_reversed = self.sentence.chars().rev().collect::<String>();
        if word_reversed == self.sentence {Some(true)} else {None}
    }

    fn count_vowel(&self) -> usize {
        let list_vowels = vec!['a', 'e', 'i', 'o', 'u', 'y', 'A', 'E', 'I', 'O', 'U'];
        self.words.iter()
            .map(|(_, word)| word.chars().filter(|c| list_vowels.contains(c)).count())
            .sum()
    }

    fn sort_by_len_word(&mut self) -> String {
        self.words.sort_by_key(|&(k, _)| k);
        let result = self.words.iter()
            .map(|(_, word)| word.to_string()).collect::<Vec<String>>();
        result.join(" ")
    }
}


pub fn words(input_text: &str) {
    let mut sentences = Sentence::new(input_text.to_string());
    println!("Nombre de voyelle {}", sentences.count_vowel());
    if let Some(true) = sentences.is_palindrome() {
        println!("C'est un palindrome !");
    }else {
        println!("Ce n'est pas un palindrome !");
    }
    println!("Sorted : {}", sentences.sort_by_len_word());
    println!("Acronym: {}", sentences.get_acronym());
}
