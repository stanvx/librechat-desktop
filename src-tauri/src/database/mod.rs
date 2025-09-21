use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use sqlx::sqlite::{SqliteAutoVacuum, SqliteConnectOptions, SqliteJournalMode};
use sqlx::{SqlitePool, SqlitePoolOptions};

pub mod encryption;
pub mod migrations;

/// Configuration required to bootstrap the SQLite database layer.
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub path: PathBuf,
    pub create_if_missing: bool,
    pub max_connections: u32,
}

impl DatabaseConfig {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            create_if_missing: true,
            max_connections: 5,
        }
    }

    fn connect_options(&self) -> Result<SqliteConnectOptions> {
        let mut options = SqliteConnectOptions::new()
            .filename(&self.path)
            .create_if_missing(self.create_if_missing)
            .foreign_keys(true)
            .journal_mode(SqliteJournalMode::Wal)
            .auto_vacuum(SqliteAutoVacuum::Incremental);

        if !self.create_if_missing && !self.path.exists() {
            anyhow::bail!("database file does not exist: {}", self.path.display());
        }

        Ok(options)
    }
}

/// Primary database handle used throughout the backend services.
#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn initialize(config: &DatabaseConfig) -> Result<Self> {
        if let Some(parent) = config.path.parent() {
            if config.create_if_missing {
                fs::create_dir_all(parent)
                    .with_context(|| format!("failed to create database directory {:?}", parent))?;
            }
        }

        let options = config.connect_options()?;
        let pool = SqlitePoolOptions::new()
            .max_connections(config.max_connections)
            .connect_with(options)
            .await
            .with_context(|| "failed to connect to SQLite backend")?;

        migrations::apply_migrations(&pool).await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
