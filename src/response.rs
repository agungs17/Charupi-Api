use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

/// Format standar respons API Charupi
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T = ()> {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<ValidationErrorDetail>>,
}

/// Metadata navigasi untuk respons bertipe daftar (list)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginationMeta {
    pub page: u64,
    pub limit: u64,
    pub total: u64,
}

/// Rincian kesalahan validasi per-field (HTTP 422)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidationErrorDetail {
    pub field: String,
    pub message: String,
}

impl<T> ApiResponse<T> {
    /// 200 OK dengan payload data tunggal
    pub fn ok(message: impl Into<String>, data: T) -> (StatusCode, Json<Self>) {
        (
            StatusCode::OK,
            Json(Self {
                success: true,
                message: message.into(),
                data: Some(data),
                meta: None,
                code: None,
                details: None,
            }),
        )
    }

    /// 200 OK dengan data list + pagination metadata
    pub fn ok_paginated(
        message: impl Into<String>,
        data: T,
        meta: PaginationMeta,
    ) -> (StatusCode, Json<Self>) {
        (
            StatusCode::OK,
            Json(Self {
                success: true,
                message: message.into(),
                data: Some(data),
                meta: Some(meta),
                code: None,
                details: None,
            }),
        )
    }

    /// 201 Created dengan data baru yang dibuat
    pub fn created(message: impl Into<String>, data: T) -> (StatusCode, Json<Self>) {
        (
            StatusCode::CREATED,
            Json(Self {
                success: true,
                message: message.into(),
                data: Some(data),
                meta: None,
                code: None,
                details: None,
            }),
        )
    }
}

impl ApiResponse<()> {
    /// 200 OK tanpa data balikan (misal: update status atau delete berhasil)
    pub fn ok_msg(message: impl Into<String>) -> (StatusCode, Json<Self>) {
        (
            StatusCode::OK,
            Json(Self {
                success: true,
                message: message.into(),
                data: None,
                meta: None,
                code: None,
                details: None,
            }),
        )
    }

    /// Membangun format response error standar
    pub fn error(
        status: StatusCode,
        message: impl Into<String>,
        code: impl Into<String>,
        details: Option<Vec<ValidationErrorDetail>>,
    ) -> (StatusCode, Json<Self>) {
        (
            status,
            Json(Self {
                success: false,
                message: message.into(),
                data: None,
                meta: None,
                code: Some(code.into()),
                details,
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_ok_single_data() {
        let (status, Json(res)) =
            ApiResponse::ok("Data retrieved successfully", json!({ "id": 1, "name": "Budi" }));
        assert_eq!(status, StatusCode::OK);
        let val = serde_json::to_value(&res).unwrap();
        assert_eq!(
            val,
            json!({
                "success": true,
                "message": "Data retrieved successfully",
                "data": { "id": 1, "name": "Budi" }
            })
        );
    }

    #[test]
    fn test_ok_paginated() {
        let meta = PaginationMeta {
            page: 1,
            limit: 10,
            total: 52,
        };
        let data = vec![
            json!({ "id": 1, "name": "Budi" }),
            json!({ "id": 2, "name": "Sari" }),
        ];
        let (status, Json(res)) = ApiResponse::ok_paginated("Data retrieved successfully", data, meta);
        assert_eq!(status, StatusCode::OK);
        let val = serde_json::to_value(&res).unwrap();
        assert_eq!(
            val,
            json!({
                "success": true,
                "message": "Data retrieved successfully",
                "data": [
                    { "id": 1, "name": "Budi" },
                    { "id": 2, "name": "Sari" }
                ],
                "meta": { "page": 1, "limit": 10, "total": 52 }
            })
        );
    }

    #[test]
    fn test_created() {
        let (status, Json(res)) =
            ApiResponse::created("Data created successfully", json!({ "id": 3, "name": "Andi" }));
        assert_eq!(status, StatusCode::CREATED);
        let val = serde_json::to_value(&res).unwrap();
        assert_eq!(
            val,
            json!({
                "success": true,
                "message": "Data created successfully",
                "data": { "id": 3, "name": "Andi" }
            })
        );
    }

    #[test]
    fn test_ok_msg_without_data() {
        let (status, Json(res)) = ApiResponse::ok_msg("Data deleted successfully");
        assert_eq!(status, StatusCode::OK);
        let val = serde_json::to_value(&res).unwrap();
        assert_eq!(
            val,
            json!({
                "success": true,
                "message": "Data deleted successfully"
            })
        );
    }

    #[test]
    fn test_error_bad_request() {
        let (status, Json(res)) = ApiResponse::error(
            StatusCode::BAD_REQUEST,
            "Invalid request",
            "BAD_REQUEST",
            None,
        );
        assert_eq!(status, StatusCode::BAD_REQUEST);
        let val = serde_json::to_value(&res).unwrap();
        assert_eq!(
            val,
            json!({
                "success": false,
                "message": "Invalid request",
                "code": "BAD_REQUEST"
            })
        );
    }

    #[test]
    fn test_error_validation() {
        let details = vec![
            ValidationErrorDetail {
                field: "email".to_string(),
                message: "Invalid email format".to_string(),
            },
            ValidationErrorDetail {
                field: "password".to_string(),
                message: "Minimum 8 characters".to_string(),
            },
        ];
        let (status, Json(res)) = ApiResponse::error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Validation failed",
            "VALIDATION_ERROR",
            Some(details),
        );
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        let val = serde_json::to_value(&res).unwrap();
        assert_eq!(
            val,
            json!({
                "success": false,
                "message": "Validation failed",
                "code": "VALIDATION_ERROR",
                "details": [
                    { "field": "email", "message": "Invalid email format" },
                    { "field": "password", "message": "Minimum 8 characters" }
                ]
            })
        );
    }
}
