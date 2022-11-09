use log::debug;
use nom::{branch::alt, sequence::delimited, IResult, bytes::complete::{tag, escaped, is_not}, number::complete::double};
use nom::character::complete::one_of;
use crate::ast::literal::{StringLiteral, NumberLiteral};

use super::{error::Error, helper::ws};

/// Scalar float values can be written as literal integer or floating-point numbers in the format.
pub fn parse_number_literal(input: &str) -> IResult<&str, NumberLiteral, Error<&str>>{
    debug!("parse_number_literal: {}", input);
    double(input).map(|(input, value)| (input, NumberLiteral{ value}))
}


/// Strings may be specified as literals in single quotes, double quotes or backticks.
/// In single or double quotes a backslash begins an escape sequence, which may be followed by `a`, `b`, `f`, `n`, `r`, `t`, `v` or `\`.
pub fn parse_string_literal(input: &str) -> IResult<&str, StringLiteral, Error<&str>> {
    debug!("parse_string_literal: {}", input);
    alt((
        delimited(
            ws(tag("\"")), 
            escaped(is_not("\"\\"), '\\', one_of("abfnrtv\"\\")), 
            ws(tag("\"")), 
        ),
        delimited(
            ws(tag("'")), 
            escaped(is_not("'\\"), '\\', one_of("abfnrtv'\\")), 
            ws(tag("'"))
        ),
        delimited(
            ws(tag("`")), 
            escaped(is_not("`\\"), '\\', one_of("abfnrtv`\\")), 
            ws(tag("`"))
        )
    ))(input)
    .map(|(input, s)|{
        (
            input,
            StringLiteral {
                value: s.to_owned(),
            }
        )
    })
}


#[cfg(test)]
mod tests{
    use crate::ast::literal::NumberLiteral;


    #[test]
    fn test_parse_string(){
        use crate::ast::literal::StringLiteral;
        let str_parse = vec![
            (r#" `abcd` "#, r#"abcd"#),
            (r#" `111\`222` "#, r#"111\`222"#),
            (r#" `1` "#, r#"1"#),
            (r#" '111\'222' "#, r#"111\'222"#),
            (r#" '1' "#,  r#"1"#),
            (r#" "1" "#, r#"1"#),
            (r#" "abcd" "#, r#"abcd"#),
            (r#" "ab cd _ 123" "#,  r#"ab cd _ 123"#),
            (r#" "111\"222" "#, r#"111\"222"#),
            (r#"`these are unescaped: \n \\ \t`"#, r#"these are unescaped: \n \\ \t"#)
        ];

        str_parse.iter().for_each(
            |(input, output)| {
                assert_eq!(
                    super::parse_string_literal(input), 
                    Ok(("", 
                        StringLiteral{
                            value: output.to_owned().to_string()
                        }
                    ))
                )
            }
        
        );
    }

    #[test]
    fn test_parse_number(){
        let num_parse = vec![
            (r#"1.23"#, 1.23f64),
            (r#"-1.23"#, -1.23f64),
            (r#"1e4"#, 10000f64),
            (r#"1e-4"#, 0.0001f64),
            (r#"-1e4"#, -10000f64),
            (r#"-1e-4"#, -0.0001f64),
        ];

        num_parse.iter().for_each(|(input, output)|{
            assert_eq!(
                super::parse_number_literal(input),
                Ok(("",
                    NumberLiteral{
                        value: *output
                    }
                ))
            )
        })
    }
}