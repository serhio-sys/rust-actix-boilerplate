use std::path::Path;

use chrono::NaiveDateTime;
use config::CONFIGURATION;
use config::log::{ info, error };
use diesel::{
    migration::{ Migration, MigrationSource },
    pg::Pg,
    r2d2::{ ConnectionManager, Pool },
    PgConnection,
};
use diesel_migrations::{ FileBasedMigrations, MigrationHarness };

const DATE_FORMAT: &str = "%Y-%m-%d-%H%M%S";

pub fn migrate() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let manager = ConnectionManager::<PgConnection>::new(
        &format!(
            "postgres://{}:{}@{}/{}?sslmode=disable",
            CONFIGURATION.database_user,
            CONFIGURATION.database_password,
            CONFIGURATION.database_host,
            CONFIGURATION.database_name
        )
    );
    let pool = Pool::builder()
        .max_size(5)
        .connection_timeout(std::time::Duration::from_secs(5))
        .build(manager)
        .unwrap_or_else(|_| panic!("Error: Unable to establish database connection."));

    let migration_path = Path::new(&CONFIGURATION.migration_location);
    let migrator = diesel_migrations::FileBasedMigrations
        ::find_migrations_directory_in_path(migration_path)
        .map_err(|err| format!("Error in creating FileBasedMigrations: {}", err))
        .unwrap();
    //conn.revert_all_migrations(migrator.clone()).expect("Could not revert migrations");
    if CONFIGURATION.migration_version != "latest" {
        migrate_to_version(&migrator, &pool);
    } else {
        let is_pending_migrations = !MigrationHarness::has_pending_migration(
            &mut pool.get().unwrap(),
            migrator.clone()
        ).unwrap();
        if is_pending_migrations {
            info!("Nothing to migrate");
        } else {
            MigrationHarness::run_pending_migrations(&mut pool.get().unwrap(), migrator)?;
            info!("Migrated successfully to latest migration");
        }
    }
    info!("Migrations was passed");
    return Ok(());
}

fn migrate_to_version(
    migrator: &FileBasedMigrations,
    connection: &Pool<ConnectionManager<PgConnection>>
) {
    let parsed_date = NaiveDateTime::parse_from_str(&CONFIGURATION.migration_version, DATE_FORMAT);
    if parsed_date.is_err() {
        panic!("Migration version is not parseble. Check migration version");
    }
    let migrations: Vec<Box<dyn Migration<Pg>>> = FileBasedMigrations::migrations(
        migrator
    ).unwrap();
    for migration in migrations {
        let name = migration.name().to_string();
        if name.contains("diesel_initial_setup") {
            continue;
        }
        let splitted_name: Vec<&str> = name.split("_").collect();
        match NaiveDateTime::parse_from_str(splitted_name.first().unwrap(), DATE_FORMAT) {
            Ok(parsed_datetime) => {
                if parsed_datetime <= parsed_date.unwrap() {
                    if
                        let Err(e) = MigrationHarness::run_migration(
                            &mut connection.get().unwrap(),
                            &migration
                        )
                    {
                        error!("Run migration error: {}", e.to_string());
                    } else {
                        info!("Executed migration: {}", name);
                    }
                } else {
                    if
                        let Err(e) = MigrationHarness::revert_migration(
                            &mut connection.get().unwrap(),
                            &migration
                        )
                    {
                        error!("Revert migration error: {}", e.to_string());
                    } else {
                        info!("Reverted migration: {}", name);
                    }
                }
            }
            Err(e) => {
                error!("Error parsing date: {}", e);
            }
        }
    }
}
