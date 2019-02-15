// use syn::proc_macro::TokenStream;
extern crate jens;

use jens::File as JensFile;
use quote::quote;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use syn::export::{Span, TokenStream};
use syn::{parse_macro_input, DeriveInput, Ident};

fn load_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
        let mut file = File::open(path.as_ref())?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        Ok(string)
}

fn get_path(path: &str) -> PathBuf {
        let root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
        let path = Path::new(&root).join("src/").join(&path);
        // let file_name = match path.file_name() {
        //         Some(file_name) => file_name,
        //         None => panic!("template attribute should point to a file"),
        // };
        path
}

#[proc_macro_derive(Jens, attributes(template))]
pub fn derive_jens(input: TokenStream) -> TokenStream {
        let input = parse_macro_input!(input as DeriveInput);
        let struct_ident = &input.ident;
        let mut filename = None;
        // Parse out the #[template = "filename"] attribute from the derive
        for attr in input.attrs {
                match attr.parse_meta().unwrap() {
                        syn::Meta::NameValue(v) => {
                                if v.ident.to_string() == "template" {
                                        if let syn::Lit::Str(s) = v.lit {
                                                filename = Some(s.value());
                                        }
                                }
                        }
                        _ => {}
                }
        }

        let filename = filename.expect(
                "Must provide a template file as an attribute: #[template = \"file.jens\"]",
        );

        let path = get_path(&filename);
        let data = match load_file(&path) {
                Ok(data) => data,
                Err(error) => panic!("error opening {:?}: {}", &filename, error),
        };
        let file = JensFile::parse(&data).unwrap();

        let mut template_funcs = Vec::new();
        let count = file.templates.len();
        for t in file.templates.iter() {
                let func_ident = Ident::new(&t.name, Span::call_site());
                let func_name = &t.name;
                let args: Vec<_> = t
                        .placeholder_names()
                        .iter()
                        .map(|p| {
                                let ident = Ident::new(p, Span::call_site());
                                quote! {#ident: Block}
                        })
                        .collect();
                template_funcs.push(quote! {
                        pub fn #func_ident(#(#args),*) {
                                println!("Hello from {}", #func_name);
                        }
                })
        }

        let expanded = quote! {
            impl #struct_ident {
                pub fn say_hello() {
                        let count = #count;
                        println!("{}", count)
                }
                #(#template_funcs)*
            };
        };

        TokenStream::from(expanded)
}
