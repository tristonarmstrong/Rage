use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    SINGLE(&'buf str),
    MULTIPLE(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// From is used when the conversion cannot fail
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(value: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in value.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing| match existing {
                    Value::SINGLE(prev) => {
                        *existing = Value::MULTIPLE(vec![prev, val]);
                    }
                    Value::MULTIPLE(vec) => vec.push(val),
                })
                .or_insert(Value::SINGLE(val));
        }
        QueryString { data }
    }
}
