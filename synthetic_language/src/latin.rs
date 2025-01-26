use super::{InflectionalCategory, InflectionalCategorySet, SuffixInflection};
use category_derive::{suffix_inflection_over_categories, InflectionalCategory, InflectionalCategorySet, SuffixInflection};

// Nouns and Adjectives

suffix_inflection_over_categories! {
    NominalDeclension
    NominalCategories

    pub enum Gender {
        Masculine,
        Feminine,
        Common,
        Neuter,
    }

    pub enum Case {
        Nominative,
        Genitive,
        Dative,
        Accusative,
        Ablative,
        Vocative,
        Locative,
    }

    pub enum Number {
        Singular,
        Plural,
    }
}
