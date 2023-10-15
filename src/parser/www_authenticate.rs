use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_until, take_while},
    combinator::rest,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

pub struct WwwAuthenticate<'a> {
    challenges: Vec<Challenge<'a>>,
}

pub struct Challenge<'a> {
    auth_scheme: &'a str,
    auth_params: Vec<AuthParam<'a>>,
}
pub struct AuthParam<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> WwwAuthenticate<'a> {
    pub fn parse(input: &'a str) -> Result<Self, nom::Err<nom::error::Error<&'a str>>> {
        let x = auth_param()(input)?;

        todo!()
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

fn quoted_string<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    delimited(tag("\""), take_until("\""), tag("\""))
}

fn token<'a>() -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    is_a("!#$%&'*+-.^_`|~0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
}

fn whitespace<'a>() -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    take_while(|c| c == ' ')
}

#[cfg(test)]
mod tests {
    use super::{auth_param, auth_params, challenge, quoted_string, token};

    #[test]
    fn basic() {
        let x = r#"Basic realm="Registry Realm""#;
    }

    #[test]
    fn bearer() {
        let x = r#"Bearer realm="http://127.0.0.1:5003/auth",service="Docker registry",error="invalid_token""#;
    }

    #[test]
    fn rfc() {
        let x = r#"Newauth realm="apps", type=1, title="Login to \"apps\"", Basic realm="simple""#;
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

        let (remaining, challenges) = challenge()(input).unwrap();

        println!("remaining:{remaining} challenge:{challenges:?}");
    }

    #[test]
    fn test_quoted_string() {
        let input = r#""Docker registry""#;

        let (remaining, string) = quoted_string()(input).unwrap();

        println!("remaining:{remaining} string:{string:?}");
    }

    #[test]
    fn test_token() {
        let (remaining, value) = token()("liistx").unwrap();

        println!("remaining:{remaining} value:{value}");
    }
}
