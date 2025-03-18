//! Application coordination logic
//!
//! This module contains the `App` struct, which serves as the primary
//! entry point for application functionality.

use crate::config::Config;
use crate::error::Result;
use tracing::{debug, info};

/// The main application struct that coordinates operations.
///
/// `App` is responsible for initializing and running the application's
/// primary functionality based on the provided configuration.
///
/// # Example
///
/// ```
/// use {{project-name}}::app::App;
/// use {{project-name}}::config::Config;
///
/// # fn main() -> anyhow::Result<()> {
/// let config = Config::load()?;
/// let app = App::new(config);
/// app.run()?;
/// # Ok(())
/// # }
/// ```
pub struct App {
    config: Config,
}

impl App {
    /// Creates a new App instance with the provided configuration.
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Runs the application logic.
    ///
    /// This is the main entry point for executing the application's
    /// functionality after initialization.
    pub fn run(&self) -> Result<()> {
        info!("Application starting");
        debug!("Using configuration: {:?}", self.config);

        // Application logic goes here

        info!("Application completed successfully");
        Ok(())
    }

    /// Returns a reference to the application's configuration.
    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    // This test requires Config to have a method like `default()` or a test constructor
    #[test]
    fn test_app_initialization() {
        let config = Config {
            database_url: "test:///db".to_string(),
            api_key: Some("test-key".to_string()),
            timeout_seconds: 30,
        };

        let app = App::new(config);

        assert_eq!(app.config().database_url, "test:///db");
        assert_eq!(app.config().timeout_seconds, 30);
    }
}
