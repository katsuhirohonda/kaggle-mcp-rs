//! Example demonstrating how to use the competitions_list functionality
//! 
//! This example shows how to authenticate with Kaggle and list competitions
//! with various filtering options.

use kaggle_mcp_rs::client::KaggleClient;
use kaggle_mcp_rs::models::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize tracing for better error messages
    tracing_subscriber::fmt::init();

    // Create a new Kaggle client
    let client = KaggleClient::new();

    // Load credentials from environment variables or ~/.kaggle/kaggle.json
    client.load_credentials().await?;

    println!("🔐 Authenticated with Kaggle API");
    println!();

    // Example 1: List all competitions (default parameters)
    println!("📋 All Competitions (latest deadline):");
    println!("{}", "=".repeat(50));
    
    let competitions = client.list_competitions(
        "".to_string(),               // search (empty for all)
        "all".to_string(),            // category
        "general".to_string(),        // group
        "latestDeadline".to_string(), // sort_by
        1,                            // page
    ).await?;

    for comp in competitions.iter().take(5) {
        println!("🏆 {}", comp.title);
        println!("   ID: {}", comp.ref_);
        if let Some(deadline) = &comp.deadline {
            println!("   Deadline: {}", deadline);
        }
        println!("   Teams: {}", comp.team_count);
        if let Some(reward) = &comp.reward {
            println!("   Prize: {}", reward);
        }
        println!();
    }

    // Example 2: Search for specific competitions
    println!("\n🔍 Search for 'machine learning' competitions:");
    println!("{}", "=".repeat(50));
    
    let ml_competitions = client.list_competitions(
        "machine learning".to_string(),
        "all".to_string(),
        "general".to_string(),
        "latestDeadline".to_string(),
        1,
    ).await?;

    for comp in ml_competitions.iter().take(3) {
        println!("🏆 {}", comp.title);
        if let Some(desc) = &comp.description {
            let truncated = desc.chars().take(100).collect::<String>();
            println!("   {}...", truncated);
        }
        println!();
    }

    // Example 3: List featured competitions
    println!("\n⭐ Featured Competitions:");
    println!("{}", "=".repeat(50));
    
    let featured = client.list_competitions(
        "".to_string(),
        "featured".to_string(),
        "general".to_string(),
        "latestDeadline".to_string(),
        1,
    ).await?;

    for comp in featured.iter().take(3) {
        println!("🏆 {}", comp.title);
        println!("   Category: {}", comp.category);
        println!("   URL: {}", comp.url);
        println!();
    }

    // Example 4: Sort by prize money
    println!("\n💰 Competitions sorted by prize:");
    println!("{}", "=".repeat(50));
    
    let by_prize = client.list_competitions(
        "".to_string(),
        "all".to_string(),
        "general".to_string(),
        "prize".to_string(),
        1,
    ).await?;

    for comp in by_prize.iter().take(3) {
        print!("🏆 {}", comp.title);
        if let Some(reward) = &comp.reward {
            println!(" - {}", reward);
        } else {
            println!(" - No prize");
        }
        println!();
    }

    // Example 5: Using environment variables for credentials
    println!("\n💡 Tip: You can set credentials via environment variables:");
    println!("   export KAGGLE_USERNAME=your_username");
    println!("   export KAGGLE_KEY=your_api_key");
    println!("\n   Or create ~/.kaggle/kaggle.json:");
    println!("   {{\"username\":\"your_username\",\"key\":\"your_api_key\"}}");

    Ok(())
}