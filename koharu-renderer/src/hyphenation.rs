use hyphenation::{Hyphenator, Iter, Language, Load, Standard};

// Re-export Language from hyphenation for convenience
pub use hyphenation::Language as HyphenationLanguage;

/// Word hyphenator using the Knuth-Liang algorithm.
///
/// This provides linguistically correct hyphenation for many languages,
/// using the same algorithm that powers TeX/LaTeX hyphenation.
pub struct WordHyphenator {
    hyphenator: Standard,
}

impl WordHyphenator {
    /// Creates a new hyphenator for the specified language.
    ///
    /// Falls back to English (US) if the specified language is not available.
    pub fn new(lang: Language) -> Self {
        Self {
            hyphenator: Standard::from_embedded(lang)
                .unwrap_or_else(|_| Standard::from_embedded(Language::EnglishUS).unwrap()),
        }
    }

    /// Creates an English (US) hyphenator.
    pub fn english() -> Self {
        Self::new(Language::EnglishUS)
    }

    /// Find all valid hyphenation points in a word.
    ///
    /// Returns character indices where the word can be split.
    pub fn hyphenation_points(&self, word: &str) -> Vec<usize> {
        let hyphenated = self.hyphenator.hyphenate(word);
        let breaks: Vec<String> = hyphenated.iter().collect();

        if breaks.len() <= 1 {
            return Vec::new();
        }

        let mut points = Vec::new();
        let mut char_pos = 0;

        for segment in &breaks[..breaks.len() - 1] {
            // The hyphenation library includes the hyphen '-' at the end of each segment.
            // We need to strip it before counting to get the actual character position.
            let clean_segment = segment.trim_end_matches('-');
            char_pos += clean_segment.chars().count();
            points.push(char_pos);
        }

        points
    }

    /// Find the best split point in a word, preferring positions near the center.
    ///
    /// Returns the character index for the split, or `None` if the word shouldn't be split.
    pub fn find_split_point(&self, word: &str) -> Option<usize> {
        let points = self.hyphenation_points(word);
        if points.is_empty() {
            return None;
        }

        let word_len = word.chars().count();
        let target = word_len / 2;

        // Find the hyphenation point closest to the center
        points
            .into_iter()
            .min_by_key(|&pos| (pos as isize - target as isize).unsigned_abs())
    }
}

impl Default for WordHyphenator {
    fn default() -> Self {
        Self::english()
    }
}

/// Find the longest word in the text (by character count).
/// A "word" is a sequence of non-whitespace characters.
pub fn find_longest_word(text: &str) -> String {
    text.split_whitespace()
        .max_by_key(|word| word.chars().count())
        .unwrap_or("")
        .to_string()
}

/// Split the longest word in the text at a linguistically correct hyphenation point.
/// Returns the modified text with the word split as "part1- part2".
///
/// Uses the hyphenation library (Knuth-Liang algorithm) for proper syllable splitting.
/// If no valid hyphenation point is found, returns the original text unchanged.
pub fn split_longest_word(text: &str, word: &str, hyphenator: &WordHyphenator) -> String {
    if word.is_empty() {
        return text.to_string();
    }

    // Strip punctuation before hyphenating to get clean syllable boundaries
    let (prefix, clean_word, suffix) = strip_punctuation(word);

    // If after stripping there's nothing left to split, return original
    if clean_word.is_empty() || clean_word.chars().count() < 2 {
        return text.to_string();
    }

    // Use proper hyphenation on the clean word
    let split_pos = match hyphenator.find_split_point(&clean_word) {
        Some(pos) => pos,
        None => return text.to_string(), // No valid hyphenation point found
    };

    let chars: Vec<char> = clean_word.chars().collect();

    if split_pos == 0 || split_pos >= chars.len() {
        return text.to_string();
    }

    let part1: String = chars[..split_pos].iter().collect();
    let part2: String = chars[split_pos..].iter().collect();

    // Reconstruct with prefix on part1, suffix on part2
    let replacement = format!("{}{}- {}{}", prefix, part1, part2, suffix);

    // Replace only the first occurrence
    text.replacen(word, &replacement, 1)
}

/// Check if a character is word-boundary punctuation that should be stripped.
fn is_word_punctuation(c: char) -> bool {
    matches!(
        c,
        '.' | ','
            | '!'
            | '?'
            | ':'
            | ';'
            | '"'
            | '\''
            | '('
            | ')'
            | '['
            | ']'
            | '{'
            | '}'
            | '«'
            | '»'
            | '„'
            | '\u{201C}'
            | '\u{201D}'
            | '\u{2018}'
            | '\u{2019}'
            | '…'
            | '–'
            | '—'
    )
}

/// Strip leading and trailing punctuation from a word.
/// Returns (prefix, clean_word, suffix).
fn strip_punctuation(word: &str) -> (String, String, String) {
    let chars: Vec<char> = word.chars().collect();
    let len = chars.len();

    if len == 0 {
        return (String::new(), String::new(), String::new());
    }

    // Find start of actual word (skip leading punctuation)
    let start = chars
        .iter()
        .position(|c| !is_word_punctuation(*c))
        .unwrap_or(len);

    // Find end of actual word (skip trailing punctuation)
    let end = chars
        .iter()
        .rposition(|c| !is_word_punctuation(*c))
        .map(|pos| pos + 1)
        .unwrap_or(0);

    if start >= end {
        // Word is all punctuation
        return (word.to_string(), String::new(), String::new());
    }

    let prefix: String = chars[..start].iter().collect();
    let clean: String = chars[start..end].iter().collect();
    let suffix: String = chars[end..].iter().collect();

    (prefix, clean, suffix)
}

/// Maps a language code string to the hyphenation Language enum.
/// Returns None if the language code is not recognized.
pub fn map_language_code(code: &str) -> Option<Language> {
    match code.to_lowercase().as_str() {
        // German
        "de" | "de-de" | "german" => Some(Language::German1996),
        "de-1901" | "german-1901" => Some(Language::German1901),
        "de-ch" | "german-swiss" => Some(Language::GermanSwiss),

        // English
        "en" | "en-us" | "english" | "english-us" => Some(Language::EnglishUS),
        "en-gb" | "english-gb" | "english-uk" => Some(Language::EnglishGB),

        // French
        "fr" | "fr-fr" | "french" => Some(Language::French),

        // Spanish
        "es" | "es-es" | "spanish" => Some(Language::Spanish),

        // Italian
        "it" | "it-it" | "italian" => Some(Language::Italian),

        // Portuguese
        "pt" | "pt-pt" | "pt-br" | "portuguese" => Some(Language::Portuguese),

        // Dutch
        "nl" | "nl-nl" | "dutch" => Some(Language::Dutch),

        // Polish
        "pl" | "pl-pl" | "polish" => Some(Language::Polish),

        // Russian
        "ru" | "ru-ru" | "russian" => Some(Language::Russian),

        // Swedish
        "sv" | "sv-se" | "swedish" => Some(Language::Swedish),

        // Danish
        "da" | "da-dk" | "danish" => Some(Language::Danish),

        // Finnish
        "fi" | "fi-fi" | "finnish" => Some(Language::Finnish),

        // Czech
        "cs" | "cs-cz" | "czech" => Some(Language::Czech),

        // Hungarian
        "hu" | "hu-hu" | "hungarian" => Some(Language::Hungarian),

        // Turkish
        "tr" | "tr-tr" | "turkish" => Some(Language::Turkish),

        // Greek
        "el" | "el-gr" | "greek" => Some(Language::GreekMono),

        // Ukrainian
        "uk" | "uk-ua" | "ukrainian" => Some(Language::Ukrainian),

        // Croatian
        "hr" | "hr-hr" | "croatian" => Some(Language::Croatian),

        // Romanian
        "ro" | "ro-ro" | "romanian" => Some(Language::Romanian),

        // Slovak
        "sk" | "sk-sk" | "slovak" => Some(Language::Slovak),

        // Slovenian
        "sl" | "sl-si" | "slovenian" => Some(Language::Slovenian),

        // Bulgarian
        "bg" | "bg-bg" | "bulgarian" => Some(Language::Bulgarian),

        // Catalan
        "ca" | "ca-es" | "catalan" => Some(Language::Catalan),

        // Estonian
        "et" | "et-ee" | "estonian" => Some(Language::Estonian),

        // Latvian
        "lv" | "lv-lv" | "latvian" => Some(Language::Latvian),

        // Lithuanian
        "lt" | "lt-lt" | "lithuanian" => Some(Language::Lithuanian),

        // Indonesian
        "id" | "id-id" | "indonesian" => Some(Language::Indonesian),

        // Latin
        "la" | "latin" => Some(Language::Latin),

        // No match
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hyphenator_finds_english_hyphenation_points() {
        let hyph = WordHyphenator::english();

        // "internationalization" should have multiple hyphenation points
        let points = hyph.hyphenation_points("internationalization");
        assert!(!points.is_empty(), "should find hyphenation points");

        // The hyphenation points should be at syllable boundaries
        // in-ter-na-tion-al-iza-tion
        assert!(points.len() >= 3, "should have multiple split points");
    }

    #[test]
    fn hyphenator_finds_german_hyphenation_points() {
        let hyph = WordHyphenator::new(Language::German1996);

        // "Donaudampfschifffahrt" is a famous long German compound word
        let points = hyph.hyphenation_points("Donaudampfschifffahrt");
        assert!(
            !points.is_empty(),
            "should find hyphenation points for German"
        );
    }

    #[test]
    fn hyphenator_finds_split_point_near_center() {
        let hyph = WordHyphenator::english();

        // For "internationalization", the split should be near the middle
        let word = "internationalization";
        let split = hyph.find_split_point(word);
        assert!(split.is_some());

        let pos = split.unwrap();
        let len = word.chars().count();
        let mid = len / 2;

        // The split should be reasonably close to the middle (within 5 chars)
        assert!(
            (pos as isize - mid as isize).unsigned_abs() <= 5,
            "split at {} should be near middle {}",
            pos,
            mid
        );
    }

    #[test]
    fn hyphenator_returns_none_for_short_words() {
        let hyph = WordHyphenator::english();

        // Very short words typically have no hyphenation points
        assert!(hyph.hyphenation_points("cat").is_empty());
        assert!(hyph.hyphenation_points("dog").is_empty());
        assert!(hyph.find_split_point("hi").is_none());
    }

    #[test]
    fn split_longest_word_with_hyphenator() {
        let hyph = WordHyphenator::english();

        let text = "This is internationalization test";
        let result = split_longest_word(text, "internationalization", &hyph);

        // Should contain a hyphen followed by space
        assert!(
            result.contains("- "),
            "result should contain hyphen: {}",
            result
        );
        // Should still contain both parts of the word
        assert!(result.contains("inter"), "should have first part");
    }

    #[test]
    fn split_longest_word_returns_unchanged_if_no_hyphenation() {
        let hyph = WordHyphenator::english();

        // Short words with no hyphenation points should be returned unchanged
        let text = "Test cat dog";
        let result = split_longest_word(text, "cat", &hyph);

        // Should not contain a hyphen since "cat" has no hyphenation points
        assert_eq!(result, text, "short words should not be split");
    }

    #[test]
    fn strip_punctuation_trailing_period() {
        let (prefix, clean, suffix) = strip_punctuation("word.");
        assert_eq!(prefix, "");
        assert_eq!(clean, "word");
        assert_eq!(suffix, ".");
    }

    #[test]
    fn strip_punctuation_trailing_exclamation() {
        let (prefix, clean, suffix) = strip_punctuation("Hello!");
        assert_eq!(prefix, "");
        assert_eq!(clean, "Hello");
        assert_eq!(suffix, "!");
    }

    #[test]
    fn strip_punctuation_multiple_trailing() {
        let (prefix, clean, suffix) = strip_punctuation("What?!");
        assert_eq!(prefix, "");
        assert_eq!(clean, "What");
        assert_eq!(suffix, "?!");
    }

    #[test]
    fn strip_punctuation_leading_quote() {
        let (prefix, clean, suffix) = strip_punctuation("\"quoted\"");
        assert_eq!(prefix, "\"");
        assert_eq!(clean, "quoted");
        assert_eq!(suffix, "\"");
    }

    #[test]
    fn strip_punctuation_ellipsis() {
        let (prefix, clean, suffix) = strip_punctuation("wait...");
        assert_eq!(prefix, "");
        assert_eq!(clean, "wait");
        assert_eq!(suffix, "...");
    }

    #[test]
    fn strip_punctuation_german_quotes() {
        // German opening quote „ (U+201E) and closing quote " (U+201C)
        let (prefix, clean, suffix) = strip_punctuation("\u{201E}Wort\u{201C}");
        assert_eq!(prefix, "\u{201E}");
        assert_eq!(clean, "Wort");
        assert_eq!(suffix, "\u{201C}");
    }

    #[test]
    fn strip_punctuation_no_punctuation() {
        let (prefix, clean, suffix) = strip_punctuation("hello");
        assert_eq!(prefix, "");
        assert_eq!(clean, "hello");
        assert_eq!(suffix, "");
    }

    #[test]
    fn strip_punctuation_all_punctuation() {
        let (prefix, clean, suffix) = strip_punctuation("...");
        assert_eq!(prefix, "...");
        assert_eq!(clean, "");
        assert_eq!(suffix, "");
    }

    #[test]
    fn split_word_with_trailing_period_german() {
        let hyph = WordHyphenator::new(Language::German1996);

        // This is the real-world case: using a long German compound word with punctuation
        let text = "Test Persönlichkeitsausscheidung. Ende";
        let result = split_longest_word(text, "Persönlichkeitsausscheidung.", &hyph);

        // The period should be preserved at the end
        assert!(
            result.contains("."),
            "period should be preserved: {}",
            result
        );
        // Should have a hyphen
        assert!(result.contains("- "), "should have hyphen: {}", result);
        // The split should be at a syllable boundary, not including the period in the analysis
        // German hyphenation for "Persönlichkeitsausscheidung" splits at proper syllables
    }

    #[test]
    fn split_word_with_question_mark_english() {
        let hyph = WordHyphenator::english();

        let text = "Is this internationalization?";
        let result = split_longest_word(text, "internationalization?", &hyph);

        // The question mark should be preserved
        assert!(
            result.contains("?"),
            "question mark should be preserved: {}",
            result
        );
        assert!(result.contains("- "), "should have hyphen: {}", result);
    }

    #[test]
    fn split_word_with_quotes() {
        let hyph = WordHyphenator::english();

        let text = "The word \"internationalization\" is long";
        let result = split_longest_word(text, "\"internationalization\"", &hyph);

        // Both quotes should be preserved
        assert!(
            result.contains("\""),
            "quotes should be preserved: {}",
            result
        );
        assert!(result.contains("- "), "should have hyphen: {}", result);
    }
}
