use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct ParsedCommit {
    #[serde(skip_serializing)]
    pub header: String,
    /// A list of gitmoji found on the commit header, serialized as a single element.
    #[serde(
        serialize_with = "serialize_gitmoji",
        skip_serializing_if = "Option::is_none"
    )]
    pub gitmoji: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_breaking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breaking_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
