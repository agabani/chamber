use nom::{
    bytes::complete::{is_a, tag},
    sequence::separated_pair,
    IResult,
};

pub struct WwwAuthenticate<'a> {
    challenges: Vec<Challenge<'a>>,
}

pub struct Challenge<'a> {
    scheme: &'a str,
}

impl<'a> WwwAuthenticate<'a> {
    pub fn parse(value: &'a str) -> Self {
        Self {
            challenges: vec![Challenge { scheme: value }],
        }
    }
}

fn auth_param<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, (&'a str, &'a str)> {
    separated_pair(token(), tag("="), token())
}

fn token<'a>() -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    is_a("!#$%&'*+-.^_`|~0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
}

#[cfg(test)]
mod tests {
    use super::{auth_param, token};

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
        let (remaining, (key, value)) = auth_param()("key=value").unwrap();

        println!("remaining:{remaining} key:{key} value:{value}");
    }

    #[test]
    fn test_token() {
        let (remaining, value) = token()("liistx").unwrap();

        println!("remaining:{remaining} value:{value}");
    }
}
