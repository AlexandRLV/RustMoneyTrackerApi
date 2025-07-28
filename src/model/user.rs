use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::prelude::FromRow;
use crate::model::task::{TaskBmc, TaskForUpdate};
use super::base::{self, DbBmc};

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub login: String,
    pub password_hash: String,
    pub telegram_chat_id: i64,
}

#[derive(Fields, Deserialize)]
pub struct UserDataModel {
    pub user_id: i64,
    pub login: String,
    pub password_hash: String,
    pub telegram_chat_id: i64,
}

pub struct UserBmc;

impl DbBmc for UserBmc {
    const TABLE: &'static str = "users";
}

impl UserBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, data: UserDataModel) -> Result<i64> {
        base::create::<Self, _>(_ctx, mm, data).await
    }

    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<User> {
        base::get::<Self, _>(_ctx, mm, id).await
    }

    pub async fn get_all(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<User>> {
        base::list::<Self, _>(_ctx, mm).await
    }

    pub async fn update(
        _ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        data: UserDataModel,
    ) -> Result<()> {
        base::update::<Self, _>(_ctx, mm, id, data).await
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(_ctx, mm, id).await
    }
}