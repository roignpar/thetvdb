use std::collections::HashMap;

#[derive(Debug)]
pub enum SearchBy<S> {
    Name(S),
    ImdbID(S),
    Zap2itID(S),
    Slug(S),
}

// SearchBy needs to be transformed into a query param and deriving
// Serialize doesn't help:
// "top-level serializer supports only maps and structs";
//
// implementing Into instead of From because clippy incorrectly
// complains: https://github.com/rust-lang/rust-clippy/issues/3899
impl<S, B> Into<HashMap<String, String, B>> for SearchBy<S>
where
    S: Into<String>,
    B: std::hash::BuildHasher + Default,
{
    fn into(self) -> HashMap<String, String, B> {
        use SearchBy::*;

        let mut map = HashMap::default();

        match self {
            Name(name) => map.insert("name".to_string(), name.into()),
            ImdbID(id) => map.insert("imdbId".to_string(), id.into()),
            Zap2itID(id) => map.insert("zap2itId".to_string(), id.into()),
            Slug(slug) => map.insert("slug".to_string(), slug.into()),
        };

        map
    }
}
