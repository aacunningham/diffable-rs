extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Diffable)]
pub fn diffable_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    proc_macro::TokenStream::from(impl_diffable(&ast))
}

fn impl_diffable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = get_fields(&ast.data);
    let diff_struct_name = syn::Ident::new(&format!("{}Diff", name), name.span());
    let diff_struct = generate_diff_struct(&diff_struct_name, &fields);
    let function_body = generate_function_body(&diff_struct_name, &fields);
    quote! {
        #[derive(Debug, PartialEq, Eq)]
        #diff_struct

        impl Diffable for #name {
            type DiffStruct = #diff_struct_name;

            fn diff(&self, right: &#name) -> Self::DiffStruct {
                #function_body
            }
        }
    }
}

fn generate_diff_struct(name: &syn::Ident, fields: &syn::Fields) -> TokenStream {
    let names = fields.iter().map(|f| &f.ident);
    let types = fields.iter().map(|f| {
        let ty = &f.ty;
        quote! {
            Option<(#ty, #ty)>
        }
    });
    let gen = quote! {
        struct #name {
            #( #names: #types, )*
        }
    };
    gen.into()
}

fn generate_function_body(diff_struct_name: &syn::Ident, fields: &syn::Fields) -> TokenStream {
    let variables = generate_variables(&fields);
    let eq_checks = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            if self.#name != right.#name { #name = Some((self.#name.clone(), right.#name.clone())); }
        }
    });
    let final_diff_struct = generate_final_diff_struct(&diff_struct_name, &fields);
    quote! {
        #variables
        #( #eq_checks )*
        #final_diff_struct
    }
}

fn generate_variables(fields: &syn::Fields) -> TokenStream {
    let names = fields.iter().map(|f| &f.ident);
    let types = fields.iter().map(|f| {
        let ty = &f.ty;
        quote! {
            Option<(#ty, #ty)>
        }
    });
    quote! {
        #( let mut #names: #types = None; )*
    }
}
fn generate_final_diff_struct(diff_struct_name: &syn::Ident, fields: &syn::Fields) -> TokenStream {
    let names = fields.iter().map(|f| &f.ident);
    quote! {
        #diff_struct_name { #( #names, )* }
    }
}

fn get_fields(data: &syn::Data) -> &syn::Fields {
    // This will probably fail, change to *data
    match data {
        syn::Data::Struct(ref d) => &d.fields,
        _ => panic!("We don't support anything other than structs"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
