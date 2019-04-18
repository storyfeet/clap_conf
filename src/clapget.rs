use crate::{Filter, Getter};
use clap::{ArgMatches, Values};

fn _dig<'a, 'b, 'c, IT: Iterator<Item = &'c str>>(
    m: &'a ArgMatches<'b>,
    down: &'c str,
    mut i: IT,
) -> Option<(&'a ArgMatches<'b>, &'c str)> {
    match i.next() {
        None => Some((m, down)),
        Some(s) => _dig(m.subcommand_matches(down)?, s, i),
    }
}

fn dig<'a, 'b, 'c>(m: &'a ArgMatches<'b>, s: &'c str) -> Option<(&'a ArgMatches<'b>, &'c str)> {
    let mut it = s.split(".");
    _dig(m, it.next()?, it)
}

impl<'a, 'b> Getter<'a,&'a str> for &'a ArgMatches<'b> {
    type Iter = Values<'a>;
    fn bool_flag<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        if f != Filter::Arg {
            return false;
        }
        let (r, dot_last) = match dig(self, s.as_ref()) {
            Some(v) => v,
            None => return false,
        };
        r.is_present(dot_last)
    }

    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<&'a str> {
        if f != Filter::Arg {
            return None;
        }
        let (r, dot_last) = dig(self, s.as_ref())?;
        r.value_of(dot_last)
    }

    fn values<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<Values<'a>> {
        if f != Filter::Arg {
            return None;
        }
        let (r, dot_last) = dig(self, s.as_ref())?;
        r.values_of(dot_last)
    }

    fn sub<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        if f != Filter::Arg {
            return false;
        }
        let (r, dot_last) = match dig(self, s.as_ref()) {
            Some(v) => v,
            None => return false,
        };
        match r.subcommand_matches(dot_last) {
            Some(_) => true,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::clap_app;
    //TODO add tests involving clap_app::App::get_matches_from
    #[test]
    pub fn test_sub_get() {
        let m = clap_app!(
            test_app=>
                (@arg a : -a +takes_value "astuff")
                (@subcommand subby =>
                    (@arg b: -b +takes_value "bstuff")
                        )

        )
        .get_matches_from("test_app -a hi subby -b world".split(" "));

        let mm = &m;

        assert_eq!(mm.sub("a", Filter::Arg), false, "A");
        assert_eq!(mm.grab().arg("a").done(), Some("hi"), "HI");
        assert_eq!(mm.sub("subby", Filter::Arg), true, "--Sub Subby--");
        assert_eq!(mm.grab().arg("subby.b").done(), Some("world"), "C");
        assert_eq!(mm.bool_flag("a", Filter::Arg), true);
    }
}
