// use syn::proc_macro::TokenStream;
extern crate jens;

use jens::File as JensFile;
use jens::{Block, LineSegment};
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

#[proc_macro_derive(Template, attributes(filename))]
pub fn derive_jens(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = &input.ident;
    let mut filename = None;
    // Parse out the #[filename = "filename"] attribute from the derive
    for attr in input.attrs {
        match attr.parse_meta().unwrap() {
            syn::Meta::NameValue(v) => {
                if v.ident.to_string() == "filename" {
                    if let syn::Lit::Str(s) = v.lit {
                        filename = Some(s.value());
                    }
                }
            }
            _ => {}
        }
    }

    let filename = filename
        .expect("Must provide a template file as an attribute: #[filename = \"file.jens\"] (relative to crate `/src` directory)");

    let path = get_path(&filename);
    let data = match load_file(&path) {
        Ok(data) => data,
        Err(error) => panic!("error opening {:?}: {}", &filename, error),
    };
    let file = JensFile::parse(&data).unwrap();

    let mut template_funcs = Vec::new();
    for t in file.templates.iter() {
        let func_ident = Ident::new(&t.name, Span::call_site());

        let placeholder_names = t.placeholder_names();
        let args = placeholder_names.iter().map(|p| {
            let placeholder_ident = Ident::new(&format!("placeholder_{}", p), Span::call_site());
            quote! {#placeholder_ident: impl Into<Block>}
        });
        let blocks = placeholder_names.iter().map(|p| {
            let placeholder_ident = Ident::new(&format!("placeholder_{}", p), Span::call_site());
            let block_ident = Ident::new(&format!("block_{}", p), Span::call_site());
            quote! {
                    let #block_ident: Block = #placeholder_ident.into();
            }
        });
        let block: Block = t.into();
        let lines: Vec<_> = block
            .0
            .iter()
            .map(|line| {
                let segments: Vec<_> = line
                    .0
                    .iter()
                    .map(|segment| match segment {
                        LineSegment::EndOfInput => {
                            quote! {jens::LineSegment::EndOfInput}
                        }
                        LineSegment::Content(c) => {
                            quote! {jens::LineSegment::Content(#c.into())}
                        }
                        LineSegment::Placeholder(c) => {
                            let ident = Ident::new(&format!("block_{}", c), Span::call_site());
                            quote! {jens::LineSegment::Block(#ident.clone())}
                        }
                        LineSegment::Block(_) => {
                            panic!("Unexpected block inside template");
                        }
                    })
                    .collect();
                quote! { jens::Line(vec![#(#segments),*]) }
            })
            .collect();
        template_funcs.push(quote! {
                pub fn #func_ident(#(#args),*) -> Block {
                        #(#blocks)*
                        jens::Block(vec![#(#lines),*])
                }
        })
    }

    let expanded = quote! {
        impl #struct_ident {
            #(#template_funcs)*
        };
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod test {
    use jens::{Block, Line, LineSegment};

    // Example of macro output for debugging
    struct SampleTemplate {}
    impl SampleTemplate {
        fn template1() -> Block {
            Block(vec![Line(vec![LineSegment::Content(
                "Simple template".into(),
            )])])
        }

        fn template2(placeholder1: impl Into<Block>, placeholder2: impl Into<Block>) -> Block {
            let b1: Block = placeholder1.into();
            let b2: Block = placeholder2.into();
            Block(vec![Line(vec![
                LineSegment::Content("Template with [".into()),
                LineSegment::Block(b1.clone()),
                LineSegment::Content("] and [".into()),
                LineSegment::Block(b2.clone()),
                LineSegment::Content("] placeholders but the second placeholder [".into()),
                LineSegment::Block(b2.clone()),
                LineSegment::Content("] appears twice".into()),
            ])])
        }
    }

    #[test]
    fn test_sample_template() {
        let t1 = SampleTemplate::template1();
        let t2 = SampleTemplate::template2(t1, "Hello");
        let output = format!("{}", t2);
        assert_eq!(
            output,
            "Template with [Simple template] and [Hello] placeholders but the second placeholder [Hello] appears twice"
        )
    }
}
