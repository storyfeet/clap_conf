//use failure_derive::*;
use thiserror::*;

#[derive(Clone, Error, Debug)]
pub enum ConfError {
    #[error("Syntax Error Parsing String")]
    Syntax,
    #[error("Environment variable not found")]
    VarNotFound,
    #[error("Could not load file {}", 0)]
    LoadError(String),
    #[error("{}", _0)]
    Mess(&'static str),
    #[error("{}", _0)]
    Message(String),
}

impl ConfError {
    pub fn with_info(self, s: &str) -> Self {
        ConfError::Message(format!("{} - {}", self, s))
    }
}

impl From<&'static str> for ConfError {
    fn from(s: &'static str) -> Self {
        ConfError::Mess(s)
    }
}

impl From<std::io::Error> for ConfError {
    fn from(e: std::io::Error) -> Self {
        ConfError::LoadError(e.to_string())
    }
}

impl From<std::env::VarError> for ConfError {
    fn from(_: std::env::VarError) -> Self {
        ConfError::VarNotFound
    }
}
impl From<toml::de::Error> for ConfError {
    fn from(_: toml::de::Error) -> Self {
        ConfError::Syntax
    }
}

type Job<E> = dyn Fn(&str) -> Result<String, E>;

fn _replace<IT, E>(it: &mut IT, f: &Job<E>, depth: u8) -> Result<String, ConfError>
where
    IT: Iterator<Item = char>,
    //J:Job<E>,
    ConfError: From<E>,
{
    let mut res = String::new();
    while let Some(c) = it.next() {
        match c {
            '\\' => res.push(it.next().ok_or(ConfError::Syntax)?),
            '{' => {
                let s = _replace(it, f, depth + 1)?;
                res.push_str(&f(&s)?);
            }
            '}' => {
                if depth == 0 {
                    return Err(ConfError::Syntax);
                }
                return Ok(res);
            }
            c => res.push(c),
        }
    }
    if depth > 0 {
        return Err(ConfError::Syntax);
    }
    Ok(res)
}

pub fn replace<E>(s: &str, f: &Job<E>) -> Result<String, ConfError>
where
    ConfError: From<E>,
    //J:Job<E>,
{
    _replace(&mut s.chars(), f, 0)
}

pub fn replace_simple<F: 'static + Fn(&str) -> String>(s: &str, f: F) -> Result<String, ConfError> {
    replace::<ConfError>(s, &move |s| Ok(f(s)))
}

pub fn replace_env(s: &str) -> Result<String, ConfError> {
    replace(s, &|v| std::env::var(v))
}

#[cfg(test)]
mod test {
    use super::*;
    fn mini_rep(s: &str) -> String {
        s.to_lowercase()
    }
    #[test]
    pub fn rep_test() {
        let s2 = replace_simple("HELLO{WORLD}", &mini_rep).unwrap();
        assert_eq!(&s2, "HELLOworld");
    }
}
