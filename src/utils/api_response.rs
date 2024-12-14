use serde::Serialize;


#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(messag: &str) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(messag.to_string()),
        }
    }
}
