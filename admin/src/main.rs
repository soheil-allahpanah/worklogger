use std::process::ExitCode;
use std::sync::Arc;

use clap::{Parser, Subcommand};
use domain::bootstrap::legacy_user_id;
use domain::value_objects::{TokenId, UserId};
use infrastructure::postgres::{
    connect, PostgresTokenRepository, PostgresUserRepository,
};
use use_cases::{
    CreateTokenCommand, CreateTokenUseCase, CreateUserCommand, CreateUserUseCase,
    DeleteUserCommand, DeleteUserUseCase, DisableUserCommand, DisableUserUseCase,
    EnableUserCommand, EnableUserUseCase, RevokeTokenCommand, RevokeTokenUseCase,
    SetPasswordCommand, SetPasswordUseCase,
};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "worklogger-admin")]
#[command(about = "Admin CLI for user and device-token management (requires DATABASE_URL)")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new user
    CreateUser {
        #[arg(long)]
        name: String,
        #[arg(long)]
        email: Option<String>,
        #[arg(long)]
        password: Option<String>,
    },
    /// Mint a device token for a user (token printed once)
    CreateToken {
        #[arg(long)]
        user: Uuid,
        #[arg(long)]
        label: String,
    },
    /// Mint a device token for the bootstrap legacy user (token printed once)
    CreateLegacyToken {
        #[arg(long, default_value = "legacy-default")]
        label: String,
    },
    /// Revoke a device token by id
    RevokeToken {
        #[arg(long = "token-id")]
        token_id: Uuid,
    },
    /// Disable a user account
    DisableUser {
        #[arg(long)]
        user: Uuid,
    },
    /// Re-enable a disabled user account
    EnableUser {
        #[arg(long)]
        user: Uuid,
    },
    /// Soft-delete a user account
    DeleteUser {
        #[arg(long)]
        user: Uuid,
    },
    /// Set or reset a user's password
    SetPassword {
        #[arg(long)]
        user: Uuid,
        #[arg(long)]
        password: String,
    },
}

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(message) = run().await {
        eprintln!("error: {message}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

async fn run() -> Result<(), String> {
    let cli = Cli::parse();
    ensure_admin_token()?;

    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL must be set".to_string())?;

    let pool = connect(&database_url)
        .await
        .map_err(|e| format!("database connection failed: {e}"))?;

    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
    let token_repo = Arc::new(PostgresTokenRepository::new(pool));

    match cli.command {
        Command::CreateUser { name, email, password } => {
            let use_case = CreateUserUseCase::new(Arc::clone(&user_repo));
            let response = use_case
                .execute(CreateUserCommand {
                    name,
                    email,
                    password,
                })
                .await
                .map_err(|e| e.to_string())?;
            println!("user_id={}", response.id());
            println!("name={}", response.name());
        }
        Command::CreateToken { user, label } => {
            create_token(
                Arc::clone(&token_repo),
                Arc::clone(&user_repo),
                UserId::from_uuid(user),
                label,
            )
            .await?;
        }
        Command::CreateLegacyToken { label } => {
            create_token(
                Arc::clone(&token_repo),
                Arc::clone(&user_repo),
                legacy_user_id(),
                label,
            )
            .await?;
        }
        Command::RevokeToken { token_id } => {
            let use_case = RevokeTokenUseCase::new(Arc::clone(&token_repo));
            use_case
                .execute(RevokeTokenCommand {
                    token_id: TokenId::from_uuid(token_id),
                })
                .await
                .map_err(|e| e.to_string())?;
            println!("token revoked");
        }
        Command::DisableUser { user } => {
            let use_case = DisableUserUseCase::new(Arc::clone(&user_repo));
            use_case
                .execute(DisableUserCommand {
                    user_id: UserId::from_uuid(user),
                })
                .await
                .map_err(|e| e.to_string())?;
            println!("user disabled");
        }
        Command::EnableUser { user } => {
            let use_case = EnableUserUseCase::new(Arc::clone(&user_repo));
            use_case
                .execute(EnableUserCommand {
                    user_id: UserId::from_uuid(user),
                })
                .await
                .map_err(|e| e.to_string())?;
            println!("user enabled");
        }
        Command::DeleteUser { user } => {
            let use_case = DeleteUserUseCase::new(Arc::clone(&user_repo));
            use_case
                .execute(DeleteUserCommand {
                    user_id: UserId::from_uuid(user),
                })
                .await
                .map_err(|e| e.to_string())?;
            println!("user soft-deleted");
        }
        Command::SetPassword { user, password } => {
            let use_case = SetPasswordUseCase::new(Arc::clone(&user_repo));
            use_case
                .execute(SetPasswordCommand {
                    user_id: UserId::from_uuid(user),
                    password,
                })
                .await
                .map_err(|e| e.to_string())?;
            println!("password updated");
        }
    }

    Ok(())
}

async fn create_token(
    token_repo: Arc<PostgresTokenRepository>,
    user_repo: Arc<PostgresUserRepository>,
    user_id: UserId,
    label: String,
) -> Result<(), String> {
    let use_case = CreateTokenUseCase::new(token_repo, user_repo);
    let response = use_case
        .execute(CreateTokenCommand { user_id, label })
        .await
        .map_err(|e| e.to_string())?;
    println!("user_id={user_id}");
    println!("token_id={}", response.id());
    println!("token={}", response.token());
    eprintln!("Save this token now — it will not be shown again.");
    Ok(())
}

fn ensure_admin_token() -> Result<(), String> {
    match std::env::var("WORKLOGGER_ADMIN_TOKEN") {
        Ok(value) if !value.trim().is_empty() => Ok(()),
        _ => Err("WORKLOGGER_ADMIN_TOKEN must be set".to_string()),
    }
}
