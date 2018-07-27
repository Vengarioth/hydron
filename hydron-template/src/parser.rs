use ast::*;
use nom::*;
use nom::{Err, ErrorKind};
use proc_macro2::{Delimiter, Spacing, TokenStream, TokenTree};

type TokenTreeSlice<'a> = &'a [TokenTree];
pub type TemplateIResult<'a, T> = IResult<TokenTreeSlice<'a>, T>;

macro_rules! many0Tokens(
    ($i:expr, $submac:ident!( $($args:tt)* )) => ({
        use ::nom::lib::std::result::Result::*;
        use ::nom::Err;

        let ret;
        let mut vec_of_responses = ::nom::lib::std::vec::Vec::new();
        let mut input = $i.clone();

        loop {
            let input_ = input.clone();
            match $submac!(input_, $($args)*) {
                Ok((i, o)) => {
                    if i.len() == 0 || i.len() == input.len() {
                        vec_of_responses.push(o);
                        ret = Ok((input, vec_of_responses));
                        break;
                    }
                    vec_of_responses.push(o);
                    input = i;
                },
                Err(Err::Error(_)) => {
                    ret = Ok((input, vec_of_responses));
                    break;
                },
                Err(e) => {
                    ret = Err(e);
                    break;
                },
            }
        }

        ret
    });
    ($i:expr, $f:expr) => (
        many0Tokens!($i, call!($f));
    );
);

fn match_punct(input: TokenTreeSlice, c: char, spacing: Spacing) -> TemplateIResult<char> {
    match input[0] {
        TokenTree::Punct(ref punct) => {
            if c != punct.as_char() {
                return Err(Err::Error(error_position!(input, ErrorKind::Custom(1000))));
            }

            if spacing != punct.spacing() {
                return Err(Err::Error(error_position!(input, ErrorKind::Custom(1000))));
            }

            return Ok((&input[1..], punct.as_char()));
        }
        _ => return Err(Err::Error(error_position!(input, ErrorKind::Custom(1000)))),
    }
}

pub fn match_ident(input: TokenTreeSlice) -> TemplateIResult<String> {
    match input[0] {
        TokenTree::Ident(ref ident) => {
            return Ok((&input[1..], format!("{}", ident)));
        }
        _ => return Err(Err::Error(error_position!(input, ErrorKind::Custom(1000)))),
    }
}

fn match_group(input: TokenTreeSlice, delimiter: Delimiter) -> TemplateIResult<TokenStream> {
    match input[0] {
        TokenTree::Group(ref group) => {
            if delimiter != group.delimiter() {
                return Err(Err::Error(error_position!(input, ErrorKind::Custom(1000))));
            }

            return Ok((&input[1..], group.stream()));
        }
        _ => return Err(Err::Error(error_position!(input, ErrorKind::Custom(1000)))),
    }
}

named!(parse_property<TokenTreeSlice, Property>,
    do_parse!(
        name: match_ident >>
        apply!(match_punct, '=', Spacing::Alone) >>
        value: apply!(match_group, Delimiter::Brace) >>
        (Property {
            name: Identifier{ name },
            value: value,
        })
    )
);

named!(parse_properties<TokenTreeSlice, Vec<Property>>,
    many0Tokens!(
        parse_property
    )
);

named!(parse_escaped_group<TokenTreeSlice, TagContent>,
    do_parse!(
        group: apply!(match_group, Delimiter::Brace) >>
        (TagContent::Escaped(group))
    )
);

named!(parse_child_tag<TokenTreeSlice, TagContent>,
    do_parse!(
        tag: parse_tag >>
        (TagContent::Child(tag))
    )
);

named!(parse_tag_content<TokenTreeSlice, Vec<TagContent>>,
    many0Tokens!(
        alt!(
            parse_escaped_group |
            parse_child_tag
        )
    )
);

named!(parse_open_tag<TokenTreeSlice, (Identifier, Vec<Property>)>,
    do_parse!(
        apply!(match_punct, '<', Spacing::Alone) >>
        ident: match_ident >>
        properties: parse_properties >>
        apply!(match_punct, '>', Spacing::Alone) >>
        ((Identifier {
            name: ident,
        }, properties))
    )
);

named!(parse_close_tag<TokenTreeSlice, Identifier>,
    do_parse!(
        apply!(match_punct, '<', Spacing::Alone) >>
        apply!(match_punct, '/', Spacing::Alone) >>
        ident: match_ident >>
        apply!(match_punct, '>', Spacing::Alone) >>
        (Identifier {
            name: ident,
        })
    )
);

named!(parse_container_tag<TokenTreeSlice, Tag>,
    do_parse!(
        open: parse_open_tag >>
        content: parse_tag_content >>
        // TODO check if tags match
        _close: parse_close_tag >>
        (Tag {
            name: open.0,
            properties: open.1,
            content: content,
        })
    )
);

named!(parse_standalone_tag<TokenTreeSlice, Tag>,
    do_parse!(
        apply!(match_punct, '<', Spacing::Alone) >>
        name: match_ident >>
        properties: parse_properties >>
        apply!(match_punct, '/', Spacing::Alone) >>
        apply!(match_punct, '>', Spacing::Alone) >>
        (Tag {
            name: Identifier{ name },
            properties,
            content: vec![],
        })
    )
);

named!(parse_tag<TokenTreeSlice, Tag>,
    alt!(
        parse_standalone_tag |
        parse_container_tag
    )
);

named!(parse_root<TokenTreeSlice, Tag>,
    do_parse!(
        tag: parse_tag >>
        (tag)
    )
);

pub fn parse(tree: TokenTreeSlice) -> Tag {
    parse_root(tree).unwrap().1
}
