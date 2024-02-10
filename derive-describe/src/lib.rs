use proc_macro::{self, TokenStream};
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, parse_macro_input, FieldsNamed, DataEnum, FieldsUnnamed};

#[proc_macro_derive(Describe)]
pub fn describe_it(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        ..
    } = parse_macro_input!(input);
    /* enum Data {
        Struct { Fields {Named, Unnamed, Unit}, },
        Enum,
         Union
     */
    let description = match data {
        Data::Struct(s) => match s.fields {
            /* Fields {
            Named(FieldsNamed),  // e.g x:i8, y:i32
            Unnammed(FieldsUnnamed), // Some(T)
            Unit, // None }
            */
            syn::Fields::Named(FieldsNamed { named, ..}) => {
                // Destructuring
                // TODO practice Pattern Matching
                let idents_types = named.iter().map(|f|{
                    format!("{}: {}",f.ident.as_ref().unwrap(), f.ty.to_token_stream()) });
                format!("A struct with Named fields\n\t {}", quote! {#(#idents_types),*})
            }
            syn::Fields::Unnamed(FieldsUnnamed { unnamed, ..}) => {
                 let types = unnamed.iter().map(|f|
                 format!("{}",f.ty.to_token_stream()) );
                 format!("A struct with Unnamed Fields\n\t {}", quote! {#(#types),*})
            }
            syn::Fields::Unit => format!("A unit struct"),
        },
        Data::Enum(DataEnum {
            variants, ..}) => {
            let vs = variants.iter().map(|v| &v.ident);
            format!("An Enum with this variants\n\t {}",quote! {#(#vs),*})
        }
        _ => "Union".to_string()
    };

    let output = quote!{
        impl #ident {
            fn describe() {
                println!("{} : {}.",stringify!(#ident), #description);
            }
        }
    };
    output.into()
}
