#[macro_use]
extern crate nom;

use nom::{IResult, alpha};

use Node::{Literal, Interpolation, Conditional};

#[derive(Debug,PartialEq,Eq)]
pub enum Node<'a> {
    Literal { contents: &'a [u8] },
    Interpolation { identifier: &'a [u8] },
    Conditional {
        identifier: &'a [u8],
        children: Vec<Node<'a>>,
    },
}

named!(block<Node>, alt!(
    not_token =>
        { |contents| Literal { contents: contents} }
  | interpolation =>
        { |identifier| Interpolation { identifier: identifier } }
  | conditional =>
        { |(_, identifier, children)| Conditional { identifier: identifier, children: children } }
));

named!(bool_positive_token, tag!("👍"));
named!(pen_start_token, tag!("✒️"));
named!(pen_end_token, tag!("🖋"));
named!(interp_token, tag!("🔤"));

named!(not_token, is_not!("🔤🖋✒️👍"));

named!(interpolation, delimited!(interp_token, alpha, interp_token));

named!(pen<&[u8], Vec<Node> >, delimited!(pen_start_token, multi, pen_end_token));

named!(conditional<&[u8], (&[u8], &[u8], Vec<Node>)>, tuple!(
  bool_positive_token, alpha, pen));

named!(multi<&[u8], Vec<Node> >, many0!( block ) );

pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<Node>> {
    return multi(input);
}

#[test]
fn interpolation_works() {
    let r = interpolation("🔤defgh🔤h".as_bytes());
    assert_eq!(
    r,
    IResult::Done(
      "h".as_bytes(),
      "defgh".as_bytes()
    )
  );
}

#[test]
fn block_works() {
    assert_eq!(
        block("wooot".as_bytes()),
        IResult::Done("".as_bytes(), Node::Literal { contents: "wooot".as_bytes()})
    );
}

#[test]
fn parse_works() {
    assert_eq!(
        parse("wooot🔤dang🔤oooo".as_bytes()),
        IResult::Done("".as_bytes(), vec!(
            Literal { contents: "wooot".as_bytes()},
            Interpolation { identifier: "dang".as_bytes()},
            Literal { contents: "oooo".as_bytes()}
        ))
    );
}

#[test]
fn conditional_works() {
    assert_eq!(
        conditional("👍foo✒️🖋".as_bytes()),
        IResult::Done("".as_bytes(), ("👍".as_bytes(), "foo".as_bytes(), vec![]))
    );
    assert_eq!(
        conditional("👍foo✒️hello world🖋".as_bytes()),
        IResult::Done("".as_bytes(),
            (
                "👍".as_bytes(),
                "foo".as_bytes(),
                vec!(Literal { contents: "hello world".as_bytes()})
            )
        )
    );
}

#[test]
fn nested_works() {
    assert_eq!(
        multi("blahblah👍foo✒️hello 🔤name🔤!🖋".as_bytes()),
        IResult::Done("".as_bytes(), vec!(
            Literal {contents: "blahblah".as_bytes()},
            Conditional { identifier: "foo".as_bytes(), children: vec!(
                Literal { contents: "hello ".as_bytes()},
                Interpolation { identifier: "name".as_bytes()},
                Literal { contents: "!".as_bytes()}
            )}
        ))
    );
}
