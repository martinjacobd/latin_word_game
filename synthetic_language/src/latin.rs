use super::{InflectionalCategory};

// Nouns and Adjectives
enum Gender {
    Masculine,
    Feminine,
    Common,
    Neuter,
}

enum Case {
    Nominative,
    Genitive,
    Dative,
    Accusative,
    Ablative,
    Vocative,
    Locative,
}

enum Number {
    Singular,
    Plural,
}

struct NominalCategories {
    gender: Gender,
    case: Case,
    number: Number,
}

struct NounDeclension<'a> {
    name: &'a str,
    suffixes: [&'a str; 42]
}