use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct ParsedCommit {
    #[serde(skip_serializing)]
    pub header: String,
    /// A list of gitmoji found on the commit header, serialized as a single element.
    #[serde(serialize_with = "serialize_gitmoji")]
    pub gitmoji: Option<Vec<String>>,
    pub commit_type: Option<String>,
    pub scope: Option<String>,
    pub description: String,
    pub body: Option<String>,
    pub footers: Option<Vec<String>>,
    pub is_breaking: Option<bool>,
    pub breaking_description: Option<String>,
    pub references: Option<Vec<String>>,
}

fn serialize_gitmoji<S>(gitmoji: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(gitmoji_vec) = gitmoji {
        if gitmoji_vec.len() == 1 {
            return serializer.serialize_some(&gitmoji_vec[0]);
        }
    }
    serializer.serialize_none()
}
