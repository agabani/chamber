use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::complete::{escaped, is_a, tag, take_while},
    character::complete::{none_of, one_of},
    combinator::all_consuming,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

/// WWW-Authenticate response header.
#[derive(Debug, Clone)]
pub struct WwwAuthenticate<'a> {
    /// Challenges.
    pub challenges: Vec<Challenge<'a>>,
}

/// Challenge.
#[derive(Debug, Clone)]
pub struct Challenge<'a> {
    /// Authentication scheme.
    pub auth_scheme: Cow<'a, str>,
    /// Authentication parameters.
    pub auth_params: Vec<AuthParam<'a>>,
}

/// Authentication parameter.
#[derive(Debug, Clone)]
pub struct AuthParam<'a> {
    /// Key.
    pub key: Cow<'a, str>,
    /// Value.
    pub value: Cow<'a, str>,
}

impl<'a> WwwAuthenticate<'a> {
    /// Parse a [`WwwAuthenticate`] from a string.
    ///
    /// # Errors
    ///
    /// Will return `Err` if Www-Authenticate header is unparsable.
    pub fn parse(input: &'a str) -> Result<Self, nom::Err<nom::error::Error<&'a str>>> {
        let (_, challenges) = all_consuming(challenges())(input)?;

        Ok(Self {
            challenges: challenges
                .into_iter()
                .map(|(auth_scheme, auth_params)| Challenge {
                    auth_scheme: Cow::Borrowed(auth_scheme),
                    auth_params: auth_params
                        .into_iter()
                        .map(|(key, value)| AuthParam {
                            key: Cow::Borrowed(key),
                            value: Cow::Borrowed(value),
                        })
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
        // Arrange
        let input = r#"Basic realm="Registry Realm""#;

        // Act
        let www_authenticate = WwwAuthenticate::parse(input).unwrap();

        // Assert
        println!("{www_authenticate:?}");
    }

    #[test]
    fn bearer() {
        // Arrange
        let input = r#"Bearer realm="http://127.0.0.1:5003/auth",service="Docker registry",error="invalid_token""#;

        // Act
        let www_authenticate = WwwAuthenticate::parse(input).unwrap();

        // Assert
        println!("{www_authenticate:?}");
    }

    #[test]
    fn rfc_7235() {
        // Arrange
        let input =
            r#"Newauth realm="apps", type=1, title="Login to \"apps\"", Basic realm="simple""#;

        // Act
        let www_authenticate = WwwAuthenticate::parse(input).unwrap();

        // Assert
        println!("{www_authenticate:?}");
    }

    #[test]
    fn test_auth_param() {
        // Arrange
        let input = "key=value";

        // Act
        let (remaining, (key, value)) = auth_param()(input).unwrap();

        // Assert
        assert_eq!(remaining, "");
        assert_eq!(key, "key");
        assert_eq!(value, "value");
    }

    #[test]
    fn test_auth_params() {
        // Arrange
        let input =
            r#"realm="http://127.0.0.1:5003/auth",service="Docker registry",error="invalid_token""#;

        // Act
        let (remaining, params) = auth_params()(input).unwrap();

        // Assert
        assert_eq!(remaining, "");
        assert_eq!(params.len(), 3);
        assert_eq!(params[0].0, "realm");
        assert_eq!(params[0].1, "http://127.0.0.1:5003/auth");
        assert_eq!(params[1].0, "service");
        assert_eq!(params[1].1, "Docker registry");
        assert_eq!(params[2].0, "error");
        assert_eq!(params[2].1, "invalid_token");
    }

    #[test]
    fn test_challenge() {
        // Arrange
        let input = r#"Bearer realm="http://127.0.0.1:5003/auth",service="Docker registry",error="invalid_token""#;

        // Act
        let (remaining, challenge) = challenge()(input).unwrap();

        // Assert
        assert_eq!(remaining, "");
        assert_eq!(challenge.0, "Bearer");
        assert_eq!(challenge.1.len(), 3);
        assert_eq!(challenge.1[0].0, "realm");
        assert_eq!(challenge.1[0].1, "http://127.0.0.1:5003/auth");
        assert_eq!(challenge.1[1].0, "service");
        assert_eq!(challenge.1[1].1, "Docker registry");
        assert_eq!(challenge.1[2].0, "error");
        assert_eq!(challenge.1[2].1, "invalid_token");
    }

    #[test]
    fn test_challenges() {
        // Arrange
        let input: &str =
            r#"Newauth realm="apps", type=1, title="Login to \"apps\"", Basic realm="simple""#;

        // Act
        let (remaining, challenges) = challenges()(input).unwrap();

        // Assert
        assert_eq!(remaining, "");
        assert_eq!(challenges.len(), 2);
        assert_eq!(challenges[0].0, "Newauth");
        assert_eq!(challenges[0].1.len(), 3);
        assert_eq!(challenges[0].1[0].0, "realm");
        assert_eq!(challenges[0].1[0].1, "apps");
        assert_eq!(challenges[0].1[1].0, "type");
        assert_eq!(challenges[0].1[1].1, "1");
        assert_eq!(challenges[0].1[2].0, "title");
        assert_eq!(challenges[0].1[2].1, "Login to \\\"apps\\\"");
        assert_eq!(challenges[1].0, "Basic");
        assert_eq!(challenges[1].1.len(), 1);
        assert_eq!(challenges[1].1[0].0, "realm");
        assert_eq!(challenges[1].1[0].1, "simple");
    }

    #[test]
    fn test_quoted_string() {
        // Arrange
        let input = r#""Docker registry""#;

        // Act
        let (remaining, string) = quoted_string()(input).unwrap();

        // Assert
        assert_eq!(remaining, "");
        assert_eq!(string, "Docker registry");
    }

    #[test]
    fn test_quoted_string_escaped() {
        // Arrange
        let input = r#""Login to \"apps\"""#;

        // Act
        let (remaining, string) = quoted_string()(input).unwrap();

        // Assert
        assert_eq!(remaining, "");
        assert_eq!(string, "Login to \\\"apps\\\"");
    }

    #[test]
    fn test_token() {
        // Arrange
        let input = "list";

        // Act
        let (remaining, string) = token()(input).unwrap();

        // Assert
        assert_eq!(remaining, "");
        assert_eq!(string, "list");
    }
}
