mod latin;

pub trait InflectionalCategory {
    fn index(self) -> usize;
}

pub trait InflectionalCategorySet {
    type IndexType;
    fn index(self) -> Self::IndexType;
}

pub trait Inflection<'a> {
    type CategorySet: InflectionalCategorySet;

    fn inflect(self, root: &'a str, categories: Self::CategorySet) -> Option<String>;
}

pub trait SuffixInflection<'a> {
    type CategorySet: InflectionalCategorySet;

    fn suffix (self, categories: Self::CategorySet) -> Option<&'a str>;
}

impl<'a, T> Inflection<'a> for T where T: SuffixInflection<'a> {
    type CategorySet = T::CategorySet;

    fn inflect(self, root: &'a str, categories: T::CategorySet) -> Option<String> {
        let suffix = self.suffix(categories)?;
        let mut result = String::with_capacity(root.len() + suffix.len());

        result.push_str(root);
        result.push_str(suffix);
        Some(result)
    }
}