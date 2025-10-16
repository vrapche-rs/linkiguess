use std::sync::Arc;

use nats::asynk::Message;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{config, constants, errorln, outputln};

#[derive(serde::Serialize, serde::Deserialize)]
struct QueueObject {
    #[serde(rename = "i")]
    short_link_id: i64,
}

async fn open_db_connection() -> anyhow::Result<Pool<Postgres>> {
    let config = config::object();

    let connection_string = format!(
        "postgres://{}:{}@{}/{}",
        config.pg_username, config.pg_password, config.pg_fqdn, config.pg_dbname
    );

    Ok(PgPoolOptions::new()
        .max_connections(config.pg_max_conns)
        .connect(&connection_string)
        .await?)
}

async fn increment_link(message: Arc<Message>) -> anyhow::Result<()> {
    let message: QueueObject =
        serde_json::from_str(unsafe { str::from_utf8_unchecked(&message.data) })?;

    let db_conn = open_db_connection().await?;
    let db_txn = db_conn.begin().await?;

    sqlx::query(
        r#"
            UPDATE tb_link
            SET tb_link_count = tb_link_count + 1
            WHERE tb_link_id = $1
        "#,
    )
    .bind(&message.short_link_id)
    .execute(&db_conn)
    .await?;

    db_txn.commit().await?;

    Ok(())
}

async fn worker(mut rx: Receiver<Arc<Message>>) -> anyhow::Result<()> {
    while let Some(message) = rx.recv().await {
        if let Err(e) = increment_link(message).await {
            errorln!("incrementing link failed, error: {}", e.to_string());
        }
    }

    Ok(())
}

pub async fn start() -> anyhow::Result<()> {
    let config = config::object();
    let mut worker_index = 0;
    let workers: Vec<Sender<Arc<Message>>> = (0..config.batch_workers)
        .into_iter()
        .map(|_| {
            let (tx, rx) = tokio::sync::mpsc::channel::<Arc<Message>>(config.workers_queue);

            tokio::spawn(worker(rx));
            tx
        })
        .collect();

    loop {
        let queue_conn = nats::asynk::Options::with_token(&config.analytic_queue_pass)
            .connect(&config.analytic_queue_fqdn)
            .await?;

        let queue = queue_conn
            .queue_subscribe(
                constants::ANALYTIC_QUEUE_NAME,
                constants::ANALYTIC_QUEUE_NAME,
            )
            .await?;

        while let Some(message) = queue.next().await {
            let message = Arc::new(message);

            loop {
                worker_index = (worker_index + 1) % workers.len();

                if workers[worker_index].try_send(message.clone()).is_ok() {
                    break;
                }

                tokio::task::yield_now().await;
            }
        }

        outputln!("queue connection was closed, sleeping 10 seconds and restarting...");

        tokio::time::sleep(tokio::time::Duration::from_secs(
            constants::ANALYTIC_QUEUE_CONN_CLOSE_WAIT_SEC,
        ))
        .await;
    }
}
