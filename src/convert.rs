use crate::{Filter, Getter};
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Holder<'a, A, B, R, RB> {
    a: A,
    b: B,
    _r: PhantomData<&'a R>, //Just to help lock types
    _rb: PhantomData<RB>,
}

impl<'a, A, RA, B, RB> Holder<'a, A, B, RA, RB>
where
    A: Getter<'a, RA>,
    B: Getter<'a, RB>,
    RA: std::convert::From<RB> + Debug + PartialEq + Display,
    RB: Debug + PartialEq + Display,
{
    pub fn new(a: A, b: B) -> Self {
        Holder {
            a,
            b,
            _r: PhantomData,
            _rb: PhantomData,
        }
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

impl<'a, A, RA, B, RB> Getter<'a, RA> for Holder<'a, A, B, RA, RB>
where
    A: Getter<'a, RA>,
    B: Getter<'a, RB>,
    A::Iter: Iterator<Item = RA>,
    B::Iter: Iterator<Item = RB>,
    RA: PartialEq + Debug + Display,
    RB: PartialEq + Debug + Display,
    RA: std::convert::From<RB>,
{
    type Iter = OrIter<A::Iter, B::Iter>;
    fn bool_flag<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        self.a.bool_flag(s.as_ref(), f) || self.b.bool_flag(s, f)
    }

    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<RA> {
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
pub struct Wrapper<G, F, CR> {
    pub g: G,
    pub f: F,
    _cr: PhantomData<CR>,
}

impl<G, F, CR> Wrapper<G, F, CR> {
    pub fn new(g: G, f: F) -> Self {
        Wrapper {
            g,
            f,
            _cr: PhantomData,
        }
    }
}

impl<'a, G, R, F, CR> Getter<'a, R> for Wrapper<G, F, CR>
where
    G: Getter<'a, CR>,
    F: Fn(CR) -> R + Clone,
    CR: PartialEq + Debug + Display,
    R: PartialEq + Debug + Display,
{
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

impl<'a, R, G> Getter<'a, R> for Localizer<G>
where
    R: PartialEq + Debug + Display,
    G: Getter<'a, R>,
{
    type Iter = G::Iter;
    fn bool_flag<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        self.g.bool_flag(s, f)
    }
    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<R> {
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
