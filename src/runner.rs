// #[cfg(not (test))]
use super::filename;
use crate::migrations::Migration;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct MigrationExecution {
    pub parsed: filename::Parsed,
}

#[derive(Debug, Clone)]
pub struct MigrationExecutionError;

pub fn run_migrations(
    migrations: &Vec<Migration>,
) -> Result<Vec<MigrationExecution>, MigrationExecutionError> {
    let mut migration_exections: Vec<MigrationExecution> = Vec::new();
    for migration in migrations {
        println!("{:?}", migration);
    }
    return Ok(migration_exections);
}
