pub trait InflectionalCategory { }

pub trait InflectionalCategorySet { }

pub trait Inflection {
    type CategorySet: InflectionalCategorySet;

    fn inflect(self, root: &str, categories: Self::CategorySet) -> String;
}