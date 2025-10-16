use std::sync::OnceLock;

use anyhow::Context;

use crate::outputln;

#[derive(Debug)]
pub struct Object {
    pub log_days_retention: usize,
    // server's fqdn
    pub server_name: String,
    // this is the root path forward
    pub fallback_url: String,
    pub pg_fqdn: String,
    pub pg_username: String,
    pub pg_password: String,
    pub pg_dbname: String,
    pub pg_max_conns: u32,
    pub cache_fqdn: String,
    pub analytic_queue_fqdn: String,
    pub analytic_queue_pass: String,
}

static OBJECT: OnceLock<Object> = OnceLock::new();

pub fn object<'a>() -> &'a Object {
    OBJECT.get().expect("config was not initialized")
}

pub fn load() -> anyhow::Result<()> {
    let object: Object = Object {
        log_days_retention: std::env::var("SHORT_SERVICE_LOG_RETENTION")
            .context("missing 'SHORT_SERVICE_LOG_RETENTION'")?
            .parse()?,
        server_name: std::env::var("SHORT_SERVICE_SERVER_NAME")
            .context("missing 'SHORT_SERVICE_SERVER_NAME'")?,
        fallback_url: std::env::var("SHORT_SERVICE_FALLBACK_URL")
            .context("missing 'SHORT_SERVICE_FALLBACK_URL'")?,
        pg_fqdn: std::env::var("SHORT_SERVICE_DB_FQDN")
            .context("missing 'SHORT_SERVICE_DB_FQDN'")?,
        pg_username: std::env::var("SHORT_SERVICE_DB_USER")
            .context("missing 'SHORT_SERVICE_DB_USER'")?,
        pg_password: std::env::var("SHORT_SERVICE_DB_PASSWORD")
            .context("missing 'SHORT_SERVICE_DB_PASSWORD'")?,
        pg_dbname: std::env::var("SHORT_SERVICE_DB_NAME")
            .context("missing 'SHORT_SERVICE_DB_NAME'")?,
        pg_max_conns: std::env::var("SHORT_SERVICE_DB_MAX_CONNS")
            .context("missing 'SHORT_SERVICE_DB_MAX_CONNS'")?
            .parse()?,
        cache_fqdn: std::env::var("SHORT_SERVICE_CACHE_FQDN")
            .context("missing 'SHORT_SERVICE_CACHE_FQDN'")?
            .parse()?,
        analytic_queue_fqdn: std::env::var("SHORT_SERVICE_ANALYTIC_QUEUE_FQDN")
            .context("missing 'SHORT_SERVICE_ANALYTIC_QUEUE_FQDN'")?
            .parse()?,
        analytic_queue_pass: std::env::var("SHORT_SERVICE_ANALYTIC_QUEUE_PASS")
            .context("missing 'SHORT_SERVICE_ANALYTIC_QUEUE_PASS'")?
            .parse()?,
    };

    OBJECT.set(object).expect("config was already initialized");

    outputln!("configuration was loaded");

    Ok(())
}
