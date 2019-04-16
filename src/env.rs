use crate::{Filter, Getter};

pub static EV: Enver = Enver {};

pub struct Enver {}

impl Getter<String, std::option::IntoIter<String>> for Enver {
    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<String> {
        if f == Filter::Env {
            return std::env::var(s.as_ref()).ok();
        }
        None
    }
    fn values<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<std::option::IntoIter<String>> {
        if f == Filter::Env {
            return Some(std::env::var(s.as_ref()).ok().into_iter());
        }
        None
    }
}

