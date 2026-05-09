use sea_orm::{ConnectionTrait, DatabaseConnection, Schema};
use crate::middleware::error::ApiResult;

pub mod dat_certificates;

pub async fn create_all_table(db: &DatabaseConnection) -> ApiResult<()>
{
    // dat certificates
    db.execute(
        Schema::new(db.get_database_backend())
            .create_table_from_entity(dat_certificates::Entity)
            .if_not_exists()
    ).await?;

    Ok(())
}
