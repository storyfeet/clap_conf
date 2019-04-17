use crate::{Filter, Getter};
use clap::{ArgMatches, Values};

fn dig<'a, 'b,'c,IT:Iterator<Item=&'c str>>(m: &'a ArgMatches<'b>,down:&'c str,mut i:IT) -> Option<(&'a ArgMatches<'b>,&'c str)> {
    match i.next(){
        None=>Some((m,down)),
        Some(s)=>
            dig(m.subcommand_matches(s)?,s,i),
        
    }

}

impl<'a, 'b> Getter<&'a str, Values<'a>> for &'a ArgMatches<'b> {
    fn value<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<&'a str> {
        if f != Filter::Arg {
            return None;
        }
        let mut it = s.as_ref().split(".");
        let (r,dot_last) = dig(self,it.next()?,it)?;
        r.value_of(dot_last)
    }

    fn values<S: AsRef<str>>(&self, s: S, f: Filter) -> Option<Values<'a>> {
        if f != Filter::Arg {
            return None;
        }
        let mut it = s.as_ref().split(".");
        let (r,dot_last) = dig(self,it.next()?,it)?;
        r.values_of(dot_last)
    }

    fn sub<S:AsRef<str>>(&self,s:S,f:Filter)->bool{
        if f != Filter::Arg {
            return false;
        }
        let mut it = s.as_ref().split(".");
        let (r,dot_last) = match dig(self,it.next().unwrap(),it){
            Some(v)=>v,
            None=>return false,
        };
        match r.subcommand_matches(dot_last){
            Some(_)=>true,
            None=>false,
        }
    }
}


#[cfg(Test)]
mod tests{
   //TODO add tests involving clap_app::App::get_matches_from 
}
