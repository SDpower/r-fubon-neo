use clap::Parser;
use r_fubon_neo::{FubonSDK, CoreSDK, Mode};
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// API Key for authentication
    #[arg(long)]
    api_key: Option<String>,
    
    /// Secret Key for authentication  
    #[arg(long)]
    secret_key: Option<String>,
    
    /// Command to execute
    #[arg(value_enum)]
    command: Command,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Command {
    /// Get library version
    Version,
    /// Test SDK connection
    Test,
    /// Initialize market data
    MarketData,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if it exists
    let _ = dotenvy::dotenv();
    
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Command::Version => {
            println!("r-fubon-neo version: {}", r_fubon_neo::VERSION);
        }
        
        Command::Test => {
            // Get credentials from CLI args or environment variables
            let api_key = cli.api_key
                .or_else(|| env::var("FUBON_API_KEY").ok())
                .or_else(|| env::var("API_KEY").ok());
            let secret_key = cli.secret_key
                .or_else(|| env::var("FUBON_SECRET_KEY").ok())
                .or_else(|| env::var("SECRET_KEY").ok());
                
            if let (Some(api_key), Some(secret_key)) = (api_key, secret_key) {
                let sdk = FubonSDK::new().with_credentials(api_key, secret_key);
                
                // Test basic SDK functionality
                match sdk.get_account_balance() {
                    Ok(balance) => println!("Account balance: ${:.2}", balance),
                    Err(e) => eprintln!("Error getting balance: {}", e),
                }
                
                match sdk.get_positions() {
                    Ok(positions) => {
                        println!("Positions: {:?}", positions);
                    }
                    Err(e) => eprintln!("Error getting positions: {}", e),
                }
            } else {
                eprintln!("API key and secret key are required for testing");
                eprintln!("Provide them via:");
                eprintln!("  CLI args: --api-key YOUR_KEY --secret-key YOUR_SECRET");
                eprintln!("  Environment variables: FUBON_API_KEY and FUBON_SECRET_KEY");
                eprintln!("  Or create a .env file with:");
                eprintln!("    FUBON_API_KEY=your_api_key");
                eprintln!("    FUBON_SECRET_KEY=your_secret_key");
                std::process::exit(1);
            }
        }
        
        Command::MarketData => {
            // Get credentials from CLI args or environment variables
            let api_key = cli.api_key
                .or_else(|| env::var("FUBON_API_KEY").ok())
                .or_else(|| env::var("API_KEY").ok());
            let secret_key = cli.secret_key
                .or_else(|| env::var("FUBON_SECRET_KEY").ok())
                .or_else(|| env::var("SECRET_KEY").ok());
                
            if let (Some(api_key), Some(secret_key)) = (api_key, secret_key) {
                let mut sdk = FubonSDK::new().with_credentials(api_key, secret_key);
                
                // Initialize market data
                match sdk.init_realtime(Mode::Speed) {
                    Ok(_) => {
                        println!("Market data initialized successfully");
                        
                        if let Some(market_data) = sdk.market_data() {
                            println!("WebSocket client ready");
                            println!("REST client ready");
                        }
                    }
                    Err(e) => eprintln!("Error initializing market data: {}", e),
                }
            } else {
                eprintln!("API key and secret key are required for market data");
                eprintln!("Provide them via:");
                eprintln!("  CLI args: --api-key YOUR_KEY --secret-key YOUR_SECRET");
                eprintln!("  Environment variables: FUBON_API_KEY and FUBON_SECRET_KEY");
                eprintln!("  Or create a .env file with:");
                eprintln!("    FUBON_API_KEY=your_api_key");
                eprintln!("    FUBON_SECRET_KEY=your_secret_key");
                std::process::exit(1);
            }
        }
    }
    
    Ok(())
}

