use crate::{Filter, Getter};

pub static EV: Enver = Enver {};

pub struct Enver {}

impl<'a> Getter<'a> for Enver {
    type Iter = Option<String>;
    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<String> {
        if f == Filter::Env {
            return std::env::var(s.as_ref()).ok();
        }
        None
    }
    fn values<S: AsRef<str>>(&'a self, s: S, f: Filter) -> Option<Self::Iter> {
        if f == Filter::Env {
            return Some(std::env::var(s.as_ref()).ok());
        }
        None
    }
}