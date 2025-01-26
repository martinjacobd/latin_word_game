use proc_macro::{self, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemEnum, ItemStruct, parse, Ident, Token};
use syn::parse::{Parser, Parse, ParseStream};

#[proc_macro_derive(InflectionalCategory)]
pub fn derive_inflectional_category(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    let name = &input.ident;
    let variants = input.variants;
    let variant_vec = variants.iter().map(|v| &v.ident);
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
pub fn derive_suffix_inflection(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = &input.ident;

    let underlying_type_attr = input
        .attrs
        .iter()
        .find(|a| a.path.is_ident("suffix_inflection_over"))
        .expect("suffix_inflection_over attribute specifying InflectionalCategorySet required to derive SuffixInflection");
    let underlying_type = underlying_type_attr.parse_args::<syn::Type>().expect("suffix_inflection_over attribute specifying InflectionalCategorySet required to derive SuffixInflection");

    let n_categories_attr = input
        .attrs
        .iter()
        .find(|a| a.path.is_ident("suffix_inflection_categories"))
        .expect("suffix_inflection_categories attribute specifying number of attribute categories required to derive SuffixInflection");
    let n_categories = n_categories_attr.parse_args::<syn::LitInt>().expect("suffix_inflection_categories attribute specifying number of attribute categories required to derive SuffixInflection");
    let n_categories = n_categories.base10_parse::<usize>().expect("suffix_inflection_categories attribute specifying number of attribute categories required to derive SuffixInflection");
    let n_categories = 0..n_categories;


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
pub fn suffix_inflection_over_categories(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as SuffixInflectionOverCategoriesInput);

    let category_set_name = &input.category_set_name;
    let suffix_inflection_struct_name = &input.suffix_inflection_struct_name;
    let categories = input.categories.iter();
    let categories_idents = input.categories.iter().map(|c| &c.ident);
    let categories_sizes = input.categories.iter().map(|c| c.variants.len());
    let mut categories_strides = input.categories.iter().map(|c| c.variants.len()).collect::<Vec<_>>();
    let i = 0..input.categories.len();
    categories_strides.remove(0);
    categories_strides.push(1);

    let gen = quote! {
        #(
            #[derive(PartialEq,InflectionalCategory)]
            #categories
        )*


        #[derive(InflectionalCategorySet)]
        pub struct #category_set_name (#(#categories_idents),*);

        pub struct #suffix_inflection_struct_name<'a> {
            name: &'a str,
            suffixes: [Option<&'a str>; 1 #(* #categories_sizes)*],
        }

        impl<'a> SuffixInflection<'a> for #suffix_inflection_struct_name<'a> {
            type CategorySet = #(input.category_set_name);

            fn suffix(self, categories: Self::CategorySet) -> Option<&'a str> {
                let index = categories.index();

                self.suffixes[0 #(+ index.#i * #categories_strides)*]
            }
        }
    };

    gen.into()
}