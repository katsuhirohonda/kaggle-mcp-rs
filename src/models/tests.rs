#[cfg(test)]
mod tests {
    use crate::models::*;

    #[test]
    fn test_kaggle_credentials_serialization() {
        let creds = KaggleCredentials {
            username: "test_user".to_string(),
            key: "test_key".to_string(),
        };

        let json = serde_json::to_value(&creds).unwrap();
        assert_eq!(json["username"], "test_user");
        assert_eq!(json["key"], "test_key");

        let deserialized: KaggleCredentials = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.username, creds.username);
        assert_eq!(deserialized.key, creds.key);
    }

    #[test]
    fn test_authentication_response() {
        let response = AuthenticationResponse {
            success: true,
            message: "Success".to_string(),
            username: Some("test_user".to_string()),
        };

        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(json["success"], true);
        assert_eq!(json["message"], "Success");
        assert_eq!(json["username"], "test_user");
    }

    #[test]
    fn test_kaggle_config_default() {
        let config = KaggleConfig::default();
        assert!(config.competition.is_none());
        assert!(config.path.is_none());
        assert!(config.proxy.is_none());
    }

    #[test]
    fn test_error_display() {
        let error = Error::AuthenticationError("Invalid credentials".to_string());
        assert_eq!(error.to_string(), "Authentication failed: Invalid credentials");

        let error = Error::NotAuthenticated;
        assert_eq!(error.to_string(), "Not authenticated");

        let kaggle_error = KaggleError {
            code: "404".to_string(),
            message: "Not found".to_string(),
        };
        assert_eq!(kaggle_error.to_string(), "404: Not found");
    }

    #[test]
    fn test_competition_model() {
        let competition = Competition {
            ref_: "titanic".to_string(),
            title: "Titanic - Machine Learning from Disaster".to_string(),
            url: "https://www.kaggle.com/c/titanic".to_string(),
            category: "Getting Started".to_string(),
            deadline: None,
            reward: Some("$0".to_string()),
            team_count: 1000,
            user_has_entered: false,
        };

        let json = serde_json::to_value(&competition).unwrap();
        assert_eq!(json["ref_"], "titanic");
        assert_eq!(json["title"], "Titanic - Machine Learning from Disaster");
    }

    #[test]
    fn test_dataset_model() {
        let dataset = Dataset {
            id: "dataset-123".to_string(),
            title: "Test Dataset".to_string(),
            subtitle: Some("A test dataset".to_string()),
            creator_name: "test_user".to_string(),
            total_bytes: 1024 * 1024,
            url: "https://www.kaggle.com/datasets/test/dataset".to_string(),
        };

        let json = serde_json::to_value(&dataset).unwrap();
        assert_eq!(json["id"], "dataset-123");
        assert_eq!(json["total_bytes"], 1024 * 1024);
    }

    #[test]
    fn test_error_from_reqwest() {
        // This tests the From<reqwest::Error> implementation
        // In a real scenario, we'd need to create a mock reqwest error
    }

    #[test]
    fn test_error_from_io() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let error = Error::from(io_error);
        match error {
            Error::IoError(_) => {},
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_error_from_json() {
        let json_str = "{invalid json}";
        let result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(json_str);
        let error = Error::from(result.unwrap_err());
        match error {
            Error::JsonError(_) => {},
            _ => panic!("Expected JsonError"),
        }
    }
}