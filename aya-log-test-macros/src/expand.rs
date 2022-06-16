use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Expr, LitStr, Token,
};

use aya_log_common::DisplayHint;
use aya_log_parser::{parse, Fragment};

pub(crate) struct LogArgs {
    pub(crate) target: Expr,
    pub(crate) level: Expr,
    pub(crate) format_string: LitStr,
    pub(crate) formatting_args: Option<Punctuated<Expr, Token![,]>>,
}

impl Parse for LogArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let target: Expr = input.parse()?;
        let level: Expr = input.parse()?;
        let format_string: LitStr = input.parse()?;
        let formatting_args: Option<Punctuated<Expr, Token![,]>> = if input.is_empty() {
            None
        } else {
            input.parse::<Token![,]>()?;
            Some(Punctuated::parse_terminated(input)?)
        };
    }
}

fn string_to_expr(s: Cow<str>) -> Result<Expr> {
    parse_str(&format!("\"{}\"", s))
}

fn hint_to_expr(hint: DisplayHint) -> Result<Expr> {
    match hint {
        DisplayHint::Default => {
            parse_str("::aya_log_ebpf::macro_support::DisplayHint::Default")
        }
        DisplayHint::LowerHex => {
            parse_str("::aya_log_ebpf::macro_support::DisplayHint::LowerHex")
        }
        DisplayHint::UpperHex => {
            parse_str("::aya_log_ebpf::macro_support::DisplayHint::UpperHex")
        }
        DisplayHint::IPv4 => {
            parse_str("::aya_log_ebpf::macro_support::DisplayHint::IPv4")
        }
        DisplayHint::IPv6 => {
            parse_str("::aya_log_ebpf::macro_support::DisplayHint::IPv6")
        }
    }
}

pub(crate) fn log(args: LogArgs) -> Result<TokenStream> {
    let format_string = args.format_string;
    let format_string_val = format_string.value();
    let fragments = parse(&format_string_val)
        .map_err(|_| Error::new(format_string.span(), "failed to parse format string"))?;

    let mut values = Vec::new();
    let mut hints = Vec::new();
    let mut arg_i = 0;
    for fragment in fragments {
        match fragment {
            Fragment::Literal(s) => {
                values.push(string_to_expr(s)?);
                hints.push(hint_to_expr(DisplayHint::Default)?);
            }
            Fragment::Parameter(p) => {
                let arg = match args.formatting_args {
                    Some(ref args) => args[arg_i].clone(),
                    None => return Err(Error::new(
                        format_string.span(),
                        "no arguments provided",
                    )),
                };
                values.push(arg);
                hints.push(hint_to_expr(p.hint)?);
                arg_i += 1;
            }
        }
    }
    let num_args = values.len();

    let values_iter = values.iter();
    let hints_iter = hints.iter();

    Ok(quote! {
        {
            if let Some(header_len) = ::aya_log_common::write_record_header(
                &mut buf,
                "test",
                ::aya_log_common::Level::Info,
                "test",
                "test.rs",
                123,
                args
            ) {
                let record_len = header_len;

                if let Ok(record_len) = {
                    use aya
                }
            }
        }
    })
}
