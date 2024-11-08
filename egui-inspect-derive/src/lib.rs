use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, Attribute, Data, DeriveInput, Expr, Lit, Member, Meta},
};

enum FieldInspectKind {
    /// Auto-inspected (it is assumed that the field implements Inspect)
    Auto,
    /// A function named by the token stream is called to inspect the field.
    /// The function takes (thing: &mut T, ui: &mut Ui, id_source: u64)
    WithFn(syn::Ident),
    /// Not visited, left alone.
    /// Useful when you want to skip a field that doesn't implement Inspect.
    Opaque,
}

fn inspect_kind(attrs: &[Attribute]) -> FieldInspectKind {
    for attr in attrs {
        if attr.path().is_ident("opaque") {
            return FieldInspectKind::Opaque;
        } else if attr.path().is_ident("inspect_with") {
            let fun: syn::Ident = attr.parse_args().expect("Failed to parse ident");
            return FieldInspectKind::WithFn(fun);
        }
    }
    FieldInspectKind::Auto
}

trait SynFieldExt {
    fn doc_comment_string(&self) -> String;
}

impl SynFieldExt for syn::Field {
    fn doc_comment_string(&self) -> String {
        let mut out = String::new();

        for attr in &self.attrs {
            if attr.path().is_ident("doc") {
                if let Meta::NameValue(meta_name_value) = &attr.meta {
                    if let Expr::Lit(syn::ExprLit {
                        lit: Lit::Str(s), ..
                    }) = &meta_name_value.value
                    {
                        out.push_str(&s.value());
                        out.push('\n');
                    }
                }
            }
        }

        out.trim_end().to_string()
    }
}

#[proc_macro_derive(Inspect, attributes(opaque, inspect_with))]
pub fn derive_inspect(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ty_ident = input.ident;
    let ts = match input.data {
        Data::Struct(s) => {
            let mut exprs = Vec::new();
            for (i, f) in s.fields.iter().enumerate() {
                let memb = match &f.ident {
                    Some(ident) => Member::from(ident.clone()),
                    None => Member::from(i),
                };
                let doc_comment_string = f.doc_comment_string();
                match inspect_kind(&f.attrs) {
                    FieldInspectKind::Auto => {
                        let ident = &f.ident;
                        exprs.push(quote! {
                            ui.horizontal(|ui| {
                                let mut re = ui.add(::egui_inspect::egui::Label::new(stringify!(#ident)).sense(::egui_inspect::egui::Sense::click()));
                                if !#doc_comment_string.is_empty()  {
                                    re = re.on_hover_text(#doc_comment_string);
                                }
                                if re.clicked() {
                                    ui.output_mut(|o| o.copied_text = format!("{:?}", self.#memb));
                                }
                                ::egui_inspect::Inspect::inspect_mut(&mut self.#memb, ui, #i as u64)
                            });
                        });
                    }
                    FieldInspectKind::Opaque => {
                        exprs.push(quote! {
                            ui.horizontal(|ui| {
                                let re = ui.label(concat!(stringify!(#memb), " <opaque>"));
                                if !#doc_comment_string.is_empty()  {
                                    re.on_hover_text(#doc_comment_string);
                                }
                            });
                        });
                    }
                    FieldInspectKind::WithFn(fun) => {
                        exprs.push(quote! {
                            ui.horizontal(|ui| {
                                let re = ui.label(stringify!(#memb));
                                if !#doc_comment_string.is_empty()  {
                                    re.on_hover_text(#doc_comment_string);
                                }
                                #fun(&mut self.#memb, ui, #i as u64)
                            });
                        });
                    }
                }
            }
            quote! {
                ::egui_inspect::egui::CollapsingHeader::new(stringify!(#ty_ident)).id_source(id_source).show(ui, |ui| {
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
                ::egui_inspect::egui::ComboBox::new(id_source, stringify!(#ty_ident)).selected_text(sel_text).show_ui(ui, |ui| {
                    #(#selectable_value_exprs;)*
                });
            }
        }
        Data::Union(_) => panic!("Unions are not supported"),
    };
    let (intro_generics, forward_generics, where_clauses) = input.generics.split_for_impl();
    let expanded = quote! {
        impl #intro_generics ::egui_inspect::Inspect for #ty_ident #forward_generics #where_clauses {
            fn inspect(&self, ui: &mut ::egui_inspect::egui::Ui, id_source: u64) {

            }
            fn inspect_mut(&mut self, ui: &mut ::egui_inspect::egui::Ui, id_source: u64) {
                #ts
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}
