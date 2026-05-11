use crate::env::ENV;
use crate::middleware::database::db_pool;
use crate::middleware::error::ApiResult;
use crate::service::cms;
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn bind() -> ApiResult<()> {
    if !ENV.cron {
        return Ok(())
    }

    let sched = JobScheduler::new().await.unwrap();

    // DatCertificate Generate Cron
    cms::generate(db_pool()).await?; // initial generate
    sched.add(
        Job::new_async("0 0/10 * * * *", |_,_| {
            Box::pin(async move {
                tracing::info!("DatCertificate Generate Cron");
                cms::generate(db_pool()).await.unwrap();
            })
        }).unwrap(),
    ).await.unwrap();

    sched.start().await.unwrap();

    Ok(())
}
