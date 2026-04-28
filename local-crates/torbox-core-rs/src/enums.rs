use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for OneOrMany<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Inner<T> {
            Many(Vec<T>),
            One(T),
        }

        Inner::deserialize(deserializer).map(|inner| match inner {
            Inner::Many(v) => OneOrMany::Many(v),
            Inner::One(v) => OneOrMany::One(v),
        })
    }
}
