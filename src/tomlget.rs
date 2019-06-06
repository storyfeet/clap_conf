use crate::convert::Localizer;
use crate::replace::{replace_env, ConfError};
use crate::{Filter, Getter};
use std::path::{Path, PathBuf};
use toml::Value;

pub fn load_toml<S: AsRef<str>>(s: S) -> Result<Localizer<Value>, ConfError> {
    let fname = replace_env(s.as_ref())?;
    let fcont = std::fs::read_to_string(&fname)?;
    let v = fcont.parse::<Value>()?;
    let fpar = PathBuf::from(PathBuf::from(fname).parent().unwrap_or(Path::new("./")));

    Ok(Localizer::new(v, fpar))
}

pub fn load_first_toml<S: AsRef<str>, IT: IntoIterator<Item = S>>(
    a: Option<&str>,
    i: IT,
) -> Result<Localizer<Value>, ConfError> {
    if let Some(m) = a {
        println!("config selected = {} ", m);
        match load_toml(m) {
            Ok(v) => return Ok(v),
            Err(e) => {
                println!("Could not load selected file {:?}", e);
                return Err(e);
            }
        }
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

impl<'a> Getter<'a, String> for Value {
    //This uses the Getter for &Value impl but has a different return type.
    type Iter = std::vec::IntoIter<String>;
    fn bool_flag<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        (&self).bool_flag(s, f)
    }

    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<String> {
        match (&self).value(s, f)? {
            Value::String(s) => Some(s.clone()),
            Value::Integer(i) => Some(i.to_string()),
            Value::Boolean(i) => Some(i.to_string()),
            Value::Float(f) => Some(f.to_string()),
            Value::Datetime(f) => Some(f.to_string()),
            _ => None,
        }
    }

    fn values<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<std::vec::IntoIter<String>> {
        let res: Vec<String> = (&self)
            .values(s, f)?
            .filter_map(|v| v.as_str().map(|vr| vr.to_string()))
            .collect();
        Some(res.into_iter())
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
        let r = (&&t).value("a.b.c.car", Filter::Conf).unwrap();
        assert_eq!(r.as_str().unwrap(), "red");

        let t: Value = "[[a.b]]\ncar=\"red\"\n[[a.b]]\ncar=\"green\""
            .parse()
            .unwrap();
        let r = (&&t).value("a.b.1.car", Filter::Conf).unwrap();
        assert_eq!(r.as_str().unwrap(), "green");
    }

    #[test]
    fn test_iter() {
        let t: Value = "[a.b]\ncar=[\"red\",\"green\"]".parse().unwrap();
        let mut r = (&&t)
            .values("a.b.car", Filter::Conf)
            .expect("Could not get values");
        assert_eq!(r.next().unwrap().as_str().unwrap(), "red");
        assert_eq!(r.next().unwrap().as_str().unwrap(), "green");
    }
}
