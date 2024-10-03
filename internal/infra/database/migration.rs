use std::path::Path;

use config::CONFIGURATION;
use config::log::info;
use diesel::{ r2d2::{ ConnectionManager, Pool }, PgConnection };
use diesel_migrations::MigrationHarness;

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
    // TODO: migrate to version
    //let migrations: Vec<Box<dyn Migration<Pg>>> = FileBasedMigrations::migrations(
    //    &migrator
    //).unwrap();
    //for item in migrations {
    //    println!("{}", item.name());
    //}
    //conn.revert_all_migrations(migrator.clone()).expect("Could not revert migrations");
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
    info!("Migrations was passed");
    return Ok(());
}
