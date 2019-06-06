use crate::{Filter, Getter};

pub static EV: Enver = Enver {};

#[derive(Debug)]
pub struct Enver {}

impl<'a> Getter<'a,String> for Enver {
    type Iter = std::option::IntoIter<String>;
    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<String> {
        if f == Filter::Env {
            return std::env::var(s.as_ref()).ok();
        }
        None
    }
    fn values<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<Self::Iter> {
        if f == Filter::Env {
            return Some(std::env::var(s.as_ref()).ok().into_iter());
        }
        None
    }
}
