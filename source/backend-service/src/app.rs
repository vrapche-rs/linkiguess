use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use anyhow::Context;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions, prelude::FromRow};

use crate::{config, constants, errorln};

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

#[get("/api/link")]
async fn get_links() -> impl Responder {
    #[derive(serde::Serialize, serde::Deserialize, FromRow)]
    struct Row {
        #[sqlx(rename = "tb_link_short")]
        short: String,
        #[sqlx(rename = "tb_link_long")]
        long: String,
        #[sqlx(rename = "tb_link_count")]
        count: i64,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Result {
        data: Vec<Row>,
    }

    let wrapper = async || {
        let db_conn = open_db_connection().await?;

        let rows: Vec<Row> = sqlx::query_as(
            r#"
                SELECT tb_link_short, tb_link_long, tb_link_count FROM tb_link;
            "#,
        )
        .fetch_all(&db_conn)
        .await?;

        Ok::<serde_json::Value, anyhow::Error>(serde_json::to_value(Result { data: rows })?)
    };

    match wrapper().await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => {
            errorln!("failed to query the database, {}", e.to_string());

            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn run() -> anyhow::Result<()> {
    HttpServer::new(|| App::new().service(get_links))
        .bind(constants::LISTEN_ADDRESS)
        .context("address already binded")?
        .run()
        .await?;

    Ok(())
}
