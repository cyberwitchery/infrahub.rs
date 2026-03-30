//! file upload types for the graphql multipart request spec
//!
//! used with [`CoreFileObject`](https://docs.infrahub.app) mutations that accept
//! an `Upload` scalar argument.

/// a file to upload via graphql multipart request
#[derive(Debug, Clone)]
pub struct FileUpload {
    /// file name (e.g. `"report.pdf"`)
    pub filename: String,
    /// mime type (e.g. `"application/pdf"`)
    pub content_type: String,
    /// raw file bytes
    pub data: Vec<u8>,
}

impl FileUpload {
    /// create a new file upload
    pub fn new(
        filename: impl Into<String>,
        content_type: impl Into<String>,
        data: Vec<u8>,
    ) -> Self {
        Self {
            filename: filename.into(),
            content_type: content_type.into(),
            data,
        }
    }
}
