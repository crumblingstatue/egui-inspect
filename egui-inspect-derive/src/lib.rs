use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Inspect)]
pub fn derive_inspect(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = match input.data {
        Data::Struct(s) => {
            let mut exprs = Vec::new();
            for (i, f) in s.fields.iter().enumerate() {
                let name = f.ident.as_ref().unwrap();
                exprs.push(quote! {
                    ui.horizontal(|ui| {
                        ui.label(stringify!(#name));
                        self.#name.inspect(ui, #i as u64)
                    });
                });
            }
            let name = input.ident;
            quote! {
                impl Inspect for #name {
                    fn inspect(&mut self, ui: &mut egui::Ui, id_source: u64) {
                        egui::CollapsingHeader::new(stringify!(#name)).id_source(id_source).show(ui, |ui| {
                            #(#exprs)*
                        });
                    }
                }
            }
        }
        _ => panic!("Unsupported"),
    };
    proc_macro::TokenStream::from(expanded)
}
