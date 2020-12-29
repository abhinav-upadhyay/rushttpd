use std::collections::HashMap;
use std::convert::From;

#[derive(Debug)]
pub struct QueryString<'a> {
    data: HashMap<&'a str, Value<'a>>
}

impl<'a> QueryString<'a> {
    pub fn get(&self, key: &str) -> Option<&'a Value> {
        self.data.get(key)
    }
}

impl<'a> From<&'a str> for QueryString<'a> {
    fn from(s: &'a str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..]
            }
            data.entry(key)
            .and_modify(|existing: &mut Value| match existing {
                Value::Single(string_val) => {
                    *existing = Value::Multiple(vec![string_val, val]);
                },
                Value::Multiple(vec_val) => vec_val.push(val)
            })
            .or_insert(Value::Single(val));
            
        }
        QueryString{data}
    }
} 

#[derive(Debug)]
pub enum Value<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>)
}