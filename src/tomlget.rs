use crate::replace::{replace_env, ConfError};
use crate::{Filter, Getter};
use toml::Value;

pub fn load_toml<S: AsRef<str>>(s: S) -> Result<Value, ConfError> {
    let fname = replace_env(s.as_ref())?;
    let fcont = std::fs::read_to_string(fname)?;
    fcont.parse::<Value>().map_err(|e| e.into())
}

pub fn load_first_toml<S: AsRef<str>, IT: IntoIterator<Item = S>>(
    a: Option<&str>,
    i: IT,
) -> Result<Value, ConfError> {
    if let Some(m) = a {
        return load_toml(m);
    }
    for s in i {
        match load_toml(s) {
            Ok(m) => return Ok(m),
            Err(_) => continue,
        }
    }
    Err("could not load".into())
}

fn dig<S: AsRef<str>, I: Iterator<Item = S>>(v: &Value, mut i: I) -> Option<&Value> {
    match i.next() {
        None => Some(v),
        Some(s) => match v {
            Value::Table(t) => dig(t.get(s.as_ref())?, i),
            Value::Array(a) => dig(a.get(s.as_ref().parse::<usize>().ok()?)?, i),
            _ => None,
        },
    }
}


impl<'a> Getter<'a, &'a Value> for &'a Value {
    type Iter = std::slice::Iter<'a, Value>;
    fn bool_flag<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        if f != Filter::Conf {
            return false;
        }
        match dig(self, s.as_ref().split(".")) {
            Some(v) => v.as_bool().unwrap_or(false),
            None => false,
        }
    }

    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<&'a Value> {
        if f != Filter::Conf {
            return None;
        }
        let r = dig(self, s.as_ref().split("."))?;
        Some(r)
    }

    fn values<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<std::slice::Iter<'a, Value>> {
        if f != Filter::Conf {
            return None;
        }

        let v = dig(self, s.as_ref().split("."))?;

        if let Value::Array(a) = v {
            return Some(a.into_iter());
        }
        None
    }
}

#[cfg(test)]
mod tomltests {
    use super::*;
    #[test]
    fn test_load() {
        let t: Value = "[a.b.c]\ncar=\"red\"".parse().unwrap();
        let r = (&t).value("a.b.c.car", Filter::Conf).unwrap();
        assert_eq!(r.as_str().unwrap(), "red");

        let t: Value = "[[a.b]]\ncar=\"red\"\n[[a.b]]\ncar=\"green\""
            .parse()
            .unwrap();
        let r = (&t).value("a.b.1.car", Filter::Conf).unwrap();
        assert_eq!(r.as_str().unwrap(), "green");
    }

    #[test]
    fn test_iter() {
        let t: Value = "[a.b]\ncar=[\"red\",\"green\"]".parse().unwrap();
        let mut r = (&t)
            .values("a.b.car", Filter::Conf)
            .expect("Could not get values");
        assert_eq!(r.next().unwrap().as_str().unwrap(), "red");
        assert_eq!(r.next().unwrap().as_str().unwrap(), "green");
    }
}