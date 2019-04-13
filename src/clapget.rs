use crate::{Filter, Getter};
use clap::{ArgMatches, Values};

impl<'a,'b> Getter<&'a str, Values<'a>> for &'a ArgMatches<'b> {

    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<&'a str> {
        if f == Filter::Arg {
            return self.value_of(s.as_ref());
        }
        None
    }

    fn values<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<Values<'a>> {
        if f == Filter::Arg {
            let r: Option<Values<'a>> = self.values_of(s);
            return r;
        }
        None
    }
}
