use crate::middleware::error::ApiResult;
use dat::crypto_algorithm::CryptoAlgorithm;
use dat::crypto_key::CryptoKey;
use dat::dat_key::DatKey;
use dat::error::DatError;
use dat::signature_algorithm::SignatureAlgorithm;
use dat::signature_key::SignatureKey;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::sea_query::StringLen;
use sea_orm::{ActiveModelBehavior, Set};
use serde::{Deserialize, Serialize};

// https://www.sea-ql.org/SeaORM/docs/generate-entity/column-types/
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "z_saro_dat_bank_v2")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sea_orm(column_type = "BigInteger")]
    pub kid: i64,

    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub signature_algorithm: String,

    pub signing_key: Vec<u8>,

    pub verifying_key: Vec<u8>,

    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub crypto_algorithm: String,

    pub crypto_key: Vec<u8>,

    #[sea_orm(column_type = "BigInteger")]
    pub issue_begin_time: i64,

    #[sea_orm(column_type = "BigInteger")]
    pub issue_end_time: i64,

    #[sea_orm(column_type = "BigInteger")]
    pub token_ttl: i64,

    #[sea_orm(column_type = "BigInteger")]
    pub expire_time: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Model {
    pub fn to_dat_key_set(&self) -> ApiResult<DatKey<i64>> {
        let signature_algorithm = self.signature_algorithm.parse::<SignatureAlgorithm>()?;
        let signature_key = SignatureKey::from_bytes(signature_algorithm, &self.signing_key, &self.verifying_key)?;
        let crypto_algorithm = self.crypto_algorithm.parse::<CryptoAlgorithm>()?;
        let crypto_key = CryptoKey::from_bytes(crypto_algorithm, &self.crypto_key)?;
        Ok(DatKey::from(
            self.kid,
            signature_key,
            crypto_key,
            self.issue_begin_time as u64,
            self.issue_end_time as u64,
            self.token_ttl as u64,
        )?)
    }
}

impl ActiveModel {
    pub fn generate(signature_algorithm: SignatureAlgorithm, crypto_algorithm: CryptoAlgorithm, issue_begin: u64, issue_end: u64, token_ttl: u64) -> Result<Self, DatError> {
        let key = DatKey::generate(0, signature_algorithm, crypto_algorithm, issue_begin, issue_end, token_ttl)?;
        let signature_key = key.signature_key();
        let (signing_key, verifying_key) = signature_key.to_bytes();
        let crypto_key = key.crypto_key();
        Ok(ActiveModel {
            signature_algorithm: Set(signature_key.algorithm().to_string()),
            signing_key: Set(signing_key.to_vec()),
            verifying_key: Set(verifying_key.to_vec()),
            crypto_algorithm: Set(crypto_key.algorithm().to_string()),
            crypto_key: Set(crypto_key.to_bytes().to_vec()),
            issue_begin_time: Set(issue_begin as i64),
            issue_end_time: Set(issue_end as i64),
            token_ttl: Set(token_ttl as i64),
            expire_time: Set((issue_end + token_ttl) as i64),
            ..Default::default()
        })
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
}
