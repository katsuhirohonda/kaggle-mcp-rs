#[cfg(test)]
mod tests {
    use super::super::*;

    fn create_test_server() -> KaggleMcpServer {
        KaggleMcpServer::new()
    }

    #[tokio::test]
    async fn test_server_info() {
        let server = create_test_server();
        let info = server.get_info();
        
        assert_eq!(info.server_info.name, "kaggle-mcp-rs");
        assert_eq!(info.server_info.version, env!("CARGO_PKG_VERSION"));
        assert!(info.capabilities.tools.is_some());
        assert!(info.instructions.is_some());
    }

    #[tokio::test]
    async fn test_authenticate_params_schema() {
        // Test that AuthenticateParams can be serialized/deserialized correctly
        let params = AuthenticateParams {
            kaggle_username: "test_user".to_string(),
            kaggle_key: "test_key".to_string(),
        };

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["kaggle_username"], "test_user");
        assert_eq!(json["kaggle_key"], "test_key");

        let deserialized: AuthenticateParams = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.kaggle_username, params.kaggle_username);
        assert_eq!(deserialized.kaggle_key, params.kaggle_key);
    }

    #[tokio::test]
    async fn test_authenticate_response_format() {
        let response = AuthenticationResponse {
            success: true,
            message: "Successfully authenticated".to_string(),
            username: Some("test_user".to_string()),
        };

        let json = serde_json::to_string_pretty(&response).unwrap();
        assert!(json.contains("success"));
        assert!(json.contains("message"));
        assert!(json.contains("username"));
    }

    // Integration tests would be better done with a full server setup
    // For now, we focus on unit testing the components
}