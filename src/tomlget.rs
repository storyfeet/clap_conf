
use crate::{Getter,Filter};
use toml::Value;

impl<'a> Getter<'a> for Value{
    type Iter=std::slice::Iter<'a,Value>;
    fn value<S:AsRef<str>>(&self,s:S,f:Filter)->Option<String>{
        if f != Filter::Conf {
            return None;
        }
        match self {
            Value::Table(t)=>{
                t.get(s.as_ref()).and_then(|v|v.as_str()).map(|v|v.to_string())
            }
            _=>{None}
        }
    }

    fn values<S:AsRef<str>>(&'a self,s:S,f:Filter)->Option<Self::Iter>{
        if f != Filter::Conf {
            return None;
        }
        let t = if let Value::Table(t)= self{t} else {return None};
        let v = t.get(s.as_ref())?;

        if let Value::Array(a) = v {
            return Some(a.into_iter());
        }
        None
    }

}

