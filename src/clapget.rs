use crate::{Filter, Getter};
use clap::{ArgMatches, Values};

impl<'a> Getter<'a> for ArgMatches<'a> where {
    type Iter = Values<'a>;
    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<String> {
        if f == Filter::Arg {
            return self.value_of(s).map(|s| s.to_string());
        }
        None
    }

    fn values<S: AsRef<str>>(&'a self, s: S, f: Filter) -> Option<Self::Iter> {
        if f == Filter::Arg {
            let r: Option<Values<'a>> = self.values_of(s);
            return r;
        }
        None
    }
}
