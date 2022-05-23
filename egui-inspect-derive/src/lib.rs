use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput};

enum FieldInspectKind {
    /// Auto-inspected (it is assumed that the field implements Inspect)
    Auto,
    /// A function named by the token stream is called to inspect the field.
    /// The function takes (thing: &mut T, ui: &mut Ui, id_source: u64)
    WithFn(TokenStream),
    /// Not visited, left alone.
    /// Useful when you want to skip a field that doesn't implement Inspect.
    Opaque,
}

fn inspect_kind(attrs: &[Attribute]) -> FieldInspectKind {
    for attr in attrs {
        for seg in &attr.path.segments {
            if seg.ident == "opaque" {
                return FieldInspectKind::Opaque;
            } else if seg.ident == "inspect_with" {
                return FieldInspectKind::WithFn(attr.tokens.clone().into());
            }
        }
    }
    FieldInspectKind::Auto
}

#[proc_macro_derive(Inspect, attributes(opaque, inspect_with))]
pub fn derive_inspect(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ty_ident = input.ident;
    let ts = match input.data {
        Data::Struct(s) => {
            let mut exprs = Vec::new();
            for (i, f) in s.fields.iter().enumerate() {
                let name = f.ident.as_ref().unwrap();
                match inspect_kind(&f.attrs) {
                    FieldInspectKind::Auto => {
                        exprs.push(quote! {
                            ui.horizontal(|ui| {
                                if ui.add(egui::Label::new(stringify!(#name)).sense(egui::Sense::click())).clicked() {
                                    ui.output().copied_text = format!("{:?}", self.#name);
                                }
                                egui_inspect::Inspect::inspect_mut(&mut self.#name, ui, #i as u64)
                            });
                        });
                    }
                    FieldInspectKind::Opaque => {
                        exprs.push(quote! {
                            ui.horizontal(|ui| {
                                ui.label(concat!(stringify!(#name), " <opaque>"));
                            });
                        });
                    }
                    FieldInspectKind::WithFn(ts) => {
                        let ts: proc_macro2::TokenStream = ts.into();
                        exprs.push(quote! {
                            ui.horizontal(|ui| {
                                ui.label(stringify!(#name));
                                #ts(&mut self.#name, ui, #i as u64)
                            });
                        });
                    }
                }
            }
            quote! {
                egui::CollapsingHeader::new(stringify!(#ty_ident)).id_source(id_source).show(ui, |ui| {
                    #(#exprs)*
                });
            }
        }
        Data::Enum(e) => {
            let mut sel_name_match_exprs = Vec::new();
            let mut selectable_value_exprs = Vec::new();
            for var in &e.variants {
                let name = &var.ident;
                sel_name_match_exprs.push(quote! {Self::#name => stringify!(#name)});
                selectable_value_exprs
                    .push(quote! {ui.selectable_value(self, Self::#name, stringify!(#name))});
            }
            quote! {
                let sel_text = match self {
                    #(#sel_name_match_exprs,)*
                };
                egui::ComboBox::from_label(stringify!(#ty_ident)).selected_text(sel_text).show_ui(ui, |ui| {
                    #(#selectable_value_exprs;)*
                });
            }
        }
        Data::Union(_) => panic!("Unions are not supported"),
    };
    let expanded = quote! {
        impl ::egui_inspect::Inspect for #ty_ident {
            fn inspect(&self, ui: &mut egui::Ui, id_source: u64) {

            }
            fn inspect_mut(&mut self, ui: &mut egui::Ui, id_source: u64) {
                #ts
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}
