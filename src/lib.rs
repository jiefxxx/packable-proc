use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields };


#[proc_macro_derive(packable_derive)]
pub fn derive_packable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let sum = size_sum(&input.data);
    let pack = pack_sum(&input.data);
    let unpack = unpack_sum(&input.data);

    let expanded = quote! {

        impl Packable for #name  {
            fn pack(&self, litle_endian: bool) -> Vec<u8>{
                #pack
            }
            fn size(&self) -> usize {
                #sum
            }

            fn unpack(&mut self, data: &mut Vec<u8>, litle_endian: bool) -> Result<(), PackableError>{
                #unpack
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn size_sum(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            Packable::size(&self.#name)
                        }
                    });
                    quote! {
                        0 #(+ #recurse)*
                    }
                }
                Fields::Unnamed(ref _fields) => {
                    quote!(0)
                }
                Fields::Unit => {
                    quote!(0)
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn pack_sum(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            self.#name
                        }
                    });
                    quote! {
                        pack!(litle_endian #(, #recurse)*)
                    }
                }
                Fields::Unnamed(ref _fields) => {
                    quote!(0)
                }
                Fields::Unit => {
                    quote!(0)
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn unpack_sum(in_data: &Data) -> TokenStream {
    match *in_data {
        Data::Struct(ref in_data) => {
            match in_data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            self.#name
                        }
                    });
                    quote! {
                        unpack!(litle_endian, data #(, #recurse)*)?;
                        Ok(())
                    }
                }
                Fields::Unnamed(ref _fields) => {
                    quote!(0)
                }
                Fields::Unit => {
                    quote!(0)
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
