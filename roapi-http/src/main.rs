#![deny(warnings)]

use roapi_http::config::get_configuration;
use roapi_http::startup::Application;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let config = get_configuration()?;

    let application = Application::build(config).await?;
    application.run_until_stopped().await?;
    Ok(())
}
