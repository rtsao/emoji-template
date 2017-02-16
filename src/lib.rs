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

named!(thing<Node>, alt!(
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

named!(multi<&[u8], Vec<Node> >, many0!( thing ) );

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
fn thing_works() {
    // one
    let r1 = thing("wooot".as_bytes());
    assert_eq!(r1, IResult::Done("".as_bytes(), Node::Literal { contents: "wooot".as_bytes()}));
}

#[test]
fn parse_works() {
    // two
    let r2 = parse("wooot🔤dang🔤oooo".as_bytes());
    let e2 = vec!(
        Literal { contents: "wooot".as_bytes()},
        Interpolation { identifier: "dang".as_bytes()},
        Literal { contents: "oooo".as_bytes()}
    );
    assert_eq!(r2, IResult::Done("".as_bytes(), e2));
}

#[test]
fn conditional_works() {
    // conditional
    let r5 = conditional("👍foo✒️🖋".as_bytes());
    let e5 = ("👍".as_bytes(), "foo".as_bytes(), vec![]);
    assert_eq!(r5, IResult::Done("".as_bytes(), e5));

    // conditional
    let r3 = conditional("👍foo✒️hello world🖋".as_bytes());
    let e3 =
        ("👍".as_bytes(), "foo".as_bytes(), vec!(Literal { contents: "hello world".as_bytes()}));
    assert_eq!(r3, IResult::Done("".as_bytes(), e3));
}

#[test]
fn nested_works() {
    // multi-nested
    let r4 = multi("blahblah👍foo✒️hello 🔤name🔤!🖋".as_bytes());
    let e4 = vec!(
        Literal {contents: "blahblah".as_bytes()},
        Conditional { identifier: "foo".as_bytes(), children: vec!(
            Literal { contents: "hello ".as_bytes()},
            Interpolation { identifier: "name".as_bytes()},
            Literal { contents: "!".as_bytes()}
        )}
    );
    assert_eq!(r4, IResult::Done("".as_bytes(), e4));
}
