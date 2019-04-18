use crate::replace::{replace_env, ConfError};
use crate::{Filter, Getter};

#[derive(Clone)]
pub struct Grabber<'a, H, R, I>
where
    I: Iterator<Item = R>,
    H: Getter<'a,R>,
    R: PartialEq + std::fmt::Debug,
{
    h: &'a H,
    res: Option<R>,
    _i: Option<I>,
}

impl<'a, H, R, I> Grabber<'a, H, R, I>
where
    I: Iterator<Item = R>,
    H: Getter<'a,R>,
    R: PartialEq + std::fmt::Debug,
{
    pub fn new(h: &'a H) -> Self {
        Grabber {
            h,
            res: None,
            _i: None,
        }
    }

    pub fn op<S: AsRef<str>>(mut self, s: S, f: Filter) -> Self {
        println!("Getting {:?}, res = {:?}", f, self.res);
        if self.res == None {
            self.res = self.h.value(s, f);
            println!("Setting {:?}, res = {:?}", f, self.res);
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

    pub fn done(self) -> Option<R> {
        self.res
    }
}

impl<'a, H, R, I> Grabber<'a, H, R, I>
where
    I: Iterator<Item = R>,
    H: Getter<'a, R>,
    R: PartialEq + std::fmt::Debug + AsRef<str>,
{
    pub fn rep_env(self) -> Result<String, ConfError> {
        replace_env(self.res.ok_or("No Res")?.as_ref())
    }
}
