use clap::Parser;
use r_fubon_neo::{FubonSDK, CoreSDK, Mode, LoginCredentials};
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Personal ID for login
    #[arg(long)]
    personal_id: Option<String>,
    
    /// Password for authentication
    #[arg(long)]
    password: Option<String>,
    
    /// Certificate file path
    #[arg(long)]
    cert_path: Option<String>,
    
    /// Certificate password (optional)
    #[arg(long)]
    cert_pass: Option<String>,
    
    /// Command to execute
    #[arg(value_enum)]
    command: Command,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Command {
    /// Get library version
    Version,
    /// Login and list accounts
    Login,
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
        
        Command::Login => {
            // Get credentials from CLI args or environment variables
            let personal_id = cli.personal_id
                .or_else(|| env::var("FUBON_PERSONAL_ID").ok())
                .or_else(|| env::var("PERSONAL_ID").ok());
            let password = cli.password
                .or_else(|| env::var("FUBON_PASSWORD").ok())
                .or_else(|| env::var("PASSWORD").ok());
            let cert_path = cli.cert_path
                .or_else(|| env::var("FUBON_CERT_PATH").ok())
                .or_else(|| env::var("CERT_PATH").ok());
            let cert_pass = cli.cert_pass
                .or_else(|| env::var("FUBON_CERT_PASS").ok())
                .or_else(|| env::var("CERT_PASS").ok());
                
            if let (Some(personal_id), Some(password), Some(cert_path)) = (personal_id, password, cert_path) {
                let credentials = LoginCredentials {
                    personal_id,
                    password,
                    cert_path,
                    cert_pass,
                };
                
                let mut sdk = FubonSDK::new();
                
                // Login and get accounts
                match sdk.login(credentials) {
                    Ok(accounts) => {
                        println!("Login successful! Available accounts:");
                        for account in accounts {
                            println!("  - ID: {}, Name: {}, Type: {}, Status: {}", 
                                account.account_id, 
                                account.account_name, 
                                account.account_type,
                                account.status
                            );
                            if let (Some(available), Some(total)) = (account.available_balance, account.total_balance) {
                                println!("    Balance: Available: {:.2} {}, Total: {:.2} {}", 
                                    available, account.currency, total, account.currency);
                            }
                        }
                    }
                    Err(e) => eprintln!("Login failed: {}", e),
                }
            } else {
                eprintln!("Personal ID, password, and certificate path are required for login");
                eprintln!("Provide them via:");
                eprintln!("  CLI args: --personal-id YOUR_ID --password YOUR_PASSWORD --cert-path /path/to/cert");
                eprintln!("  Environment variables: FUBON_PERSONAL_ID, FUBON_PASSWORD, FUBON_CERT_PATH");
                eprintln!("  Or create a .env file with:");
                eprintln!("    FUBON_PERSONAL_ID=your_personal_id");
                eprintln!("    FUBON_PASSWORD=your_password");
                eprintln!("    FUBON_CERT_PATH=/path/to/your/certificate.p12");
                eprintln!("    FUBON_CERT_PASS=cert_password_if_needed");
                std::process::exit(1);
            }
        }
        
        Command::Test => {
            // Get credentials from CLI args or environment variables
            let personal_id = cli.personal_id
                .or_else(|| env::var("FUBON_PERSONAL_ID").ok())
                .or_else(|| env::var("PERSONAL_ID").ok());
            let password = cli.password
                .or_else(|| env::var("FUBON_PASSWORD").ok())
                .or_else(|| env::var("PASSWORD").ok());
            let cert_path = cli.cert_path
                .or_else(|| env::var("FUBON_CERT_PATH").ok())
                .or_else(|| env::var("CERT_PATH").ok());
            let cert_pass = cli.cert_pass
                .or_else(|| env::var("FUBON_CERT_PASS").ok())
                .or_else(|| env::var("CERT_PASS").ok());
                
            if let (Some(personal_id), Some(password), Some(cert_path)) = (personal_id, password, cert_path) {
                let credentials = LoginCredentials {
                    personal_id,
                    password,
                    cert_path,
                    cert_pass,
                };
                
                let mut sdk = FubonSDK::new();
                
                // Login first
                match sdk.login(credentials) {
                    Ok(accounts) => {
                        println!("Login successful! Found {} account(s)", accounts.len());
                        
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
                    }
                    Err(e) => eprintln!("Login failed: {}", e),
                }
            } else {
                eprintln!("Personal ID, password, and certificate path are required for testing");
                eprintln!("Provide them via:");
                eprintln!("  CLI args: --personal-id YOUR_ID --password YOUR_PASSWORD --cert-path /path/to/cert");
                eprintln!("  Environment variables: FUBON_PERSONAL_ID, FUBON_PASSWORD, FUBON_CERT_PATH");
                eprintln!("  Or create a .env file with:");
                eprintln!("    FUBON_PERSONAL_ID=your_personal_id");
                eprintln!("    FUBON_PASSWORD=your_password");
                eprintln!("    FUBON_CERT_PATH=/path/to/your/certificate.p12");
                eprintln!("    FUBON_CERT_PASS=cert_password_if_needed");
                std::process::exit(1);
            }
        }
        
        Command::MarketData => {
            // Get credentials from CLI args or environment variables
            let personal_id = cli.personal_id
                .or_else(|| env::var("FUBON_PERSONAL_ID").ok())
                .or_else(|| env::var("PERSONAL_ID").ok());
            let password = cli.password
                .or_else(|| env::var("FUBON_PASSWORD").ok())
                .or_else(|| env::var("PASSWORD").ok());
            let cert_path = cli.cert_path
                .or_else(|| env::var("FUBON_CERT_PATH").ok())
                .or_else(|| env::var("CERT_PATH").ok());
            let cert_pass = cli.cert_pass
                .or_else(|| env::var("FUBON_CERT_PASS").ok())
                .or_else(|| env::var("CERT_PASS").ok());
                
            if let (Some(personal_id), Some(password), Some(cert_path)) = (personal_id, password, cert_path) {
                let credentials = LoginCredentials {
                    personal_id,
                    password,
                    cert_path,
                    cert_pass,
                };
                
                let mut sdk = FubonSDK::new();
                
                // Login first
                match sdk.login(credentials) {
                    Ok(accounts) => {
                        println!("Login successful! Found {} account(s)", accounts.len());
                        
                        // Initialize market data
                        match sdk.init_realtime(Mode::Speed) {
                            Ok(_) => {
                                println!("Market data initialized successfully");
                                
                                if let Some(_market_data) = sdk.market_data() {
                                    println!("WebSocket client ready");
                                    println!("REST client ready");
                                }
                            }
                            Err(e) => eprintln!("Error initializing market data: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Login failed: {}", e),
                }
            } else {
                eprintln!("Personal ID, password, and certificate path are required for market data");
                eprintln!("Provide them via:");
                eprintln!("  CLI args: --personal-id YOUR_ID --password YOUR_PASSWORD --cert-path /path/to/cert");
                eprintln!("  Environment variables: FUBON_PERSONAL_ID, FUBON_PASSWORD, FUBON_CERT_PATH");
                eprintln!("  Or create a .env file with:");
                eprintln!("    FUBON_PERSONAL_ID=your_personal_id");
                eprintln!("    FUBON_PASSWORD=your_password");
                eprintln!("    FUBON_CERT_PATH=/path/to/your/certificate.p12");
                eprintln!("    FUBON_CERT_PASS=cert_password_if_needed");
                std::process::exit(1);
            }
        }
    }
    
    Ok(())
}

