use nom::{Parser, sequence::delimited, character::complete::{multispace0, alpha1, alphanumeric1}, error::ParseError, InputTakeAtPosition, AsChar, IResult, combinator::recognize, multi::many0};
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::sequence::tuple;
use super::error::Error;


/// trim space before and after `f`
pub fn ws<I, O, E, F> (f: F) -> impl Parser<I, O, E>
where 
    E: ParseError<I>,
    F: Parser<I, O, E>,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    delimited(multispace0, f, multispace0)
}

/// Label names must match the regex `[a-zA-Z_][a-zA-Z0-9_]*`. 
/// Label names beginning with `__` are reserved for internal use.
pub fn parse_label_name(input: &str) -> IResult<&str, &str, Error<&str>>{
    recognize(
        tuple((
            alt((alpha1, is_a("_"))),
            many0(alt((alphanumeric1, is_a("_")))),
        ))
    )(input)
}


/// The metric name must match the regex `[a-zA-Z_:][a-zA-Z0-9_:]*`.
pub fn parse_metric_name(input: &str) -> IResult<&str, &str, Error<&str>> {
    recognize(
        tuple((
            alt((alpha1, is_a("_:"))),
            many0(alt((alphanumeric1, is_a("_:"))))
        ))
    )(input)
}


#[cfg(test)]
mod tests{
    use super::parse_metric_name;

    #[test]
    fn test_parse_metric_name(){
        let ok_metric_name = vec![
            "instance",
            "a",
            "__a__",
            "__name__",
            "method_code:http_errors:rate5m"
        ];

        ok_metric_name.iter().for_each(|input|{
            assert_eq!(
                parse_metric_name(input),
                Ok(("", input.to_owned()))
            );
        })
    }
}