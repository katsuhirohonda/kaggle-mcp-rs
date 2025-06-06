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

    #[tokio::test]
    async fn test_competitions_list_params_defaults() {
        // Test that CompetitionsListParams has correct defaults
        let params_json = serde_json::json!({});
        let params: CompetitionsListParams = serde_json::from_value(params_json).unwrap();
        
        assert_eq!(params.search, "");
        assert_eq!(params.category, "all");
        assert_eq!(params.group, "general");
        assert_eq!(params.sort_by, "latestDeadline");
        assert_eq!(params.page, 1);
    }

    #[tokio::test]
    async fn test_competitions_list_params_custom() {
        // Test that CompetitionsListParams can be customized
        let params = CompetitionsListParams {
            search: "titanic".to_string(),
            category: "featured".to_string(),
            group: "entered".to_string(),
            sort_by: "prize".to_string(),
            page: 2,
        };

        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["search"], "titanic");
        assert_eq!(json["category"], "featured");
        assert_eq!(json["group"], "entered");
        assert_eq!(json["sort_by"], "prize");
        assert_eq!(json["page"], 2);
    }

    #[tokio::test]
    async fn test_competitions_list_not_authenticated() {
        let server = create_test_server();
        let params = CompetitionsListParams {
            search: String::new(),
            category: "all".to_string(),
            group: "general".to_string(),
            sort_by: "latestDeadline".to_string(),
            page: 1,
        };

        // Since the server isn't authenticated, this should fail
        let result = server.competitions_list(params).await;
        assert!(result.is_err());
        
        if let Err(e) = result {
            let error_msg = e.to_string();
            assert!(error_msg.contains("Not authenticated"));
        }
    }

    // Integration tests would be better done with a full server setup
    // For now, we focus on unit testing the components
}