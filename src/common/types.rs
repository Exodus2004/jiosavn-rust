use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct DownloadLink {
    pub quality: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}
