#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot find file")]
    FileNotFound,

    #[error("invalid config: {0}")]
    InvalidConfig(&'static str),

    #[error("invalid json: {0}")]
    JSON(#[from] serde_json::Error),

    #[error("invalid yaml: {0}")]
    YAML(#[from] serde_yaml::Error),

    #[error("FileSystem error")]
    FileSystem(#[from] std::io::Error),

    #[error("the theme you are looking for does not exists")]
    ThemeNotFound,

    #[error("{0}")]
    Custom(&'static str),

    #[error("something went wrong: {0}")]
    Generic(#[from] anyhow::Error),
}

#[allow(clippy::from_over_into)]
impl Into<String> for Error {
    fn into(self) -> String {
        self.to_string()
    }
}
