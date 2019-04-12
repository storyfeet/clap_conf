pub mod clapget;
pub mod env;

pub use clap::{clap_app, crate_version, ArgMatches, Values};

pub fn clap_env<'a>(a: &'a ArgMatches<'a>) -> Holder<env::Enver,ArgMatches<'a>,  String> {
    Holder::new(&env::EV,a)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Filter {
    Conf,
    Arg,
    Env,
    Other(char),
}

pub trait Getter<'a> {
    type Iter: IntoIterator;
    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<String>;
    fn values<S: AsRef<str>>(&'a self, s: S, f: Filter) -> Option<Self::Iter>;
}

pub struct Holder<'a, A, B, R> {
    a: &'a A,
    b: &'a B,
    _r: Option<R>, //Just to help lock types
}

impl<'a, A: Getter<'a>, B: Getter<'a>, R> Holder<'a, A, B, R>
where
    A::Iter: IntoIterator,
    B::Iter: IntoIterator,
{
    pub fn new(a: &'a A, b: &'a B) -> Holder<'a, A, B, R> {
        Holder { a, b, _r: None }
    }

    pub fn hold<C, R2>(&'a self, c: &'a C) -> Holder<'a, Holder<'a, A, B, R>, C, R>
    where
        Self: Getter<'a>,
        //Self::Iter: Iterator<Item = R>,
        C: Getter<'a>,
        C::Iter: Iterator<Item = R2>,
        R: std::convert::From<R2>,
    {
        Holder::new(self, c)
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

impl<'a, R, R2,N1,N2, A: Getter<'a>, B: Getter<'a>> Getter<'a> for Holder<'a, A, B, R>
where
    A::Iter: IntoIterator<IntoIter = N1,Item=R>,
    B::Iter: IntoIterator<IntoIter = N2,Item=R2>,
    N1:Iterator<Item = R>,
    N2:Iterator<Item = R2>,
    R: std::convert::From<R2>,
{
    type Iter = OrIter<N1,N2>;

    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<String> {
        self.a
            .value(s.as_ref(), f)
            .or_else(|| self.b.value(s.as_ref(), f))
    }

    fn values<S: AsRef<str>>(&'a self, s: S, f: Filter) -> Option<Self::Iter> {
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
        let ce = clap_env(&a);
        assert_eq!(ce.value("ss", Filter::Arg), None);
        //assert_eq!(ce.value("PWD",Filter::Env),Some("/home/matthew".to_string()));
    }
}