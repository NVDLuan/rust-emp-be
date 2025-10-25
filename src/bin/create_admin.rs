use emp_be::infra::database::postgres::establish_connection;
use emp_be::modules::user::repository::repository::UserRepository;
use emp_be::modules::user::roles::UserRole;
use emp_be::modules::user::utils::hasher::hash_password;
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Admin User Creation Tool");
    println!("==========================");

    // Load environment variables
    dotenv::dotenv().ok();

    // Get database URL
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment variables");

    // Connect to database
    println!("🔗 Connecting to database...");
    let db = establish_connection(database_url).await?;
    println!("✅ Connected to database successfully");

    // Get admin details from user input
    let mut name = String::new();
    let mut email = String::new();
    let mut password = String::new();

    print!("Enter admin name: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut name)?;
    name = name.trim().to_string();

    print!("Enter admin email: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut email)?;
    email = email.trim().to_string();

    print!("Enter admin password: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut password)?;
    password = password.trim().to_string();

    // Validate inputs
    if name.is_empty() || email.is_empty() || password.is_empty() {
        eprintln!("❌ All fields are required!");
        return Ok(());
    }

    // Check if admin already exists
    match UserRepository::get_user_by_email(&db, &email).await {
        Ok(_) => {
            eprintln!("❌ Admin user with email {} already exists!", email);
            return Ok(());
        }
        Err(_) => {
            // User doesn't exist, proceed with creation
        }
    }

    // Hash password
    let hashed_password = hash_password(&password);

    // Create admin user
    println!("👤 Creating admin user...");
    match UserRepository::insert_user(
        &db,
        &name,
        &email,
        &hashed_password,
        UserRole::Admin.as_str(),
    ).await {
        Ok(user) => {
            println!("✅ Admin user created successfully!");
            println!("   ID: {}", user.id);
            println!("   Name: {}", user.name);
            println!("   Email: {}", user.email);
            println!("   Role: {}", user.role);
            println!("   Created: {}", user.created_at);
        }
        Err(e) => {
            eprintln!("❌ Failed to create admin user: {}", e);
        }
    }

    Ok(())
}
