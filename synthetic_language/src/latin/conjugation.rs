use crate::*;
use category_derive::suffix_inflection_over_categories;

suffix_inflection_over_categories! {
    VerbConjugation
    VerbCategories

    pub enum Person {
        First,
        Second,
        Third,
    }

    pub enum Number {
        Singular,
        Plural,
    }

    pub enum Voice {
        Active,
        Passive,
    }

    pub enum Mood {
        Indicative,
        Subjunctive,
        Imperative,
        Participle,
    }
}
