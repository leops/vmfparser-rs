//! Defines the VMF file parser

use synom::IResult;
use synom::space::*;
use ast::*;

pub fn name<'a, K>(i: &'a str) -> IResult<&str, K> where K: From<&'a str> {
    let i = skip_whitespace(i);
    let input_length = i.len();
    if input_length == 0 {
        return IResult::Error
    }

    for (idx, item) in i.chars().enumerate() {
        match item {
            'a'...'z' | 'A'...'Z' | '0'...'9' | '_' | '-' | '$' => {},
            _ => {
                if idx == 0 {
                    return IResult::Error
                } else {
                    return IResult::Done(&i[idx..], K::from(&i[..idx]))
                }
            },
        }
    }

    IResult::Done(&i[input_length..], K::from(i))
}

named!(
    pub string -> &str,
    delimited!(
        punct!("\""),
        take_until!("\""),
        punct!("\"")
    )
);

pub fn property<'a, K>(i: &'a str) -> IResult<&str, Property<K>> where K: From<&'a str> {
    do_parse!(i,
        key: string >>
        value: string >>
        (Property { key: key.into(), value: value.into() })
    )
}

pub fn block<'a, K>(i: &'a str) -> IResult<&str, Block<K>> where K: From<&'a str> {
    do_parse!(i,
        name: name >>
        punct!("{") >>
        body: map!(
            many0!(alt!(
                map!(property, |v| (Some(v), None)) |
                map!(block, |v| (None, Some(v)))
            )),
            |items: Vec<_>| {
                items.into_iter()
                    .fold(
                        (Vec::new(), Vec::new()),
                        |(mut props, mut blocks): (Vec<_>, Vec<_>), (prop, block)| {
                            if let Some(prop) = prop {
                                props.push(prop);
                            }
                            if let Some(block) = block {
                                blocks.push(block);
                            }
                            (props, blocks)
                        }
                    )
            }
        ) >>
        punct!("}") >>
        (Block { name, props: body.0, blocks: body.1 })
    )
}

pub fn file<'a, K>(i: &'a str) -> IResult<&str, Vec<Block<K>>> where K: From<&'a str> {
    many0!(i, block)
}
