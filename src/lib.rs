pub mod clapget;
pub mod env;

pub use clap::{clap_app,crate_version,ArgMatches, Values};

pub fn clap_env<'a>(a:&'a ArgMatches<'a>)->Holder<&'a ArgMatches<'a>,env::Enver>{
    Holder::new(a,env::Enver{})
}


#[derive(Clone, Copy, Debug,PartialEq)]
pub enum Filter {
    Conf,
    Arg,
    Env,
    Other(char),
}

pub trait Getter<'a> {
    type Iter;
    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<String>;
    fn values<S: AsRef<str>>(&'a self, s: S, f: Filter) -> Option<Self::Iter>;
}

pub struct Holder<'a,A, B> {
    a: &'a A,
    b: &'a B,
}

impl<'a,A:Getter<'a>, B:Getter<'a>> Holder<'a, A, B> 
    where A::Iter:Iterator,
    B::Iter:Iterator,
{
    pub fn new(a: &'a A, b: &'a B) -> Holder<'a,A, B> {
        Holder { a, b }
    }
    pub fn hold<C>(&'a self, c: &'a C) -> Holder<'a, Holder<'a,A, B>, C> 
        where
            C:Getter<'a>,
            C::Iter:Iterator<Item=String>,
    {
        Holder::new(self, c)
    }
}

pub enum OrIter<A: Iterator, B: Iterator> {
    A(A),
    B(B),
}

impl<R,R2, A, B> Iterator for OrIter<A, B>
where
    A: Iterator<Item = R>,
    B: Iterator<Item = R2>,
    R: std::fmt::Display,
    R2:std::fmt::Display,

{
    type Item = String;
    fn next(&mut self) -> Option<String> {
        match self {
            OrIter::A(a) => a.next().map(|x|x.to_string()),
            OrIter::B(b) => b.next().map(|x|x.to_string()),
        }
    }
}

impl<'a, R, A: Getter<'a>, B: Getter<'a>> Getter<'a> for Holder<'a,A, B>
where
    A::Iter: Iterator<Item = R>,
    B::Iter: Iterator<Item = R>,
{
    type Iter = OrIter<A::Iter, B::Iter>;

    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<String> {
        self.a
            .value(s.as_ref(), f)
            .or_else(|| self.b.value(s.as_ref(), f))
    }

    fn values<S: AsRef<str>>(&'a self, s: S, f: Filter) -> Option<Self::Iter> {
        if let Some(r) = self.a.values(s.as_ref(), f) {
            return Some(OrIter::A(r));
        }
        if let Some(r) = self.b.values(s.as_ref(), f) {
            return Some(OrIter::B(r));
        }
        None
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn try_holder(){
        let a = ArgMatches::new();
        let ce = clap_env(&a);
        assert_eq!(ce.value("ss",Filter::Arg),None);
    }
}
