use crate::{Filter, Getter};
use toml::Value;

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

impl <'a> Getter<&'a Value,std::slice::Iter<'a, Value>> for Value {
    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<&'a Value> {
        if f != Filter::Conf {
            return None;
        }
        let r = dig(self, s.as_ref().split("."))?;
        Some(r.as_str()?.to_string())
    }

    fn values<S: AsRef<str>>(&'a self, s: S, f: Filter) -> Option<std::slice::Iter<'a,Value>> {
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
        let r = t.value("a.b.c.car", Filter::Conf).unwrap();
        assert_eq!(r, "red");

        let t: Value = "[[a.b]]\ncar=\"red\"\n[[a.b]]\ncar=\"green\""
            .parse()
            .unwrap();
        let r = t.value("a.b.1.car", Filter::Conf).unwrap();
        assert_eq!(r, "green");
    }

    #[test]
    fn test_iter() {
        let t: Value = "[a.b]\ncar=[\"red\",\"green\"]".parse().unwrap();
        let mut r = t
            .values("a.b.car", Filter::Conf)
            .expect("Could not get values");
        assert_eq!(r.next().unwrap().as_str().unwrap(), "red");
        assert_eq!(r.next().unwrap().as_str().unwrap(), "green");
    }
}
