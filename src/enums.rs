/// Represents the type of components found in Korean syllables,
/// including their classification as initial (first) consonants, medial (middle) vowels,
/// final (last) consonants, characters that can serve as either initial or final consonants,
/// numeric characters, and characters that do not fit into the previous categories.
#[derive(PartialEq, Debug)]
pub enum SyllableType {
    FirstConsonantLetter,
    MiddleVowelLetter,
    LastConsonantLetter,
    BothFirstLastConsonant,
    Number,
    NotConsonant,
}

/// Represents the classification of Korean characters into distinct types based on their phonetic and structural characteristics.
/// This includes basic consonants, basic vowels, complex or double consonants, complex vowels, and a category for characters that do not fit into the aforementioned classifications.
#[derive(PartialEq, Debug)]
pub enum KoreanType {
    Consonant,      // Basic consonants (자음)
    Vowel,          // Basic vowels (모음)
    ComplexConsonant, // Complex or double consonants (복합자음)
    ComplexVowel,   // Complex vowels (복합모음)
    Unknown         // For characters that do not fit in the above categories (알 수 없는 유형)
}