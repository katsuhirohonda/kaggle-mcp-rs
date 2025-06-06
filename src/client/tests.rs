#[cfg(test)]
mod tests {
    use super::super::*;
    use mockito::{Server, ServerGuard};
    use tempfile::TempDir;
    use serial_test::serial;

    async fn create_test_client() -> (KaggleClient, ServerGuard) {
        let server = Server::new_async().await;
        let client = KaggleClient::new()
            .with_api_base(server.url())
            .skip_save_credentials();
        (client, server)
    }

    #[tokio::test]
    async fn test_new_client() {
        let (client, _server) = create_test_client().await;
        assert!(!client.is_authenticated().await);
    }

    #[tokio::test]
    async fn test_authenticate_success() {
        let (client, mut server) = create_test_client().await;
        
        let _m = server.mock("GET", "/api/v1/competitions/list")
            .with_status(200)
            .with_body(r#"[]"#)
            .create_async()
            .await;

        let result = client
            .authenticate(
                "test_user".to_string(),
                "test_key".to_string()
            )
            .await;

        // The test should succeed if we can call the API
        assert!(result.is_ok());
        assert!(client.is_authenticated().await);
    }

    #[tokio::test]
    async fn test_authenticate_failure() {
        let (client, mut server) = create_test_client().await;
        let _m = server.mock("GET", "/api/v1/competitions/list")
            .with_status(401)
            .with_body(r#"{"message": "Unauthorized"}"#)
            .create_async()
            .await;

        let result = client
            .authenticate(
                "invalid_user".to_string(),
                "invalid_key".to_string()
            )
            .await;

        assert!(result.is_err());
        match result {
            Err(Error::AuthenticationError(msg)) => {
                assert!(msg.contains("401"));
            }
            _ => panic!("Expected AuthenticationError"),
        }
        assert!(!client.is_authenticated().await);
    }

    #[tokio::test]
    #[serial]
    async fn test_load_credentials_from_env() {
        // Save current env state
        let orig_username = std::env::var("KAGGLE_USERNAME").ok();
        let orig_key = std::env::var("KAGGLE_KEY").ok();
        
        // Set test env vars
        std::env::set_var("KAGGLE_USERNAME", "env_user");
        std::env::set_var("KAGGLE_KEY", "env_key");

        let client = KaggleClient::new();
        let result = client.load_credentials().await;
        
        // Should succeed with env vars
        assert!(result.is_ok());
        assert!(client.is_authenticated().await);

        // Restore original state
        match orig_username {
            Some(val) => std::env::set_var("KAGGLE_USERNAME", val),
            None => std::env::remove_var("KAGGLE_USERNAME"),
        }
        match orig_key {
            Some(val) => std::env::set_var("KAGGLE_KEY", val), 
            None => std::env::remove_var("KAGGLE_KEY"),
        }
    }

    #[tokio::test]
    async fn test_load_credentials_not_found() {
        // Create a temp directory and point HOME to it
        let temp_dir = TempDir::new().unwrap();
        let original_home = std::env::var("HOME").ok();
        std::env::set_var("HOME", temp_dir.path());
        
        // Ensure no env vars are set
        std::env::remove_var("KAGGLE_USERNAME");
        std::env::remove_var("KAGGLE_KEY");

        let client = KaggleClient::new();
        let result = client.load_credentials().await;
        
        // Should fail because no credentials exist
        assert!(result.is_err());
        match result {
            Err(Error::NotAuthenticated) => {},
            _ => panic!("Expected NotAuthenticated error"),
        }

        // Restore original HOME
        if let Some(home) = original_home {
            std::env::set_var("HOME", home);
        } else {
            std::env::remove_var("HOME");
        }
    }

    #[tokio::test]
    async fn test_request_without_auth() {
        let (client, _server) = create_test_client().await;
        let builder = client.http_client().get("https://example.com");
        
        let result = client.request(builder).await;
        assert!(result.is_err());
        match result {
            Err(Error::NotAuthenticated) => {},
            _ => panic!("Expected NotAuthenticated error"),
        }
    }

    #[tokio::test]
    async fn test_client_stores_credentials() {
        let client = KaggleClient::new();
        
        // Initially not authenticated
        assert!(!client.is_authenticated().await);
        
        // Manually set credentials
        {
            let mut creds = client.credentials.write().await;
            *creds = Some(KaggleCredentials {
                username: "test".to_string(),
                key: "test_key".to_string(),
            });
        }
        
        // Now should be authenticated
        assert!(client.is_authenticated().await);
    }
}