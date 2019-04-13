pub mod clapget;
pub mod env;
pub mod grabber;
pub mod tomlget;

use std::fmt::Debug;
pub use clap::{clap_app, crate_version, ArgMatches, Values};

pub fn clap_env<'a,'b>(a: &'b ArgMatches<'a>) -> Holder<env::Enver,&'b ArgMatches<'a>, String, &'b str> {
    env::Enver{}.hold(a)
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
    R:PartialEq+std::fmt::Debug,
{
    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<R>;
    fn values<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<IT>;

    fn hold<B, RB, ITB>(self, b: B) -> Holder<Self, B, R, RB>
    where
        B: Getter<RB, ITB>,
        R: std::convert::From<RB>,
        RB:PartialEq+std::fmt::Debug,
        ITB: Iterator<Item = RB>,
    {
        Holder::new(self,b)
    }

    fn grab<'a>(&'a self) -> grabber::Grabber<'a, Self, R, IT>
    {
        grabber::Grabber::new(self)
    }
}

pub struct Holder<A, B, R, RB> {
    a: A,
    b: B,
    _r: Option<R>, //Just to help lock types
    _rb: Option<RB>,
}

impl<A, RA, B, RB> Holder<A, B, RA, RB>
where
    RA: std::convert::From<RB>,
{
    pub fn new(a: A, b: B) -> Self {
        Holder {
            a,
            b,
            _r: None,
            _rb: None,
        }
    }
}

pub enum OrIter<A: Iterator, B: Iterator> {
    A(A),
    B(B),
}

impl<R, R2, A, B> Iterator for OrIter<A, B>
where
    A: Iterator<Item = R>,
    B: Iterator<Item = R2>,
    R: std::convert::From<R2>,
{
    type Item = R;
    fn next(&mut self) -> Option<R> {
        match self {
            OrIter::A(a) => a.next(),
            OrIter::B(b) => b.next().map(|x| x.into()),
        }
    }
}

impl<A, RA, IA, B, RB, IB> Getter<RA, OrIter<IA, IB>> for Holder<A, B, RA, RB>
where
    A: Getter<RA, IA>,
    B: Getter<RB, IB>,
    RA:PartialEq+std::fmt::Debug,
    RB:PartialEq+Debug,
    IA: Iterator<Item = RA>,
    IB: Iterator<Item = RB>,
    RA: std::convert::From<RB>,
{
    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<RA> {
        self.a
            .value(s.as_ref(), f)
            .or_else(|| self.b.value(s, f).map(|r|r.into()))
    }

    fn values<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<OrIter<IA, IB>> {
        if let Some(r) = self.a.values(s.as_ref(), f) {
            return Some(OrIter::A(r.into_iter()));
        }
        if let Some(r) = self.b.values(s.as_ref(), f) {
            return Some(OrIter::B(r.into_iter()));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_holder() {
        let a = ArgMatches::new();
       // let tml:toml::Value = "[a]\ncar=\"red\"".parse().unwrap();
        let ce = clap_env(&a);//.hold(tml);

        assert_eq!(ce.value("ss", Filter::Arg), None);
        assert_eq!(
            ce.value("PWD", Filter::Env),
            Some("/home/matthew/scripts/rust/mlibs/clap_conf".to_string())
        );
        

        assert_eq!(ce.grab().env("PWD").done(),Some("/home/matthew/scripts/rust/mlibs/clap_conf".to_string()));


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
