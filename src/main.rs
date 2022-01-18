use nom::{
    IResult,
    bytes::complete::{tag, take_until},
    sequence::tuple,
    branch::alt,
    combinator::{opt, value}
};
#[derive(PartialEq, Debug, Clone, Copy)]
enum Tag {
    Param,
    Author,
}

fn comment_start(input: &str) -> IResult<&str, &str> {
    tag("/**\n")(input)
}

fn comment_line_start(input: &str) -> IResult<&str, &str> {
    tag("* ")(input)
}

fn comment_description(input: &str) -> IResult<&str, &str> {
    alt((
        take_until("{"),
        take_until("*/")
    ))(input)
}

fn comment_param(input: &str) -> IResult<&str, Option<Tag>>{
    opt(value(Tag::Param, tag("{param}")))(input)
}

fn comment_end(input: &str) -> IResult<&str, &str> {
    tag("*/")(input)
}

fn foo(input: &str) -> IResult<&str, (&str, Option<Tag>)> {
    let(input, (_, _, desc, param, _)) = tuple((
        comment_start,
        comment_line_start,
        comment_description,
        comment_param,
        comment_end
    ))(input)?;

    Ok((input, (desc, param)))
}

fn main() {
    let res = foo("/**\n* This is a description\n*/");
    println!("{:?}", res);
}

