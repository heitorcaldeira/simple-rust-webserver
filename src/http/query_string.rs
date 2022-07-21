use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub in s.split('&') {
            let mut key = sub;
            let mut value = "";

            if let Some(i) = sub.find('=') {
                key = &sub[..i];
                value = &sub[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing| match existing {
                    Value::Single(prev) => {
                        *existing = Value::Multiple(vec![prev, value]);
                    }
                    Value::Multiple(vec) => {
                        vec.push(value);
                    }
                })
                .or_insert(Value::Single(value));
        }

        QueryString { data }
    }
}
