use std::net::TcpListener;

use bom_version_control::{
    configuration::get_config,
    db::{create_db_pool, models::db_component::DbComponent, DbPool},
    schema::components,
    startup::run,
};
use diesel::{insert_into, QueryDsl, RunQueryDsl};
use secrecy::ExposeSecret;
use uuid::Uuid;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_config().expect("Failed to read configuration");
    let pool = create_db_pool(config.db.conn_string().expose_secret());

    if let Err(e) = populate_components_table(&pool) {
        eprintln!("Failed to populate components table: {}", e);
    }

    let addr = format!("{}:{}", config.app.host, config.app.port);
    let listener = TcpListener::bind(addr).expect("Failed to bind to port");
    run(listener, pool)?.await?;
    Ok(())
}

fn populate_components_table(pool: &DbPool) -> Result<(), diesel::result::Error> {
    if is_components_table_empty(pool)? {
        let mut conn = pool.get().expect("Failed to get DB connection");
        let new_components: Vec<DbComponent> = (1..=100)
            .map(|i| DbComponent {
                id: Uuid::new_v4(),
                name: format!("Component {}", i),
                part_number: format!("PRT-{}", i),
                description: Some(format!("Description of component {}", i)),
                supplier: format!("Supplier {}", i),
                price_value: (rand::random::<f64>() * 100.0).floor() as i32,
                price_currency: "USD".to_string(),
            })
            .collect();

        insert_into(components::table)
            .values(&new_components)
            .execute(&mut conn)?;
    }
    Ok(())
}

fn is_components_table_empty(pool: &DbPool) -> Result<bool, diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let count = components::table.count().get_result::<i64>(&mut conn)?;
    Ok(count == 0)
}
