#[macro_use]
extern crate nom;

use nom::{IResult, alpha};

// Parser definition

use std::str;

#[derive(Debug,PartialEq,Eq)]
enum Node<'a> {
  Literal { contents: &'a [u8]},
  Interpolation { identifier: &'a [u8] },
  Conditional { identifier: &'a [u8], children: Vec<Node<'a>> }
}

named!(thing<Node>, alt!(
    not_token =>      { |contents|          Node::Literal { contents: contents} }
  | interpolation =>  { |identifier|         Node::Interpolation { identifier: identifier } }
  | conditional =>    {  |(a, b, d)|  Node::Conditional { identifier: b, children: d }  }
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


fn main() {
  let r = interpolation("🔤defgh🔤h".as_bytes());
  assert_eq!(
    r,
    IResult::Done(
      "h".as_bytes(),
      "defgh".as_bytes()
    )
  );

  // one
  let r1 = thing("wooot".as_bytes());
  assert_eq!(r1, IResult::Done("".as_bytes(), Node::Literal { contents: "wooot".as_bytes()}));

  // two
  let r2 = multi("wooot🔤dang🔤oooo".as_bytes());
  let e2 = vec!(
    Node::Literal { contents: "wooot".as_bytes()},
    Node::Interpolation { identifier: "dang".as_bytes()},
    Node::Literal { contents: "oooo".as_bytes()}
  );
  assert_eq!(r2, IResult::Done("".as_bytes(), e2));

  // conditional
  let r5 = conditional("👍foo✒️🖋".as_bytes());
  let e5 = (
    "👍".as_bytes(),
    "foo".as_bytes(),
    vec!()
  );
  assert_eq!(r5, IResult::Done("".as_bytes(), e5));

  // conditional
  let r3 = conditional("👍foo✒️hello world🖋".as_bytes());
  let e3 = (
    "👍".as_bytes(),
    "foo".as_bytes(),
    vec!(Node::Literal { contents: "hello world".as_bytes()})
  );
  assert_eq!(r3, IResult::Done("".as_bytes(), e3));

  // multi-nested
  let r4 = multi("blahblah👍foo✒️hello 🔤name🔤!🖋".as_bytes());
  let e4 = vec!(
    Node::Literal {contents: "blahblah".as_bytes()},
    Node::Conditional { identifier: "foo".as_bytes(), children: vec!(
      Node::Literal { contents: "hello ".as_bytes()},
      Node::Interpolation { identifier: "name".as_bytes()},
      Node::Literal { contents: "!".as_bytes()}
    )}
  );
  assert_eq!(r4, IResult::Done("".as_bytes(), e4));
}



