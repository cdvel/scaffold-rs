#[cfg(feature = "async")]
use tokio;

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run_app()?;

    // Demo async and time features
    tracing::info!("Running async/time utilities demo");

    let data = {{ project-name | snake_case }}::async_utils::fetch_data("demo-resource").await?;
    let now = {{ project-name | snake_case }}::time_utils::current_timestamp();

    println!("Demo results at {}: {}", now, data);

    {{ project-name | snake_case}}::time_utils::parse_timestamp(&now)
        .map(|date| {{ project-name | snake_case }}::time_utils::add_days(date, 30))
        .map(|future| println!("Next check scheduled for: {}", future))
        .ok();

    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {
    if let Err(e) = run_app() {
        eprintln!("Application error: {}", e);
    }

    println!("Note: Running without with --features async for async examples");
}

fn run_app() -> anyhow::Result<()> {
    {{ project-name | snake_case }}::logging::setup(true);

    let config = {{ project-name | snake_case }}::config::Config::load()?;
    tracing::info!("Application started with config: {:?}", config);

    let app = {{ project-name | snake_case }}::app::App::new(config);
    app.run()?;

    tracing::info!("Application completed successfully");
    Ok(())
}
