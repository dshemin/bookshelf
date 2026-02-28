mod config;
mod schema;
mod sqlite;
mod users;

use axum::{Router, routing::get};
use clap::{Args, Parser, Subcommand};
use log::info;
use tokio::signal;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Server,
    /// Manage users.
    Users(UsersArgs),
}

#[derive(Debug, Args)]
struct UsersArgs {
    #[command(subcommand)]
    command: UsersCommands,
}

#[derive(Debug, Subcommand)]
enum UsersCommands {
    /// Adds new user with provided login and password.
    Add {
        /// New user's login.
        login: String,
        /// New user's password.
        password: String,
    },
}

#[derive(Clone)]
struct Container {
    user: users::Service,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger();

    let cfg = config::load()?;
    let db_url: String = cfg.db.clone().into();

    let pool = sqlite::connect(&db_url).await?;
    let container = Container {
        user: users::Service::new(pool),
    };

    if !handle_cli(&container).await? {
        return Ok(());
    }

    let state = AppState {
        config: cfg.clone(),
        container,
    };

    let app = Router::new()
        .route("/healthz", get(|| async { "Ok" }))
        .route("/readiness", get(|| async { "Ok" }))
        .with_state(state);

    info!(config:? = &cfg; "start server");
    let listener = tokio::net::TcpListener::bind(cfg.address).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

fn setup_logger() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .parse_env("BOOKSHELF_LOG")
        .format_file(true)
        .format_line_number(true)
        .init();
}

async fn handle_cli(container: &Container) -> anyhow::Result<bool> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Server => {}
        Commands::Users(args) => match args.command {
            UsersCommands::Add { login, password } => {
                println!("Create new user");
                let user = users::User::new(login, password, String::new())?;
                container.user.create(user).await?;
                return Ok(false);
            }
        },
    };

    return Ok(true);
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("can't install CTRL-C signal");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => { info!("stop server") },
        _ = terminate => { info!("stop server") },
    }
}

#[derive(Clone)]
struct AppState {
    config: config::Config,
    container: Container,
}
