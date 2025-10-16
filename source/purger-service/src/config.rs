use std::sync::OnceLock;

use anyhow::Context;

use crate::outputln;

#[derive(Debug)]
pub struct Object {
    pub log_days_retention: usize,
    pub interval: u64,
    pub pg_fqdn: String,
    pub pg_username: String,
    pub pg_password: String,
    pub pg_dbname: String,
    pub pg_max_conns: u32,
}

static OBJECT: OnceLock<Object> = OnceLock::new();

pub fn object<'a>() -> &'a Object {
    OBJECT.get().expect("config was not initialized")
}

pub fn load() -> anyhow::Result<()> {
    let object: Object = Object {
        log_days_retention: std::env::var("PURGER_SERVICE_LOG_RETENTION")
            .context("missing 'PURGER_SERVICE_LOG_RETENTION'")?
            .parse()?,
        interval: std::env::var("PURGER_SERVICE_INTERVAL")
            .context("missing 'PURGER_SERVICE_INTERVAL'")?
            .parse()?,
        pg_fqdn: std::env::var("PURGER_SERVICE_DB_FQDN")
            .context("missing 'PURGER_SERVICE_DB_FQDN'")?,
        pg_username: std::env::var("PURGER_SERVICE_DB_USER")
            .context("missing 'PURGER_SERVICE_DB_USER'")?,
        pg_password: std::env::var("PURGER_SERVICE_DB_PASSWORD")
            .context("missing 'PURGER_SERVICE_DB_PASSWORD'")?,
        pg_dbname: std::env::var("PURGER_SERVICE_DB_NAME")
            .context("missing 'PURGER_SERVICE_DB_NAME'")?,
        pg_max_conns: std::env::var("PURGER_SERVICE_DB_MAX_CONNS")
            .context("missing 'PURGER_SERVICE_DB_MAX_CONNS'")?
            .parse()?,
    };

    OBJECT.set(object).expect("config was already initialized");

    outputln!("configuration was loaded");

    Ok(())
}
