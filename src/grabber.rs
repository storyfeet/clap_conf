use crate::{Filter,Getter};

#[derive(Clone)]
pub struct Grabber<'a, H:Getter<'a>> {
    h: &'a H,
    res: Option<String>,
}

impl<'a,H:Getter<'a>> Grabber<'a,H>{
    pub fn new(h:&'a H)->Self{
        Grabber{h,res:None}
    }
    
    pub fn op<S:AsRef<str>>(mut self,s:S,f:Filter)->Self{
        if self.res == None{
            self.res = self.h.value(s,f);
        }
        self
    }

    pub fn conf<S:AsRef<str>>(self,s:S)->Self{
        self.op(s,Filter::Conf)
    }

    pub fn env<S:AsRef<str>>(self,s:S)->Self{
        self.op(s,Filter::Env)
    }
    pub fn arg<S:AsRef<str>>(self,s:S)->Self{
        self.op(s,Filter::Arg)
    }
    
    pub fn done(self)->Option<String>{
        self.res
    }
    
}


