use nom::{
    branch::alt,
    bytes::complete::{escaped, is_a, tag, take_while},
    character::complete::{none_of, one_of},
    combinator::all_consuming,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
pub struct WwwAuthenticate<'a> {
    pub challenges: Vec<Challenge<'a>>,
}

#[derive(Debug)]
pub struct Challenge<'a> {
    pub auth_scheme: &'a str,
    pub auth_params: Vec<AuthParam<'a>>,
}

#[derive(Debug)]
pub struct AuthParam<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> WwwAuthenticate<'a> {
    pub fn parse(input: &'a str) -> Result<Self, nom::Err<nom::error::Error<&'a str>>> {
        let (_remaining, challenges) = all_consuming(challenges())(input)?;

        Ok(Self {
            challenges: challenges
                .into_iter()
                .map(|(auth_scheme, auth_params)| Challenge {
                    auth_scheme,
                    auth_params: auth_params
                        .into_iter()
                        .map(|(key, value)| AuthParam { key, value })
                        .collect(),
                })
                .collect(),
        })
    }
}

fn auth_param<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, (&'a str, &'a str)> {
    separated_pair(token(), tag("="), alt((token(), quoted_string())))
}

fn auth_params<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<(&'a str, &'a str)>> {
    separated_list0(
        delimited(whitespace(), tag(","), whitespace()),
        auth_param(),
    )
}

fn challenge<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, (&'a str, Vec<(&'a str, &'a str)>)> {
    separated_pair(token(), tag(" "), auth_params())
}

fn challenges<'a>(
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<(&'a str, Vec<(&'a str, &'a str)>)>> {
    separated_list1(delimited(whitespace(), tag(","), whitespace()), challenge())
}

fn quoted_string<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    delimited(
        tag("\""),
        escaped(none_of(r#"\""#), '\\', one_of(r#"""#)),
        tag("\""),
    )
}

fn token<'a>() -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    is_a("!#$%&'*+-.^_`|~0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
}

fn whitespace<'a>() -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    take_while(|c| c == ' ')
}

#[cfg(test)]
mod tests {
    use super::{
        auth_param, auth_params, challenge, challenges, quoted_string, token, WwwAuthenticate,
    };

    #[test]
    fn basic() {
        let input = r#"Basic realm="Registry Realm""#;

        let www_authenticate = WwwAuthenticate::parse(input).unwrap();

        println!("{www_authenticate:?}");
    }

    #[test]
    fn bearer() {
        let input = r#"Bearer realm="http://127.0.0.1:5003/auth",service="Docker registry",error="invalid_token""#;

        let www_authenticate = WwwAuthenticate::parse(input).unwrap();

        println!("{www_authenticate:?}");
    }

    #[test]
    fn rfc() {
        let input =
            r#"Newauth realm="apps", type=1, title="Login to \"apps\"", Basic realm="simple""#;

        let www_authenticate = WwwAuthenticate::parse(input).unwrap();

        println!("{www_authenticate:?}");
    }

    #[test]
    fn test_auth_param() {
        let input = "key=value";

        let (remaining, (key, value)) = auth_param()(input).unwrap();

        println!("remaining:{remaining} key:{key} value:{value}");
    }

    #[test]
    fn test_auth_params() {
        let input =
            r#"realm="http://127.0.0.1:5003/auth",service="Docker registry",error="invalid_token""#;

        let (remaining, params) = auth_params()(input).unwrap();

        println!("remaining:{remaining} params:{params:?}");
    }

    #[test]
    fn test_challenge() {
        let input = r#"Bearer realm="http://127.0.0.1:5003/auth",service="Docker registry",error="invalid_token""#;

        let (remaining, challenge) = challenge()(input).unwrap();

        println!("remaining:{remaining} challenge:{challenge:?}");
    }

    #[test]
    fn test_challenges() {
        let input =
            r#"Newauth realm="apps", type=1, title="Login to \"apps\"", Basic realm="simple""#;

        let (remaining, challenges) = challenges()(input).unwrap();

        println!("remaining:{remaining} challenge:{challenges:?}");
    }

    #[test]
    fn test_quoted_string() {
        let input = r#""Docker registry""#;

        let (remaining, string) = quoted_string()(input).unwrap();

        println!("remaining:{remaining} string:{string:?}");
    }

    #[test]
    fn test_quoted_string_escaped() {
        let input = r#""Login to \"apps\"""#;

        let (remaining, string) = quoted_string()(input).unwrap();

        println!("remaining:{remaining} string:{string:?}");
    }

    #[test]
    fn test_token() {
        let (remaining, value) = token()("liistx").unwrap();

        println!("remaining:{remaining} value:{value}");
    }
}
