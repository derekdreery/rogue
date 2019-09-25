extern crate proc_macro;

use crate::glyph::Color;
use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenStream as TokStr2};
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    token, Data, DataEnum, DeriveInput, Error, Ident, LitChar, LitStr, Result, Token, Variant,
};

mod glyph;

mod kw {
    syn::custom_keyword!(char);
    syn::custom_keyword!(fg_color);
    syn::custom_keyword!(bg_color);
}

#[derive(Debug)]
enum TileAttr {
    Char {
        keyword: kw::char,
        equals: Token![=],
        lit_char: LitChar,
    },
    Default(Token![default]),
    FgColor {
        keyword: kw::fg_color,
        equals: Token![=],
        lit_str: LitStr,
    },
    BgColor {
        keyword: kw::bg_color,
        equals: Token![=],
        lit_str: LitStr,
    },
}

impl Parse for TileAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![default]) {
            input.parse().map(TileAttr::Default)
        } else if lookahead.peek(kw::char) {
            let keyword = input.parse()?;
            let equals = input.parse()?;
            let lit_char = input.parse()?;
            Ok(TileAttr::Char {
                keyword,
                equals,
                lit_char,
            })
        } else if lookahead.peek(kw::fg_color) {
            let keyword = input.parse()?;
            let equals = input.parse()?;
            let lit_str = input.parse()?;
            Ok(TileAttr::FgColor {
                keyword,
                equals,
                lit_str,
            })
        } else if lookahead.peek(kw::bg_color) {
            let keyword = input.parse()?;
            let equals = input.parse()?;
            let lit_str = input.parse()?;
            Ok(TileAttr::BgColor {
                keyword,
                equals,
                lit_str,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Debug)]
struct TileAttrs {
    paren_token: token::Paren,
    attrs: Punctuated<TileAttr, Token![,]>,
}

impl Parse for TileAttrs {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(TileAttrs {
            paren_token: parenthesized!(content in input),
            attrs: content.parse_terminated(TileAttr::parse)?,
        })
    }
}

#[derive(Debug)]
struct TileInfo {
    ident: Ident,
    character: char,
    fg_color: Color,
    bg_color: Color,
}

#[derive(Debug)]
struct TileSetInfo {
    default: Ident,
    tile_info: Vec<TileInfo>,
}

#[proc_macro_derive(TileSet, attributes(tileset))]
pub fn derive_tileset(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = match real_derive_tileset(input) {
        Ok(t) => t.into(),
        Err(e) => e.to_compile_error().into(),
    };
    output
}

fn real_derive_tileset(input: DeriveInput) -> Result<TokStr2> {
    use crate::glyph::{Font, Glyph};

    let font = Font::new();
    let enum_ident = &input.ident;
    let data = match &input.data {
        Data::Enum(ref data) => data,
        _ => return Err(Error::new(input.span(), "expected enum")),
    };
    let tile_info = get_all_tile_info(&data, &input)?;
    let default_variant = &tile_info.default;
    //println!("{:#?}", tile_info.tile_info);
    let mut characters = Vec::new();
    let (bmp_list, idx_list): (Vec<_>, Vec<_>) = tile_info
        .tile_info
        .iter()
        .enumerate()
        .map(
            |(
                idx,
                TileInfo {
                    ident,
                    character,
                    fg_color,
                    bg_color,
                },
            )| {
                let Glyph {
                    width,
                    height,
                    data,
                } = font.glyph(*character, *fg_color, *bg_color);
                let width = width as u16;
                let height = height as u16;
                let ch = Literal::character(*character);
                characters.push(quote! {
                    #enum_ident :: #ident => #ch
                });
                let data = Literal::byte_string(&data);
                (
                    quote! {
                        tiler::RgbaImage::new(#width, #height, #data)
                    },
                    quote! {
                        #enum_ident :: #ident => #idx
                    },
                )
            },
        )
        .unzip();
    Ok(quote! {
        impl std::default::Default for #enum_ident {
            fn default() -> Self {
                #enum_ident :: #default_variant
            }
        }

        impl tiler::TileSet for #enum_ident {
            fn get_bmps() -> &'static [tiler::RgbaImage<'static>] {
                const BMPS: &'static [tiler::RgbaImage<'static>] = &[
                    #(#bmp_list),*
                ];
                BMPS
            }

            fn as_char(&self) -> char {
                match self {
                    #(#characters),*
                }
            }

            fn idx(&self) -> usize {
                match self {
                    #(#idx_list),*
                }
            }
        }
    })
}

/// Go through all the variants of the enum and work out the associated macro information (like
/// which one is the default, etc.)
fn get_all_tile_info(data: &DataEnum, input: &DeriveInput) -> Result<TileSetInfo> {
    let mut default: Option<Ident> = None;
    let mut tile_info = Vec::new();
    for variant in data.variants.iter() {
        let mut character: Option<char> = None;
        let mut fg_color = Color::WHITE;
        let mut bg_color = Color::BLACK;
        for attr in get_tile_attrs(&variant)? {
            match attr {
                TileAttr::Char {
                    keyword,
                    equals,
                    lit_char,
                } => match character {
                    Some(ch) => {
                        return Err(Error::new(
                            keyword.span(),
                            "there must only be a single `char` attribute",
                        ))
                    }
                    None => character = Some(lit_char.value()),
                },
                TileAttr::Default(default_tok) => match default {
                    None => default = Some(variant.ident.clone()),
                    Some(_) => {
                        return Err(Error::new(
                            default_tok.span(),
                            "multiple variants marked as `default`",
                        ))
                    }
                },
                TileAttr::FgColor {
                    keyword,
                    equals,
                    lit_str,
                } => match Color::parse(&lit_str.value()) {
                    Ok(color) => fg_color = color,
                    Err(msg) => return Err(Error::new(lit_str.span(), msg)),
                },
                TileAttr::BgColor {
                    keyword,
                    equals,
                    lit_str,
                } => match Color::parse(&lit_str.value()) {
                    Ok(color) => bg_color = color,
                    Err(msg) => return Err(Error::new(lit_str.span(), msg)),
                },
            };
        }
        let character = match character {
            Some(ch) => ch,
            None => return Err(Error::new(variant.span(), "no `char` set")),
        };
        tile_info.push(TileInfo {
            ident: variant.ident.clone(),
            character,
            fg_color,
            bg_color,
        });
    }
    let default = match default {
        Some(ident) => ident,
        None => {
            return Err(Error::new(
                input.span(),
                "no variant was declared `default`",
            ))
        }
    };
    return Ok(TileSetInfo { default, tile_info });
}

/// Get all the parsed attributes from the variant
fn get_tile_attrs(variant: &Variant) -> Result<Vec<TileAttr>> {
    // We re-collect in our own vec so we can flatten multiple copies of `#[tileset(..)]`
    let mut attrs = Vec::new();
    for attr in variant
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("tileset"))
    {
        let tile_attrs: TileAttrs = syn::parse2(attr.tokens.clone())?;
        attrs.extend(tile_attrs.attrs);
    }
    Ok(attrs)
}
