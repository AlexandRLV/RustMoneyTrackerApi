use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::prelude::FromRow;
use crate::model::task::{TaskBmc, TaskForUpdate};
use super::base::{self, DbBmc};

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct UserSaving {
    pub id: i64,
    pub user_id: i64,
    pub currency_id: i64,
    pub amount: f64,
    pub rate: f64,
}

#[derive(Fields, Deserialize)]
pub struct UserSavingDataModel {
    pub user_id: i64,
    pub currency_id: i64,
    pub amount: f64,
    pub rate: f64,
}

pub struct UserSavingsBmc;

impl DbBmc for UserSavingsBmc {
    const TABLE: &'static str = "user_savings";
}

impl UserSavingsBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, data: UserSavingDataModel) -> Result<i64> {
        // TODO: check if currency and user exist
        base::create::<Self, _>(_ctx, mm, data).await
    }

    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<UserSaving> {
        base::get::<Self, _>(_ctx, mm, id).await
    }

    pub async fn get_all(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<UserSaving>> {
        base::list::<Self, _>(_ctx, mm).await
    }

    pub async fn update(
        _ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        data: UserSavingDataModel,
    ) -> Result<()> {
        // TODO: check if currency and user exist
        base::update::<Self, _>(_ctx, mm, id, data).await
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(_ctx, mm, id).await
    }
}