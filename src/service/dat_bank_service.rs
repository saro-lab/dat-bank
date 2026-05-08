use crate::entity::dat_bank;
use crate::env::ENV;
use crate::middleware::error::ApiResult;
use dat::signature_key::SignatureKeyOutOption;
use dat::util::now_unix_timestamp;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

pub(crate) type KeyCount = usize;
pub(crate) type NewKid = i64;
pub(crate) type DeleteCount = u64;

pub async fn get_keys<C: ConnectionTrait>(signature_key_out_option: SignatureKeyOutOption, db: &C) -> ApiResult<(String, KeyCount)> {
    let keys = dat_bank::Entity::find().all(db).await?
        .iter().map(|e| e.to_dat_key_set().unwrap().export(signature_key_out_option).unwrap())
        .collect::<Vec<String>>();
    let count = keys.len();
    Ok((keys.join("\n"), count))
}

pub async fn generate<C: ConnectionTrait>(db: &C) -> ApiResult<(NewKid, DeleteCount)> {
    let delete_count = cleanup_expired(db).await?;
    let kid = dat_bank::ActiveModel::generate(ENV.signature, ENV.crypto, ENV.issue_begin(), ENV.issue_end(), ENV.token_ttl)?
        .save(db).await?.kid.unwrap();
    Ok((kid, delete_count))
}

async fn cleanup_expired<C: ConnectionTrait>(db: &C) -> ApiResult<u64> {
    let now = now_unix_timestamp();
    Ok(dat_bank::Entity::delete_many().filter(dat_bank::Column::ExpireTime.lt(now)).exec(db).await?.rows_affected)
}
