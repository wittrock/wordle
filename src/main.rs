use indexmap::IndexMap;

use std::{
    collections::{BTreeMap, HashSet},
    fs,
    io::BufRead,
    io::BufReader,
};

fn is_possible_starting_word(word: &&String) -> bool {
    if word.len() != 5 {
        return false;
    }

    // No apostrophes, hyphens, or special chars
    if !word.chars().all(|c| c.is_ascii() && c.is_alphanumeric()) {
        return false;
    }

    return true;
}

fn un_pluralize(word: &String) -> &str {
    word.strip_suffix('s').unwrap_or(word)
}

fn letter_frequencies(words: &HashSet<String>) -> BTreeMap<char, f32> {
    let mut occurrences = BTreeMap::<char, usize>::new();
    let mut total_letters: usize = 0;
    for word in words {
        if word.contains('\'') {
            // Don't count possessives
            continue;
        }

        if word.ends_with('s') && words.contains(un_pluralize(word)) {
            // This is a plural form of a word that already exists.
            continue;
        }

        for character in word.chars().filter(|c| c.is_ascii() && c.is_alphanumeric()) {
            *occurrences.entry(character).or_insert(0) += 1;
            total_letters += 1;
        }
    }

    let mut frequencies = BTreeMap::<char, f32>::new();
    for (character, total_occurrences) in occurrences {
        frequencies.insert(character, total_occurrences as f32 / total_letters as f32);
    }

    frequencies
}

fn single_word_score(word: &String, letter_frequencies: &BTreeMap<char, f32>) -> f32 {
    let mut letter_usage = HashSet::<char>::new();
    for char in word.chars() {
        letter_usage.insert(char);
    }

    let mut score: f32 = 0.0;
    for char in letter_usage {
        score += letter_frequencies.get(&char).unwrap();
    }

    score
}

fn score_single_words(
    words: &Vec<&String>,
    letter_frequencies: &BTreeMap<char, f32>,
) -> IndexMap<String, f32> {
    let mut word_scores = IndexMap::<String, f32>::new();

    // Simple scoring algorithm:
    // score = sum of frequencies of individual letters, not counting the frequency for every
    //         repeated letter.

    for word in words {
        word_scores.insert(
            word.to_string(),
            single_word_score(word, letter_frequencies),
        );
    }

    word_scores
}

fn score_word_pair(
    word1: &String,
    letter_frequencies: &BTreeMap<char, f32>,
    words: &Vec<&String>,
    score_map: &mut IndexMap<(String, String), f32>,
) {
    for word2 in words {
        // We've already scored this pair, and don't want dups.
        // TODO(wittrock): does order matter here?
        if score_map
            .get(&((*word1).to_owned(), (*word2).to_owned()))
            .is_some()
            || score_map
                .get(&((*word2).to_owned(), (*word1).to_owned()))
                .is_some()
        {
            continue;
        }

        let mut score = single_word_score(word1, letter_frequencies);

        let score2 = single_word_score(word2, letter_frequencies);
        score += score2;

        let word1_chars: Vec<char> = word1.chars().collect();
        for (index, char) in word2.chars().enumerate() {
            // Don't give credit for this letter if it's in the same place in the other word.
            if word1_chars[index] == char {
                continue;
            }

            score -= letter_frequencies.get(&char).unwrap();
        }

        score_map.insert(((*word1).to_owned(), (*word2).to_owned()), score);
    }
}

fn score_word_pairs(
    words: &Vec<&String>,
    letter_frequencies: &BTreeMap<char, f32>,
) -> IndexMap<(String, String), f32> {
    // We _don't_ want to give extra credit for letters in the same position in each word,
    // since they won't help us.
    let mut pair_scores = IndexMap::<(String, String), f32>::new();

    println!(
        "Iterating over {} words, {} pairs",
        words.len(),
        words.len() * words.len()
    );
    for (word_index, word1) in words.iter().enumerate() {
        if word_index % 100 == 0 {
            println!(
                "[{}%]: {}",
                ((word_index as f32 / words.len() as f32) * 100.0) as u32,
                word1
            );
        }
        score_word_pair(word1, letter_frequencies, words, &mut pair_scores);
    }

    pair_scores
}

fn main() -> std::io::Result<()> {
    println!("Parsing dictionary");
    let dict_file = fs::File::open("/usr/share/dict/american-english")?;
    let buf = BufReader::new(dict_file);
    let all_words: HashSet<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .filter(|w| w.chars().nth(0).unwrap().is_ascii_lowercase()) // remove proper nouns
        .map(|w| w.to_ascii_lowercase())
        .collect();

    let starting_words: Vec<&String> = all_words.iter().filter(is_possible_starting_word).collect();

    println!(
        "Got {} possible starting words out of {} total words",
        starting_words.len(),
        all_words.len()
    );

    // There are a couple of heuristics we could try to find the best starting words
    // 1. The best individual starting words, which contain:
    //      * no repeated letters
    //      * a set of letters which are closest to the most frequent letters in English
    //    This could be approximated as a score corresponding to the sum of frequencies of each letter.
    // 2. The best _pairs_ of words, which:
    //      * together have the highest score using the algorithm above
    //      * don't share letters in the same position (will need to adjust score for this)

    // First, we need the frequencies of English letters.
    // Let's derive that from our dictionary.
    let frequencies = letter_frequencies(&all_words);
    println!("Letter occurrences: {:#?}", frequencies);

    // Now, we need to come up with a score for each word.
    let word_scores = score_single_words(&starting_words, &frequencies);

    println!(
        "Best single words (score): {:#?}",
        word_scores
            .sorted_by(|_k1, v1, _k2, v2| v2.partial_cmp(v1).unwrap())
            .take(20)
            .collect::<Vec<(String, f32)>>()
    );

    let pair_scores = score_word_pairs(&starting_words, &frequencies);

    println!(
        "Best word pairs (score): {:#?}",
        pair_scores
            .sorted_by(|_k1, v1, _k2, v2| v2.partial_cmp(v1).unwrap())
            .take(20)
            .collect::<Vec<((String, String), f32)>>()
    );

    Ok(())
}
