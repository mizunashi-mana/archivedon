use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

#[derive(PartialEq, Eq, Debug)]
pub struct OrderedCollection;

#[derive(PartialEq, Eq, Debug)]
pub enum Collection {
    Collection,
    OrderedCollection,
}

impl Serialize for OrderedCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("OrderedCollection")
    }
}

impl<'de> Deserialize<'de> for OrderedCollection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StrVisitor;
        const FIELDS: &'static [&'static str] = &["OrderedCollection"];

        impl<'de> Visitor<'de> for StrVisitor {
            type Value = OrderedCollection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("`OrderedCollection`")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "OrderedCollection" => Ok(OrderedCollection),
                    _ => Err(de::Error::unknown_field(v, FIELDS)),
                }
            }
        }

        deserializer.deserialize_str(StrVisitor)
    }
}

impl Serialize for Collection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Collection::Collection => serializer.serialize_str("Collection"),
            Collection::OrderedCollection => serializer.serialize_str("OrderedCollection"),
        }
    }
}

impl<'de> Deserialize<'de> for Collection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StrVisitor;
        const FIELDS: &'static [&'static str] = &["Collection", "OrderedCollection"];

        impl<'de> Visitor<'de> for StrVisitor {
            type Value = Collection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("`Collection` or `OrderedCollection`")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "Collection" => Ok(Collection::Collection),
                    "OrderedCollection" => Ok(Collection::OrderedCollection),
                    _ => Err(de::Error::unknown_field(v, FIELDS)),
                }
            }
        }

        deserializer.deserialize_str(StrVisitor)
    }
}
