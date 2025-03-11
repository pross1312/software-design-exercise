extern crate proc_macro;
use proc_macro::*;

#[proc_macro_derive(EnumCount)]
pub fn enum_count(item: TokenStream) -> TokenStream {
    let mut iter = item.into_iter();
    let mut token  = iter.next().unwrap();
    let mut count = 0;
    while match &token {
        TokenTree::Ident(r#type) => r#type.to_string() != "enum",
        _ => true,
    } {
        if let Some(new_token) = iter.next() {
            token = new_token;
        } else {
            panic!("Could not find enum");
        }
    }
    if let TokenTree::Ident(r#type) = token {
        assert_eq!(r#type.to_string(), "enum");
        if let TokenTree::Ident(name) = iter.next().unwrap() {
            if let TokenTree::Group(group) = iter.next().unwrap() {
                let mut iter = group.stream().into_iter();
                for group_token in &mut iter {
                    if let TokenTree::Ident(_) = group_token {
                        count += 1;
                    }
                }
                return format!("pub const ENUM_{}_COUNT: usize = {};", name.to_string(), count).parse().unwrap();
            }
        }
    }
    panic!("???");
}

#[proc_macro]
pub fn enum_count_of(item: TokenStream) -> TokenStream {
    let mut iter = item.into_iter();
    let enum_name  = iter.next().unwrap();
    if let Some(_) = iter.next() {
        assert!(false, "Expected only enum name");
    }
    let result = format!("ENUM_{}_COUNT", enum_name.to_string());
    // println!("{result}");
    result.parse().unwrap()
}
