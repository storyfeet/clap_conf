pub mod clapget;
pub mod convert;
pub mod env;
pub mod grabber;
pub mod replace;
pub mod tomlget;

pub use clap::{clap_app, crate_version, ArgMatches, Values};

pub fn clap_env<'a, 'b>(
    a: &'b ArgMatches<'a>,
) -> convert::Holder<env::Enver, &'b ArgMatches<'a>, String, &'b str> {
    env::Enver {}.hold(a)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Filter {
    Conf,
    Arg,
    Env,
    Other(char),
}

pub trait Getter<R, IT>: Sized
where
    IT: Iterator<Item = R>,
    R: PartialEq + std::fmt::Debug,
{
    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<R>;
    fn values<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<IT>;

    fn sub<S: AsRef<str>>(&self, _: S, _: Filter) -> bool {
        return false;
    }

    fn wrap<R2, F: Fn(R) -> R2>(self, f: F) -> convert::Wrapper<Self, F> {
        convert::Wrapper { g: self, f }
    }

    fn hold<B, RB, ITB>(self, b: B) -> convert::Holder<Self, B, R, RB>
    where
        B: Getter<RB, ITB>,
        R: std::convert::From<RB>,
        RB: PartialEq + std::fmt::Debug,
        ITB: Iterator<Item = RB>,
    {
        convert::Holder::new(self, b)
    }

    fn grab<'a>(&'a self) -> grabber::Grabber<'a, Self, R, IT> {
        grabber::Grabber::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_holder() {
        let a = ArgMatches::new();
        let tml: toml::Value = "[a]\ncar=\"red\"".parse().unwrap();
        let tml2 = (&tml).wrap(|r| r.as_str().unwrap());
        let ce = clap_env(&a).hold(tml2);

        assert_eq!(ce.value("ss", Filter::Arg), None);
        assert_eq!(
            ce.value("PWD", Filter::Env),
            Some("/home/matthew/scripts/rust/mlibs/clap_conf".to_string())
        );

        assert_eq!(
            ce.grab().env("PWD").done(),
            Some("/home/matthew/scripts/rust/mlibs/clap_conf".to_string())
        );

        assert_eq!(ce.grab().conf("a.car").done(), Some("red".to_string()));

        /* assert_eq!(
            g.env("PWD").done(),
            Some("/home/matthew/scripts/rust/mlibs/clap_conf")
        );
        */
    }

    /*#[test]
    fn test_grab() {
        let a = ArgMatches::new();
        let tml: toml::Value = "a=\"cat\"".parse().unwrap();
        //let ce = clap_env(&a).hold(tml);

        //assert_eq!(ce.grab().conf("a").done(), Some("cat"));
    }*/
}
