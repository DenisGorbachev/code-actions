/// Custom serde module for serializing/deserializing Vec<syn::Path> as Vec<String>
use quote;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use syn::Path as SynPath;

pub fn serialize<S>(paths: &[SynPath], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let strings: Vec<String> = paths
        .iter()
        .map(|path| quote::quote!(#path).to_string())
        .collect();
    strings.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<SynPath>, D::Error>
where
    D: Deserializer<'de>,
{
    let strings: Vec<String> = Vec::deserialize(deserializer)?;
    strings
        .into_iter()
        .map(|s| syn::parse_str(&s).map_err(serde::de::Error::custom))
        .collect()
}
