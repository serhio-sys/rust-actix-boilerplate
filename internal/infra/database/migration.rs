use std::path::Path;

use dependencies::log::info;
use diesel::pg::Pg;
use diesel_migrations::MigrationHarness;

pub fn migrate(conn: &mut impl MigrationHarness<Pg>, migration_path: &str) {
    let migration_path = Path::new(migration_path);
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
    if !conn.has_pending_migration(migrator.clone()).unwrap() {
        info!("Nothing to migrate");
    } else {
        conn.run_pending_migrations(migrator).expect("Could not run migrations");
        info!("Migrated successfully to latest migration");
    }
    info!("Migrations was passed");
}
