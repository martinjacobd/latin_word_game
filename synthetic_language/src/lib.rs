pub mod latin;

use category_derive::*;

/// An `InflectionalCategory` is a salient category used when inflecting a word, such as
/// gender, number, case, tense, aspect, mood, etc.
pub trait InflectionalCategory: PartialEq {
    fn index(&self) -> usize;

    fn iter_through_variants() -> impl Iterator<Item = Self>;
}

/// An `InflectionalCategorySet` is the set of salient categories which together _determine_ the
/// inflection of a word, such as gender, number, and case for a Latin adjective.
pub trait InflectionalCategorySet: PartialEq {
    type IndexType;
    fn index(&self) -> Self::IndexType;

    fn iter_through_variants() -> impl Iterator<Item = Self>;
}

/// An `Inflection` is a set of transformations on a root which, when given the relevant
/// `InflectionalCategorySet` will give a fully inflected word (if it exists).
pub trait Inflection<'a> {
    type CategorySet: InflectionalCategorySet;

    fn inflect(&self, root: &'a str, categories: Self::CategorySet) -> Option<String>;
}

/// A `SuffixInflection` is a special case of an `Inflection` in which roots are merely given
/// suffixes in the happy path. There are cases, such as the Latin third declension neuter, in
/// which the lemma form and not the root is used. We can deal with this by using the `Option<>`
/// variant `None` in those cases and adding cases to deal with that.
pub trait SuffixInflection<'a> {
    type CategorySet: InflectionalCategorySet;

    fn suffix (&self, categories: Self::CategorySet) -> Option<&'a str>;
}

impl<'a, T> Inflection<'a> for T where T: SuffixInflection<'a> {
    type CategorySet = T::CategorySet;

    fn inflect(&self, root: &'a str, categories: T::CategorySet) -> Option<String> {
        let suffix = self.suffix(categories)?;
        let mut result = String::with_capacity(root.len() + suffix.len());

        result.push_str(root);
        result.push_str(suffix);
        Some(result)
    }
}

pub struct IrregularForm<'a, InflCatSet: InflectionalCategorySet>(InflCatSet, Option<&'a str>,);

pub struct Word<'a, Infl: Inflection<'a>> {
    inflection: Infl,
    lemma: &'a str,
    root: &'a str,
    regular: bool,
    irregular_forms: Vec<IrregularForm<'a, Infl::CategorySet>>,
}

impl<'a, Infl: Inflection<'a>> Word<'a, Infl> {
    pub fn inflect(&self, categories: Infl::CategorySet) -> Option<String> {
        if self.regular {
            self.inflection.inflect(self.root, categories)
        } else {
            for irregular_form in &self.irregular_forms {
                if irregular_form.0 == categories {
                    return Some(irregular_form.1?.to_string())
                }
            }
            self.inflection.inflect(self.root, categories)
        }
    }
}