use proc_macro::{self, TokenStream};
use quote::{quote};
use syn::{parse_macro_input, ItemEnum, ItemStruct, Ident};
use syn::parse::{Parse, ParseStream};
use iter_tools;

#[proc_macro_derive(InflectionalCategory)]
/// Derive an `InflectionalCategory` in the straightforward case that it is an `ItemEnum` of
/// several variants.
pub fn derive_inflectional_category(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    let name = &input.ident;
    let variants = input.variants;
    let variant_vec = variants
        .iter()
        .map(|v| &v.ident);
    let i = 0usize..variants.len();

    let gen = quote! {
        impl InflectionalCategory for #name {
            fn index(self) -> usize {
                match self {
                    #(#name::#variant_vec => #i),*
                }
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(InflectionalCategorySet)]
/// Derive an `InflectionalCategorySet` in the straightforward case that it is an `ItemStruct` of
/// several `InflectionalCategory`s.
///
/// Only use this if for some reason you cannot use `suffix_inflection_over_categories!`.
/// `suffix_inflection_over_categories` will implement the index of `InflectionalCategorySet`
/// as a single `usize`. This will implement it as a tuple of `usize`s for use with a later
/// `derive_suffix_inflection`.
pub fn derive_inflectional_category_set(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = &input.ident;
    let fields = input.fields;
    let field_vec = fields.iter().map(|f| &f.ident);
    let index_type = fields.iter().map(|_f| quote!{ usize });

    let gen = quote! {
        impl InflectionalCategorySet for #name {
            type IndexType = (#(#index_type),*);
            fn index(self) -> Self::IndexType {
                (#(self.#field_vec.index()),*)
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(SuffixInflection, attributes(suffix_inflection_over, suffix_inflection_categories))]
/// Derive a `SuffixInflection` in the straightforward case that it has a `suffixes` member that is
/// a multidimensional array indexed by a tuple type generated from a derivation of
/// `InflectionalCategorySet`.
///
/// Only use this if for some reason you cannot use `suffix_inflection_over_categories!`.
/// `suffix_inflection_over_categories` will implement the suffixes of `SuffixInflection` as a
/// single, linear array. This will implement it as a multidimensional array.
pub fn derive_suffix_inflection(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = &input.ident;

    let underlying_type_attr = input
        .attrs
        .iter()
        .find(|a| a.path.is_ident("suffix_inflection_over"))
        .expect("suffix_inflection_over attribute specifying InflectionalCategorySet required to derive SuffixInflection");
    let underlying_type = underlying_type_attr
        .parse_args::<syn::Type>()
        .expect("suffix_inflection_over attribute specifying InflectionalCategorySet required to derive SuffixInflection");

    let n_categories_attr = input
        .attrs
        .iter()
        .find(|a| a.path.is_ident("suffix_inflection_categories"))
        .expect("suffix_inflection_categories attribute specifying number of attribute categories required to derive SuffixInflection");
    let n_categories = n_categories_attr
        .parse_args::<syn::LitInt>()
        .expect("suffix_inflection_categories attribute specifying number of attribute categories required to derive SuffixInflection");
    let n_categories = n_categories
        .base10_parse::<usize>()
        .expect("suffix_inflection_categories attribute specifying number of attribute categories required to derive SuffixInflection");
    let n_categories = (0..n_categories).map(syn::Index::from);


    let gen = quote! {
        impl<'a> SuffixInflection<'a> for #name<'a> {
            type CategorySet = #underlying_type;

            fn suffix(self, categories: Self::CategorySet) -> Option<&'a str> {
                let index = categories.index();

                self.suffixes #([index.#n_categories])*
            }
        }
    };

    gen.into()
}

struct SuffixInflectionOverCategoriesInput {
    suffix_inflection_struct_name: Ident,
    category_set_name: Ident,
    categories: Vec<ItemEnum>,
}

impl Parse for SuffixInflectionOverCategoriesInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut res = SuffixInflectionOverCategoriesInput {
            suffix_inflection_struct_name : input.parse()?,
            category_set_name : input.parse()?,
            categories : Vec::new(),
        };

        while let Ok(item) = input.parse::<ItemEnum>() {
            res.categories.push(item);
        }

        Ok(res)
    }
}

#[proc_macro]
/// Derive an `InflectionalCategorySet` and `SuffixInflection` for several `InflectionalCategory`s
/// of a type acceptable to `derive_inflectional_category`. Usage:
/// ```compile_fail
///  suffix_inflection_over_categories! {
///     SuffixInflectionName
///     InflectionalCategorySetName
///
///     pub enum CategoryOne {
///         CategoryOneVariantOne,
///         CategoryOneVariantTwo,
///         /* ... */
///     }
///
///     pub enum CategoryTwo {
///         CategoryTwoVariantOne,
///         CategoryTwoVariantTwo,
///         /* ... */
///     }
///
///     /* ... */
///  }
/// ```
pub fn suffix_inflection_over_categories(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SuffixInflectionOverCategoriesInput);

    let category_set_name = &input.category_set_name;
    let suffix_inflection_struct_name = &input.suffix_inflection_struct_name;
    let categories = input
        .categories
        .iter()
        .collect::<Vec<_>>();
    let categories_idents = input
        .categories
        .iter()
        .map(|c| &c.ident)
        .collect::<Vec<_>>();
    let mut categories_sizes: Vec<usize> = input
        .categories
        .iter()
        .map(|c| c.variants.len())
        .collect();
    categories_sizes.reverse();
    let mut categories_strides: Vec<usize> = categories_sizes
        .iter()
        .fold(vec![1], |mut acc, &s| {acc.push(s * acc.last().unwrap()); acc});
    let total_n_elements = categories_strides.pop().unwrap();
    categories_strides.reverse();
    let i = (0..input.categories.len()).map(syn::Index::from);

    let categories_variants = input
        .categories
        .iter()
        .map(|c| c.variants.iter())
        .map(|v| v.map(|v| &v.ident).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let categories_tuples =
        iter_tools::Itertools::multi_cartesian_product(categories_variants.into_iter())
        .map(|v| { quote! { (#(#categories_idents::#v),*) } });

    let indices = (0..categories.len())
        .map(syn::Index::from)
        .into_iter()
        .collect::<Vec<_>>();
    let formats = (0..categories.len())
        .map(|_| quote! { "{:20} " })
        .into_iter()
        .collect::<Vec<_>>();

    let gen = quote! {
        #(
            #[derive(Clone,Copy,Debug,PartialEq,InflectionalCategory)]
            #categories
        )*

        #[derive(Clone,Copy,Debug,PartialEq)]
        pub struct #category_set_name (#(#categories_idents),*);

        impl InflectionalCategorySet for #category_set_name {
            type IndexType = usize;

            fn index(self) -> Self::IndexType {
                0 #(+ self.#i.index() * #categories_strides)*
            }
        }

        #[derive(Debug)]
        pub struct #suffix_inflection_struct_name<'a> {
            name: &'a str,
            suffixes: [Option<&'a str>; #total_n_elements],
        }

        impl<'a> SuffixInflection<'a> for #suffix_inflection_struct_name<'a> {
            type CategorySet = #category_set_name;

            fn suffix(self, categories: Self::CategorySet) -> Option<&'a str> {
                let index = categories.index();

                self.suffixes[index]
            }
        }

        impl<'a> ::std::fmt::Display for #suffix_inflection_struct_name<'a> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}\n====\n", self.name);
                let categories_set = vec![#(#category_set_name #categories_tuples),*];
                for categories in categories_set {
                    if let Some(suffix) = self.suffixes[categories.index()] {
                        write!(f,
                            concat!(#(#formats),* , "{:?}\n"),
                            #(format!("{:?}",categories.#indices)),* ,
                            suffix);
                    }
                }

                write!(f, "\n====\n");

                Ok(())
            }
        }
    };

    gen.into()
}

struct Suffixes(Vec<Option<String>>);

impl Parse for Suffixes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut res = Vec::new();

        while !input.is_empty() {
            if let Ok(s) = input.parse::<syn::LitStr>() {
                res.push(Some(s.value()));
            } else if input.parse::<syn::token::Comma>().is_ok() {
                continue;
            } else if let Ok(ident) = input.parse::<Ident>() {
                if ident == "None" || ident == "N" {
                    res.push(None);
                } else {
                    res.push(Some(ident.to_string()));
                }
            }
        }

        Ok(Suffixes(res))
    }
}
#[proc_macro]
/// This will generate a vector suffixes of the type Option<&'a str> with less line noise in the
/// input than normal.
/// Example:
/// ```no_compile
/// const suffixes: [Option<&'static str>; 6] = suffixes! [
///     "a" "ae" ae "am" a N
/// ];
/// ```
/// N or None represents a None. All other identifiers are turned into strings, as in the second
/// ae above. You may also use string literals (probably the preferable route).
pub fn suffixes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Suffixes);
    let suffixes = input.0;
    let suffixes = suffixes
        .into_iter()
        .map(|s| {
            if let Some(s) = s {
                quote! { Some(#s) }
            } else {
                quote! { None }
            }
        });
    let gen = quote! {
        [#(#suffixes),*]
    };
    gen.into()
}
