use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::prelude::FromRow;
use crate::model::task::{TaskBmc, TaskForUpdate};
use super::base::{self, DbBmc};

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct UserCurrency {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
}

#[derive(Fields, Deserialize)]
pub struct UserCurrencyDataModel {
    pub user_id: i64,
    pub name: String,
}

pub struct UserCurrencyBmc;

impl DbBmc for UserCurrencyBmc {
    const TABLE: &'static str = "user_currencies";
}

impl UserCurrencyBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, data: UserCurrencyDataModel) -> Result<i64> {
        // TODO: check if user exists
        base::create::<Self, _>(_ctx, mm, data).await
    }
    
    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<UserCurrency> {
        base::get::<Self, _>(_ctx, mm, id).await
    }
    
    pub async fn get_all(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<UserCurrency>> {
        base::list::<Self, _>(_ctx, mm).await
    }

    pub async fn update(
        _ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        data: UserCurrencyDataModel,
    ) -> Result<()> {
        // TODO: check if user exists
        base::update::<Self, _>(_ctx, mm, id, data).await
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(_ctx, mm, id).await
    }
}