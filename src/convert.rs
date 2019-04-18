use crate::{Filter, Getter};
use std::fmt::Debug;

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
    RA: PartialEq + std::fmt::Debug,
    RB: PartialEq + Debug,
    IA: Iterator<Item = RA>,
    IB: Iterator<Item = RB>,
    RA: std::convert::From<RB>,
{
    fn bool_flag<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        self.a.bool_flag(s.as_ref(), f) || self.b.bool_flag(s, f)
    }

    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<RA> {
        self.a
            .value(s.as_ref(), f)
            .or_else(|| self.b.value(s, f).map(|r| r.into()))
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

    fn sub<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        self.a.sub(s.as_ref(), f) || self.b.sub(s, f)
    }
}

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

pub struct Wrapper<G, F> {
    pub g: G,
    pub f: F,
}

impl<G, R, CR, CI, F> Getter<R, ConvIter<CI, F>> for Wrapper<G, F>
where
    G: Getter<CR, CI>,
    CI: Iterator<Item = CR>,
    F: Fn(CR) -> R + Clone,
    CR: PartialEq + Debug,
    R: PartialEq + Debug,
{
    fn bool_flag<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        self.g.bool_flag(s, f)
    }
    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<R> {
        self.g.value(s, f).map(&self.f)
    }

    fn values<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<ConvIter<CI, F>> {
        self.g.values(s, f).map(|i| ConvIter {
            it: i,
            f: self.f.clone(),
        })
    }

    fn sub<S: AsRef<str>>(&self, s: S, f: Filter) -> bool {
        self.g.sub(s, f)
    }
}
