use std::sync::OnceLock;

use anyhow::Context;

use crate::outputln;

#[derive(Debug)]
pub struct Object {
    pub log_days_retention: usize,
    pub batch_workers: u32,
    pub workers_queue: usize,
    pub pg_fqdn: String,
    pub pg_username: String,
    pub pg_password: String,
    pub pg_dbname: String,
    pub pg_max_conns: u32,
    pub analytic_queue_fqdn: String,
    pub analytic_queue_pass: String,
}

static OBJECT: OnceLock<Object> = OnceLock::new();

pub fn object<'a>() -> &'a Object {
    OBJECT.get().expect("config was not initialized")
}

pub fn load() -> anyhow::Result<()> {
    let object: Object = Object {
        log_days_retention: std::env::var("ANALYTIC_SERVICE_LOG_RETENTION")
            .context("missing 'ANALYTIC_SERVICE_LOG_RETENTION'")?
            .parse()?,
        batch_workers: std::env::var("ANALYTIC_BATCH_WORKERS_COUNT")
            .context("missing 'ANALYTIC_BATCH_WORKERS_COUNT'")?
            .parse()?,
        workers_queue: std::env::var("ANALYTIC_WORKER_MESSAGE_QUEUE")
            .context("missing 'ANALYTIC_WORKER_MESSAGE_QUEUE'")?
            .parse()?,
        pg_fqdn: std::env::var("ANALYTIC_SERVICE_DB_FQDN")
            .context("missing 'ANALYTIC_SERVICE_DB_FQDN'")?,
        pg_username: std::env::var("ANALYTIC_SERVICE_DB_USER")
            .context("missing 'ANALYTIC_SERVICE_DB_USER'")?,
        pg_password: std::env::var("ANALYTIC_SERVICE_DB_PASSWORD")
            .context("missing 'ANALYTIC_SERVICE_DB_PASSWORD'")?,
        pg_dbname: std::env::var("ANALYTIC_SERVICE_DB_NAME")
            .context("missing 'ANALYTIC_SERVICE_DB_NAME'")?,
        pg_max_conns: std::env::var("ANALYTIC_SERVICE_DB_MAX_CONNS")
            .context("missing 'ANALYTIC_SERVICE_DB_MAX_CONNS'")?
            .parse()?,
        analytic_queue_fqdn: std::env::var("ANALYTIC_SERVICE_ANALYTIC_QUEUE_FQDN")
            .context("missing 'ANALYTIC_SERVICE_ANALYTIC_QUEUE_FQDN'")?
            .parse()?,
        analytic_queue_pass: std::env::var("ANALYTIC_SERVICE_ANALYTIC_QUEUE_PASS")
            .context("missing 'ANALYTIC_SERVICE_ANALYTIC_QUEUE_PASS'")?
            .parse()?,
    };

    OBJECT.set(object).expect("config was already initialized");

    outputln!("configuration was loaded");

    Ok(())
}
