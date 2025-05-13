use core::convert::Into;
use core::iter::Iterator;

use proc_macro2::{Delimiter, Ident, Literal};
use proc_macro2::{Span, TokenTree};
use quote::quote;

macro_rules! err {
    ($msg:expr) => {
        return format!("compile_error!(\"{}\");", $msg).parse().unwrap()
    };
}

macro_rules! unwrap {
    ($args:expr, $t:ident) => {{
        let Some(nx) = $args.next() else { err!(format!("Unexpected end of token stream, wanted {}", stringify!($t))) };
        let TokenTree:: $t  (val) = nx else { err!(format!("Expected '{}' found '{nx}'", stringify!($t))) };
        val
    }};
}

macro_rules! try_item {
    ($args:expr, $t:ident) => {{
        if let Some(nx) = $args.peek() {
            if let TokenTree:: $t (_) = nx {
                Some(unwrap!($args, $t))
            }
            else { None }
        } else { None }
    }};
}

macro_rules! try_punct {
    ($args:expr, $p:literal) => {{
        if let Some(TokenTree::Punct(p)) = $args.peek() {
            if p.as_char() == $p {
                $args.next();
                true
            } else { false }
        } else {
            false
        }
    }};
}

macro_rules! expect_punct {
    ($args:expr, $p:literal) => {{
        let c = unwrap!($args, Punct).as_char();
        if c != $p {
            err!(format!("Expected punctuation to be '{}' but found '{c}'", $p));
        }
    }};
}

macro_rules! expect_delimited {
    ($args:expr, $d:ident) => {{
        let it = unwrap!($args, Group);
        if !matches!(it.delimiter(), Delimiter :: $d) {
            err!(format!("Expected delimiter {}", stringify!($d)));
        }
        it.stream()
    }};
}

macro_rules! try_delimited {
    ($args:expr, $d:ident) => {{
        let it = try_item!($args, Group);

        if it.as_ref().is_some_and(|i| matches!(i.delimiter(), Delimiter :: $d)) {
            Some(it.unwrap().stream())
        } else {
            None
        }
    }};
}

macro_rules! expect_ident {
    ($args:expr, $p:literal) => {{
        let c = unwrap!($args, Ident).to_string();
        if c != $p {
            err!(format!("Expected Ident to be '{}' but found '{c}'", $p));
        }
    }};
}

fn make_field_multi_byte(ty: &Ident, name: Ident, start: Literal, end: Literal, mutable: bool) -> proc_macro2::TokenStream {
    let get_ident = proc_macro2::Ident::new(&format!("get_{name}"), Span::call_site());
    let mut r = quote! {
        #[inline]
        pub fn #get_ident (&self) -> #ty {
            bitfi::BitField::get_bit_range(self, #start..=#end)
        }
    };

    if mutable {
        let set_ident = proc_macro2::Ident::new(&format!("set_{name}"), Span::call_site());
        r = quote! {
            #r

            #[inline]
            pub fn #set_ident (&mut self, val: #ty) {
                bitfi::BitField::set_bit_range(self, #start..=#end, val)
            }
        }
    }

    r
}

fn make_field_single_byte(name: Ident, index: Literal, mutable: bool) -> proc_macro2::TokenStream {
    let get_ident = proc_macro2::Ident::new(&format!("get_{name}"), Span::call_site());
    let mut r = quote! {
        #[inline]
        pub fn #get_ident (&self) -> bool {
            bitfi::BitField::get_bit(self, #index)
        }
    };

    if mutable {
        let set_ident = proc_macro2::Ident::new(&format!("set_{name}"), Span::call_site());
        let clear_ident = proc_macro2::Ident::new(&format!("clear_{name}"), Span::call_site());
        r = quote! {
            #r

            #[inline]
            pub fn #set_ident (&mut self) {
                bitfi::BitField::set_bit(self, #index)
            }

            #[inline]
            pub fn #clear_ident (&mut self) {
                bitfi::BitField::clear_bit(self, #index)
            }
        }
    }

    r
}

fn parse_bitfield(ts: &mut impl Iterator<Item = TokenTree>) -> proc_macro2::TokenStream {
    let name = unwrap!(ts, Ident);

    expect_punct!(ts, '=');

    let ty = unwrap!(ts, Ident);

    let ts = expect_delimited!(ts, Brace);
    let mut ts = ts.into_iter().peekable();

    let mut fields = vec![];

    while ts.peek().is_some() {
        let name = unwrap!(ts, Ident);

        expect_punct!(ts, ':');

        let start = unwrap!(ts, Literal);

        #[allow(clippy::collapsible_match)]
        let end = if try_punct!(ts, '-') {
            Some(unwrap!(ts, Literal))
        } else {
            None
        };

        let mut mutable = true;

        #[allow(clippy::collapsible_match)]
        if let Some(inner) = try_delimited!(ts, Bracket) {
            let mut inner = inner.into_iter().peekable();

            expect_ident!(inner, "mut");
            expect_punct!(inner, '=');

            mutable = match unwrap!(inner, Ident).to_string().as_str() {
                "true" => true,
                "false" => false,
                _ => panic!()
            };
        }

        expect_punct!(ts, ';');
        fields.push((name, start, end, mutable));
    }

    let fields = fields.into_iter().map(|(name, start, end, mutable)| {
        match end {
            Some(end) => make_field_multi_byte(&ty, name, start, end, mutable),
            None => make_field_single_byte(name, start, mutable),
        }
    });

    quote! {
        #[repr(transparent)]
        pub struct #name(#ty);

        impl bitfi::BitField<#ty> for #name {
            #[inline(always)]
            fn set_bit(&mut self, i: #ty) {
                self.0.set_bit(i);
            }

            #[inline(always)]
            fn clear_bit(&mut self, i: #ty) {
                self.0.clear_bit(i);
            }

            #[inline(always)]
            fn get_bit(&self, i: #ty) -> bool {
                self.0.get_bit(i)
            }

            #[inline(always)]
            fn toggle_bit(&mut self, i: #ty) {
                self.0.toggle_bit(i);
            }

            #[inline(always)]
            fn set_bit_range(&mut self, range: ::core::ops::RangeInclusive<#ty>, b: #ty) {
                self.0.set_bit_range(range, b);
            }

            #[inline(always)]
            fn get_bit_range(&self, range: ::core::ops::RangeInclusive<#ty>) -> #ty {
                self.0.get_bit_range(range)
            }
        }

        impl ::core::default::Default for #name {
            fn default() -> Self { Self(0) }
        }

        impl #name {

            #[inline(always)]
            pub const fn new(n: #ty) -> Self {
                Self(n)
            }

            #[inline(always)]
            pub const fn get_inner(&self) -> #ty { self.0 }

            #[inline(always)]
            pub const fn set_inner(&mut self, val: #ty) {
                self.0 = val;
            }

            #(#fields)*
        }
    }
}

#[proc_macro]
pub fn bitfield(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ts: proc_macro2::TokenStream = ts.into();
    let mut ts = ts.into_iter().peekable();

    let mut bitfields = vec![];

    while ts.peek().is_some() {
        let bf = parse_bitfield(&mut ts);
        bitfields.push(bf);
    }

    quote! {
        #(#bitfields)*
    }.into()
}
