use category_derive::{suffix_inflection_over_categories, suffixes};
use crate::*;

suffix_inflection_over_categories! {
    NominalDeclension
    NominalCategories

    pub enum Gender {
        Feminine,
        Masculine,
        Common,
        Neuter,
    }

    pub enum Number {
        Singular,
        Plural,
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
}

pub const FIRST_DECLENSION: NominalDeclension = NominalDeclension {
    name: "First Declension",
    suffixes: suffixes! [
        "a"  "ae"   "ae" "am" "ā"  "a"  "ae"
        "ae" "arum" "īs" "ās" "īs" "ae" "īs"
        "a"  "ae"   "ae" "am" "ā"  "a"  "ae"
        "ae" "arum" "īs" "ās" "īs" "ae" "īs"
        "a"  "ae"   "ae" "am" "ā"  "a"  "ae"
        "ae" "arum" "īs" "ās" "īs" "ae" "īs"
        N    N      N    N    N    N    N
        N    N      N    N    N    N    N
    ],
};

pub const SECOND_DECLENSION: NominalDeclension = NominalDeclension {
    name: "Second Declension",
    suffixes: suffixes! [
        "us" "ī"    "o"  "um" "o"  "e"  "ī"
        "ī"  "ōrum" "īs" "ōs" "īs" "ī"  "īs"
        "us" "ī"    "o"  "um" "o"  "e"  "ī"
        "ī"  "ōrum" "īs" "ōs" "īs" "ī"  "īs"
        "us" "ī"    "o"  "um" "o"  "e"  "ī"
        "ī"  "ōrum" "īs" "ōs" "īs" "ī"  "īs"
        "um" "ī"    "o"  "um" "o"  "um" "ī"
        "a"  "ōrum" "īs" "a" "īs"  "a"  "īs"
    ],
};

pub const THIRD_DECLENSION: NominalDeclension = NominalDeclension {
    name: "Third Declension",
    suffixes: suffixes! [
        "is"   "is"   "ī"    "em" "e"    "is" "ī"
        "ēs"   "um"   "ibus" "ēs" "ibus" "ēs" "ibus"
        "is"   "is"   "ī"    "em" "e"    "is" "ī"
        "ēs"   "um"   "ibus" "ēs" "ibus" "ēs" "ibus"
        "is"   "is"   "ī"    "em" "e"    "is" "ī"
        "ēs"   "um"   "ibus" "ēs" "ibus" "ēs" "ibus"
        "e"    "is"   "ī"    "e"  "e"    "e"  "ī"
        "a"    "um"   "ibus" "a"  "ibus" "a"  "ibus"
    ],
};

pub const FOURTH_DECLENSION: NominalDeclension = NominalDeclension {
    name: "Fourth Declension",
    suffixes: suffixes! [
        "us"  "ūs"   "uī"   "um" "ū"    "us" N
        "ūs"  "uum"  "ibus" "ūs" "ibus" "ūs" N
        "us"  "ūs"   "uī"   "um" "ū"    "us" N
        "ūs"  "uum"  "ibus" "ūs" "ibus" "ūs" N
        "us"  "ūs"   "uī"   "um" "ū"    "us" N
        "ūs"  "uum"  "ibus" "ūs" "ibus" "ūs" N
        "ū"   "ūs"   "ū"    "ū"  "ū"    "ū"  N
        "ua"  "uum"  "ibus" "ua" "ibus" "ua" N
    ],
};

pub const FIFTH_DECLENSION: NominalDeclension = NominalDeclension {
    name: "Fifth Declension",
    suffixes: suffixes! [
        "ēs" "ēī"   "ēī"   "em" "ē"    "ēs" N
        "ēs" "ērum" "ēbus" "ēs" "ēbus" "ēs" N
        "ēs" "ēī"   "ēī"   "em" "ē"    "ēs" N
        "ēs" "ērum" "ēbus" "ēs" "ēbus" "ēs" N
        "ēs" "ēī"   "ēī"   "em" "ē"    "ēs" N
        "ēs" "ērum" "ēbus" "ēs" "ēbus" "ēs" N
        N    N      N      N    N      N    N
        N    N      N      N    N      N    N
    ],
};

pub const FIRST_SECOND_ADJECTIVE_DECLENSION: NominalDeclension = NominalDeclension {
    name: "First/Second Adjective Declension",
    suffixes: suffixes! [
        "a"  "ae"   "ae" "am" "ā"  "a"  "ae"
        "ae" "arum" "īs" "ās" "īs" "ae" "īs"
        "us" "ī"    "o"  "um" "o"  "e"  "ī"
        "ī"  "ōrum" "īs" "ōs" "īs" "ī"  "īs"
        N    N      N    N    N    N    N
        N    N      N    N    N    N    N
        "um" "ī"    "o"  "um" "o"  "um" "ī"
        "a"  "ōrum" "īs" "a" "īs"  "a"  "īs"
    ],
};

pub const THIRD_ADJECTIVE_DECLENSION: NominalDeclension = NominalDeclension {
    name: "Third Adjective Declension",
    suffixes: suffixes! [
        "is"   "is"   "ī"    "em" "e"    "is" "ī"
        "ēs"   "um"   "ibus" "ēs" "ibus" "ēs" "ibus"
        "is"   "is"   "ī"    "em" "e"    "is" "ī"
        "ēs"   "um"   "ibus" "ēs" "ibus" "ēs" "ibus"
        "is"   "is"   "ī"    "em" "e"    "is" "ī"
        "ēs"   "um"   "ibus" "ēs" "ibus" "ēs" "ibus"
        "e"    "is"   "ī"    "e"  "e"    "e"  "ī"
        "a"    "um"   "ibus" "a"  "ibus" "a"  "ibus"
    ],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffixes() {
        let poeta = crate::Word {
            inflection: FIRST_DECLENSION,
            lemma: "poeta",
            root: "poet",
            regular: true,
            irregular_forms: vec![],
        };

        assert_eq!(poeta.inflect(NominalCategories(Gender::Masculine, Number::Plural, Case::Accusative)).unwrap(), "poetās");
        assert_eq!(NominalCategories(Gender::Common, Number::Plural, Case::Genitive).index(), 36);
    }
}