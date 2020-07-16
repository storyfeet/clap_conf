//! Clap Conf
//! =========
//!
//! Use this library to unify how you get configuration options from
//!
//! * The command line arguments
//! * Config Files
//! * Environmant Variables
//!
//!
//! Basic Usage
//! ----------
//!
//! ```
//! use clap_conf::prelude::*;
//!
//! let matches = clap_app!(my_app=>
//!     (@arg filename:-f +takes_value "the input filename")
//!     //...
//! ).get_matches();
//!
//! let cfg = with_toml_env(&matches,&["toml/config/locations"]);
//!
//! //the result must be a String as std::env::var has to return a String not a pointer
//! let filename =
//! cfg.grab().arg("filename").conf("input.filename").env("MY_APP_INPUT_FILE").def("default.file");
//!
//! //if the arguments were supplied this would return something else.
//! assert_eq!(filename,"default.file".to_string());
//!
//! ```

pub mod clapget;
pub mod convert;
pub mod env;
pub mod grabber;
pub mod prelude;
pub mod replace;
pub mod tomlget;

use crate::convert::Holder;
use crate::convert::Localizer;
use crate::replace::replace_env;
use std::path::PathBuf;

pub use clap::{clap_app, crate_version, ArgMatches, Values};

pub fn clap_env<'a, G: Getter<'a>>(a: G) -> Holder<env::Enver, G>
where
    String: From<G::Out>,
{
    //a.wrap(|v|v.to_string()).hold(env::Enver{})
    env::Enver {}.hold(a)
}

pub fn with_toml_env<'a, G, S, IT>(
    a: G,
    it: IT,
) -> Holder<Holder<env::Enver, G>, Localizer<toml::Value>>
where
    G: Getter<'a, Out = &'a str>,
    S: AsRef<str>,
    IT: IntoIterator<Item = S>,
{
    let tml = tomlget::load_first_toml(a.value("config", Filter::Arg), it)
        .unwrap_or(Localizer::new(toml::Value::Boolean(false), ""));
    env::Enver {}.hold(a).hold(tml)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Filter {
    Conf,
    Arg,
    Env,
    Other(char),
}

pub trait Getter<'a>: Sized {
    type Iter: Iterator<Item = Self::Out>;
    type Out: PartialEq + std::fmt::Debug + std::fmt::Display;
    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<Self::Out>;
    fn values<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<Self::Iter>;

    fn local_value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<PathBuf> {
        let v = self.value(s, f)?;
        let s = replace_env(&v.to_string()).ok()?;
        Some(PathBuf::from(s))
    }

    fn bool_flag<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        self.value(s, f).is_some()
    }

    fn sub<S: AsRef<str>>(&self, _: S, _: Filter) -> bool {
        return false;
    }

    fn wrap<R2, F: Fn(Self::Out) -> R2>(self, f: F) -> convert::Wrapper<Self, F> {
        convert::Wrapper::new(self, f)
    }

    fn hold<B>(self, b: B) -> convert::Holder<Self, B>
    where
        B: Getter<'a>,
        Self::Out: std::convert::From<B::Out>,
        B::Out: PartialEq + std::fmt::Debug + std::fmt::Display,
    {
        convert::Holder::new(self, b)
    }

    fn grab(&'a self) -> grabber::Grabber<'a, Self> {
        grabber::Grabber::new(self)
    }

    fn grab_local(&'a self) -> grabber::LocalGrabber<'a, Self> {
        grabber::LocalGrabber::new(self)
    }

    fn grab_multi(&'a self) -> grabber::MultiGrabber<'a, Self> {
        grabber::MultiGrabber::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_holder() {
        let a = ArgMatches::new();
        let tml: toml::Value = "[a]\ncar=\"red\"".parse().unwrap();
        //        let tml2 = (&tml).wrap(|r| r.as_str().unwrap());
        let ce = clap_env(&a).hold(tml);
        //let e = env::Enver {};
        //let ce = e.hold(&a).hold(tml);

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

    #[test]
    fn test_grab() {
        let a = ArgMatches::new();
        let r = with_toml_env(&a, &["test_data/test1.toml"]);
        assert_eq!(r.grab().conf("a.b.c").done(), Some("hello".to_string()));
    }
}
