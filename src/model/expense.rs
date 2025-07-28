use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::prelude::FromRow;
use crate::model::currency::{UserCurrency, UserCurrencyDataModel};
use crate::model::task::{TaskBmc, TaskForUpdate};
use super::base::{self, DbBmc};

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Expense {
    pub id: i64,
    pub description: String,
    pub currency_id: i64,
    pub date_timestamp: i64,
    pub amount: f32,
}

#[derive(Fields, Deserialize)]
pub struct ExpenseDataModel {
    pub description: String,
    pub currency_id: i64,
    pub date_timestamp: i64,
    pub amount: f32,
}

pub struct ExpenseBmc;

impl DbBmc for ExpenseBmc {
    const TABLE: &'static str = "user_expenses";
}

impl ExpenseBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, data: ExpenseDataModel) -> Result<i64> {
        // TODO: check if currency exists
        // TODO: check if user exists
        base::create::<Self, _>(_ctx, mm, data).await
    }

    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Expense> {
        base::get::<Self, _>(_ctx, mm, id).await
    }

    pub async fn get_all(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Expense>> {
        base::list::<Self, _>(_ctx, mm).await
    }

    pub async fn update(
        _ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        data: ExpenseDataModel,
    ) -> Result<()> {
        // TODO: check if currency exists
        // TODO: check if user exists
        base::update::<Self, _>(_ctx, mm, id, data).await
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(_ctx, mm, id).await
    }
}