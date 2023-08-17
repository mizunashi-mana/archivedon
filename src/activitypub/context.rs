use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/**
 * Schema: https://www.w3.org/TR/json-ld/#the-context
 */
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum Context {
    Single(Iri),
    Mix(Vec<Context>),
    TermDefs(HashMap<String, Iri>),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum Iri {
    Direct(String),
    TypeCoercion(TypeCoercion),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TypeCoercion {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub typ: Option<String>,
}

impl From<&str> for Context {
    fn from(value: &str) -> Self {
        Self::Single(Iri::from(value))
    }
}

impl<const N: usize> From<[(&str, Iri); N]> for Context {
    fn from(value: [(&str, Iri); N]) -> Self {
        let defs = HashMap::from_iter(
            value
                .into_iter()
                .map(|entry| (entry.0.to_string(), entry.1)),
        );
        Self::TermDefs(defs)
    }
}

impl From<&str> for Iri {
    fn from(value: &str) -> Self {
        Self::Direct(value.to_string())
    }
}

impl From<TypeCoercion> for Iri {
    fn from(value: TypeCoercion) -> Self {
        Self::TypeCoercion(value)
    }
}
