pub mod clapget;
pub mod convert;
pub mod env;
pub mod grabber;
pub mod prelude;
pub mod replace;
pub mod tomlget;

pub use clap::{clap_app, crate_version, ArgMatches, Values};

pub fn clap_env<'a>(a: &'a ArgMatches<'a>) -> impl Getter<'a,String>{
    //a.wrap(|v|v.to_string()).hold(env::Enver{})
    env::Enver{}.hold(a)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Filter {
    Conf,
    Arg,
    Env,
    Other(char),
}

pub trait Getter<'a,R>: Sized
where
    R: PartialEq + std::fmt::Debug,
{
    type Iter: Iterator<Item = R>;
    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<R>;
    fn values<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<Self::Iter>;

    fn bool_flag<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        self.value(s, f).is_some()
    }

    fn sub<S: AsRef<str>>(&self, _: S, _: Filter) -> bool {
        return false;
    }

    fn wrap<R2, F: Fn(R) -> R2>(self, f: F) -> convert::Wrapper<Self, F,R> {
        convert::Wrapper::new(self,f)
    }

    fn hold<B, RB>(self, b: B) -> convert::Holder<'a,Self, B, R, RB>
    where
        B: Getter<'a,RB>,
        R: std::convert::From<RB>,
        RB: PartialEq + std::fmt::Debug,
        B::Iter: Iterator<Item = RB>,
    {
        convert::Holder::new(self, b)
    }

    fn grab(&'a self) -> grabber::Grabber<'a, Self, R, Self::Iter> {
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
        //let ce = clap_env(&a).hold(tml2);
        let e = env::Enver{};
        let ce = e.hold(&a).hold(tml2);

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
