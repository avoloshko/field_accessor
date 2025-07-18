use proc_macro::{self, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

#[proc_macro_derive(FieldAccessor)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let output = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let idents_enum: Vec<_> = named.iter().map(|f| &f.ident).collect();
                let idents_getenum = idents_enum.clone();
                let tys_enum: Vec<_> = named.iter().map(|f| &f.ty).collect();;
                let tys_for_structinfo = tys_enum.clone();
                let enumname = format_ident!("{}{}", ident, "FieldEnum");
                let enumfields = format_ident!("{}{}", ident, "Fields");
                let typeslist = format_ident!("{}{}", ident, "TypeList");
                let enumtypes = format_ident!("{}{}", ident, "Types");
                let field_count = tys_for_structinfo.len();
                let structinfo = format_ident!("{}{}", ident, "StructInfo");
                let gettersetter = format_ident!("{}{}", ident, "GetterSetter");

                let mut unique_types = vec![];
                for ty in named.iter().map(|f| &f.ty) {
                    if !unique_types.iter().any(|t| *t == ty) {
                        unique_types.push(ty);
                    }
                }

                let sanitize_type_for_variant = |ty: &&syn::Type| {
                    let ty_str = quote!(#ty).to_string();
                    let sanitized_body = ty_str
                        .replace(" ", "")
                        .replace("::", "_")
                        .replace("<", "Of")
                        .replace(">", "")
                        .replace(",", "And")
                        .replace("(", "")
                        .replace(")", "")
                        .replace("[", "ArrayOf")
                        .replace("]", "")
                        .replace(";", "Len")
                        .replace("->", "Returns")
                        .replace("&", "Ref")
                        .replace("'", "");
                    format_ident!("Type_{}", sanitized_body)
                };

                let enumtype_variants: Vec<_> = unique_types
                    .iter()
                    .map(sanitize_type_for_variant)
                    .collect();

                let typeslist_values: Vec<_> = tys_for_structinfo
                    .iter()
                    .map(|field_ty| {
                        let variant_ident = sanitize_type_for_variant(&field_ty);
                        quote! { #enumtypes::#variant_ident }
                    })
                    .collect();

                let mut get_quotes = vec![];
                let mut get_mut_quotes = vec![];
                let mut take_quotes = vec![];
                let mut replace_quotes = vec![];
                let mut set_quotes = vec![];
                let mut get_tys = vec![];
                let mut get_mut_tys = vec![];
                let mut take_tys = vec![];
                let mut replace_tys = vec![];
                let mut swap_tys = vec![];
                let mut set_tys = vec![];

                let field_idents = named.iter().map(|f| &f.ident);

                let mut swap_ident = vec![];
                let mut swap_ident2 = vec![];
                for (outer_ident, outer_type) in named
                    .iter()
                    .map(|f| &f.ident)
                    .zip(named.iter().map(|f| &f.ty))
                {
                    for (inner_ident, inner_type) in named
                        .iter()
                        .map(|f| &f.ident)
                        .zip(named.iter().map(|f| &f.ty))
                    {
                        if outer_type == inner_type {
                            if outer_ident != inner_ident {
                                swap_tys.push(inner_type);
                                swap_ident.push(outer_ident.clone());
                                swap_ident2.push(inner_ident.clone());
                            }
                        }
                    }
                }

                for name in named.clone().iter() {
                    if !get_tys.contains(&name.ty) {
                        get_tys.push(name.ty.clone());
                        get_mut_tys.push(name.ty.clone());
                        take_tys.push(name.ty.clone());
                        replace_tys.push(name.ty.clone());
                        set_tys.push(name.ty.clone());

                        let get_filtered_ident =
                            named.iter().filter(|x| x.ty == name.ty).map(|f| &f.ident);
                        let get_mut_filtered_ident = get_filtered_ident.clone();
                        let take_filtered_ident = get_filtered_ident.clone();
                        let replace_filtered_ident = get_filtered_ident.clone();
                        let set_filtered_ident = get_filtered_ident.clone();

                        get_quotes.push(quote! {
                            #(
                                stringify!(#get_filtered_ident) => {
                                    Ok(&self.#get_filtered_ident)
                                }
                            ),*
                        });
                        get_mut_quotes.push(quote! {
                            #(
                                stringify!(#get_mut_filtered_ident) => {
                                    Ok(&mut self.#get_mut_filtered_ident)
                                }
                            ),*
                        });
                        take_quotes.push(quote! {
                            #(
                                stringify!(#take_filtered_ident) => {
                                    Ok(std::mem::take(&mut self.#take_filtered_ident))
                                }
                            ),*
                        });
                        replace_quotes.push(quote! {
                            #(
                                stringify!(#replace_filtered_ident) => {
                                    Ok(std::mem::replace(&mut self.#replace_filtered_ident, src))
                                }
                            ),*
                        });

                        set_quotes.push(quote! {
                            #(
                                stringify!(#set_filtered_ident) => {
                                    {self.#set_filtered_ident = value; Ok(())}
                                }
                            ),*
                        });
                    }
                }
                quote! {

                    #[derive(Debug, Clone)]
                    struct #structinfo {
                        field_names: Vec<String>,
                        field_types: Vec<String>,
                        struct_name: String
                    }

                    #[derive(Debug, PartialEq, PartialOrd, Clone)]
                    #[allow(non_camel_case_types)]
                    enum #enumname{
                        #(#idents_enum(#tys_enum)),*
                    }

                    #[derive(Debug, Clone)]
                    pub enum #enumtypes {
                        #(#enumtype_variants),*
                    }

                    #[derive(EnumString, AsRefStr, EnumIter, PartialOrd, Ord, Hash, Clone, Eq, PartialEq, Debug)]
                    #[allow(non_camel_case_types)]
                    pub enum #enumfields{
                        #(#idents_enum),*
                    }

                    pub const #typeslist: [#enumtypes; #field_count] = [
                        #(#typeslist_values),*
                    ];

                    trait #gettersetter<T> {
                        fn get(&self, field_string: &str) -> Result<&T, String>;
                        fn get_mut(&mut self, field_string: &str) -> Result<&mut T, String>;
                        fn take(&mut self, field_string: &str) -> Result<T, String>;
                        fn replace(&mut self, field_string: &str, src: T) -> Result<T, String>;
                        fn set(&mut self, field_string: &str, value: T) -> Result<(), String>;
                    }

                    #(
                        impl #gettersetter<#set_tys> for #ident {
                            fn get(&self, field_string: &str) -> Result<&#get_tys, String> {
                                match field_string {
                                    #get_quotes,
                                    _ => Err(format!("invalid field name to get '{}'", field_string)),
                                }
                            }
                            fn get_mut(&mut self, field_string: &str) -> Result<&mut #get_tys, String> {
                                match field_string {
                                    #get_mut_quotes,
                                    _ => Err(format!("invalid field name to get_mut '{}'", field_string)),
                                }
                            }
                            fn take(&mut self, field_string: &str) -> Result<#take_tys, String> {
                                match field_string {
                                    #take_quotes,
                                    _ => Err(format!("invalid field name to take '{}'", field_string)),
                                }
                            }
                            fn replace(&mut self, field_string: &str, src: #replace_tys) -> Result<#replace_tys, String> {
                                match field_string {
                                    #replace_quotes,
                                    _ => Err(format!("invalid field name to replace '{}'", field_string)),
                                }
                            }

                            fn set(&mut self, field_string: &str, value: #set_tys) -> Result<(), String>{
                                match field_string {
                                    #set_quotes,
                                    _ => Err(format!("invalid field name to set '{}'", field_string)),
                                }
                            }
                        }
                    )*

                    impl #ident {

                        fn swap(&mut self, field_string: &str, field_string_y: &str) -> Result<(), String> {
                            match (field_string, field_string_y) {
                                #(
                                    (stringify!(#swap_ident), stringify!(#swap_ident2)) => {
                                        std::mem::swap::<#swap_tys>(&mut self.#swap_ident, &mut self.#swap_ident2);
                                        Ok(())
                                    }
                                ),*
                                _ => Err(format!("invalid field name to swap")),
                            }
                        }

                        fn getenum(&self, field_string: &str) -> Result<#enumname, String> {
                            match field_string {
                                #(stringify!(#idents_getenum) => {
                                    Ok(#enumname::#idents_getenum(self.#idents_getenum.clone()))
                                }),*
                                _ => Err(format!("invalid field name to getenum '{}'", field_string)),
                            }
                        }

                        fn getstructinfo(&self) -> #structinfo {
                            #structinfo {
                                field_names: vec![#(stringify!(#field_idents).to_string()),*],
                                field_types: vec![#(stringify!(#tys_for_structinfo).to_string()),*],
                                struct_name: stringify!(#ident).to_string()}
                        }
                    }
                }
            }
            syn::Fields::Unnamed(_) => panic!("Only NamedFields is supported"),
            syn::Fields::Unit => panic!("Only NamedFields is supported"),
        },
        syn::Data::Enum(_) => panic!("Enum is not supported. Only struct is supported"),
        syn::Data::Union(_) => panic!("Union is not supported. Only struct is supported"),
    };
    output.into()
}
