pub struct App {
    config: crate::config::Config,
}

impl App {
    pub fn new(config: crate::config::Config) -> Self {
        Self { config }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        // Application logic here
        Ok(())
    }
}
