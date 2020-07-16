use crate::{Filter, Getter};
use std::fmt::{Debug, Display};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Holder<A, B> {
    a: A,
    b: B,
}

impl<'a, A, B> Holder<A, B>
where
    A: Getter<'a>,
    B: Getter<'a>,
    A::Out: std::convert::From<B::Out> + Debug + PartialEq + Display,
{
    pub fn new(a: A, b: B) -> Self {
        Holder { a, b }
    }
}

#[derive(Debug)]
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

impl<'a, A, B> Getter<'a> for Holder<A, B>
where
    A: Getter<'a>,
    B: Getter<'a>,
    A::Out: std::convert::From<B::Out>,
{
    type Out = A::Out;
    type Iter = OrIter<A::Iter, B::Iter>;
    fn bool_flag<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        self.a.bool_flag(s.as_ref(), f) || self.b.bool_flag(s, f)
    }

    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<A::Out> {
        self.a
            .value(s.as_ref(), f)
            .or_else(|| self.b.value(s, f).map(|r| r.into()))
    }

    fn values<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<OrIter<A::Iter, B::Iter>> {
        if let Some(r) = self.a.values(s.as_ref(), f) {
            return Some(OrIter::A(r.into_iter()));
        }
        if let Some(r) = self.b.values(s.as_ref(), f) {
            return Some(OrIter::B(r.into_iter()));
        }
        None
    }

    fn local_value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<PathBuf> {
        self.a
            .local_value(s.as_ref(), f)
            .or_else(|| self.b.local_value(s, f).map(|r| r.into()))
    }

    fn sub<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        self.a.sub(s.as_ref(), f) || self.b.sub(s, f)
    }
}

#[derive(Debug)]
pub struct ConvIter<I, F> {
    it: I,
    f: F,
}

impl<R, R2, I, F> Iterator for ConvIter<I, F>
where
    I: Iterator<Item = R>,
    F: Fn(R) -> R2,
{
    type Item = R2;
    fn next(&mut self) -> Option<R2> {
        self.it.next().map(&self.f)
    }
}

#[derive(Debug)]
pub struct Wrapper<G, F> {
    pub g: G,
    pub f: F,
}

impl<G, F> Wrapper<G, F> {
    pub fn new(g: G, f: F) -> Self {
        Wrapper { g, f }
    }
}

impl<'a, G, R, F> Getter<'a> for Wrapper<G, F>
where
    G: Getter<'a>,
    F: Fn(G::Out) -> R + Clone,
    G::Out: PartialEq + Debug + Display,
    R: PartialEq + Debug + Display,
{
    type Out = R;
    type Iter = ConvIter<G::Iter, F>;
    fn bool_flag<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        self.g.bool_flag(s, f)
    }
    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<R> {
        self.g.value(s, f).map(&self.f)
    }

    fn values<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<Self::Iter> {
        Some(ConvIter {
            it: self.g.values(s, f)?,
            f: self.f.clone(),
        })
    }

    fn local_value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<PathBuf> {
        self.g.local_value(s, f)
    }

    fn sub<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        self.g.sub(s, f)
    }
}

#[derive(Debug)]
pub struct Localizer<G> {
    local: PathBuf,
    g: G,
}

impl<G> Localizer<G> {
    pub fn new<P>(g: G, p: P) -> Self
    where
        PathBuf: From<P>,
    {
        Localizer {
            g,
            local: PathBuf::from(p),
        }
    }
}

impl<'a, G> Getter<'a> for Localizer<G>
where
    G: Getter<'a>,
{
    type Out = G::Out;
    type Iter = G::Iter;
    fn bool_flag<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        self.g.bool_flag(s, f)
    }
    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<G::Out> {
        self.g.value(s, f)
    }

    fn values<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<Self::Iter> {
        self.g.values(s, f)
    }

    fn sub<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        self.g.sub(s, f)
    }

    fn local_value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<PathBuf> {
        self.g.local_value(s, f).map(|iv| match iv.is_absolute() {
            true => iv,
            false => self.local.clone().join(iv),
        })
    }
}
