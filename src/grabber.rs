use std::path::PathBuf;
use std::str::FromStr;

use crate::replace::{replace_env, ConfError};
use crate::{Filter, Getter};

#[derive(Clone, Debug)]
pub struct Grabber<'a, H>
where
    H: Getter<'a>,
{
    h: &'a H,
    res: Option<H::Out>,
}

impl<'a, H> Grabber<'a, H>
where
    H: Getter<'a>,
{
    pub fn new(h: &'a H) -> Self {
        Grabber { h, res: None }
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

    pub fn done(self) -> Option<H::Out> {
        self.res
    }

    pub fn def<V>(self, v: V) -> H::Out
    where
        H::Out: From<V>,
    {
        self.res.unwrap_or(v.into())
    }

    pub fn req(self) -> Result<H::Out, ConfError> {
        self.res.ok_or("Item not supplied".into())
    }
}

impl<'a, H> Grabber<'a, H>
where
    H: Getter<'a>,
    H::Out: AsRef<str>,
{
    pub fn t_done<T: FromStr>(self) -> Option<T> {
        match self.res {
            Some(r) => r.as_ref().parse().ok(),
            None => None,
        }
    }
    pub fn t_def<T: FromStr>(self, def: T) -> T {
        match self.res {
            Some(r) => r.as_ref().parse().unwrap_or(def),
            None => def,
        }
    }
}
impl<'a, H> Grabber<'a, H>
where
    H: Getter<'a>,
    H::Out: AsRef<str>,
{
    pub fn rep_env(self) -> Result<String, ConfError> {
        replace_env(self.res.ok_or("No Res")?.as_ref())
    }

    pub fn ask<S: AsRef<str>>(self, s: S) -> Result<String, ConfError> {
        if let Some(r) = self.res {
            return Ok(r.as_ref().to_string());
        }
        println!("{}\n>", s.as_ref());
        let mut res = String::new();
        std::io::stdin().read_line(&mut res)?;
        Ok(res)
    }

    pub fn ask_def<S: AsRef<str>>(self, s: S, def: S) -> String {
        match self.ask(s) {
            Ok(r) => {
                if r == "".to_string() {
                    return def.as_ref().to_string();
                }
                r
            }
            Err(_) => def.as_ref().to_string(),
        }
    }
}

pub struct LocalGrabber<'a, G>
where
    G: Getter<'a>,
{
    g: &'a G,
    res: Option<PathBuf>,
}

impl<'a, G> LocalGrabber<'a, G>
where
    G: Getter<'a>,
{
    pub fn new(g: &'a G) -> Self {
        LocalGrabber { g, res: None }
    }

    pub fn op<S: AsRef<str>>(mut self, s: S, f: Filter) -> Self {
        if self.res == None {
            self.res = self.g.local_value(s, f);
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

    pub fn done(self) -> Option<PathBuf> {
        self.res
    }

    pub fn req(self) -> Result<PathBuf, ConfError> {
        self.res.ok_or("Item not supplied".into())
    }

    pub fn def<V>(self, v: V) -> PathBuf
    where
        PathBuf: From<V>,
    {
        self.res.unwrap_or(v.into())
    }
}

pub struct MultiGrabber<'a, G>
where
    G: Getter<'a>,
{
    g: &'a G,
    res: Option<G::Iter>,
    //_i: PhantomData<I>,
}

impl<'a, G> MultiGrabber<'a, G>
where
    G: Getter<'a>,
{
    pub fn new(g: &'a G) -> Self {
        MultiGrabber { g, res: None }
    }

    pub fn op<S: AsRef<str>>(mut self, s: S, f: Filter) -> Self {
        if let None = self.res {
            self.res = self.g.values(s, f);
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

    pub fn done(self) -> Option<G::Iter> {
        self.res
    }

    pub fn req(self) -> Result<G::Iter, ConfError> {
        self.res.ok_or("Item not supplied".into())
    }
}
