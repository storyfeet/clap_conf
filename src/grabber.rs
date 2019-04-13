use crate::{Filter, Getter};

#[derive(Clone)]
pub struct Grabber<'a, H, R, I>
where
    I: Iterator<Item = R>,
    H: Getter<R, I>,
{
    h: &'a H,
    res: Option<R>,
    _i: Option<I>,
}

impl<'a, H, R, I> Grabber<'a, H, R, I>
where
    I: Iterator<Item = R>,
    H: Getter<R, I>,
{
    pub fn new(h: &'a H) -> Self {
        Grabber {
            h,
            res: None,
            _i: None,
        }
    }

    pub fn op<S: AsRef<str>>(mut self, s: S, f: Filter) -> Self {
        if self.res == None {
            self.res = self.h.value(s, f);
        }
        self
    }

    pub fn conf<S: AsRef<str>>(self, s: S) -> Self {
        self.op(s, Filter::Conf)
    }

    pub fn env<S: AsRef<str>>(self, s: S) -> Self {
        self.op(s, Filter::Env)
    }
    pub fn arg<S: AsRef<str>>(self, s: S) -> Self {
        self.op(s, Filter::Arg)
    }

    pub fn done(self) -> Option<String> {
        self.res
    }
}
