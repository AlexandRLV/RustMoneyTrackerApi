use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::prelude::FromRow;

use super::base::{self, DbBmc};

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Fields, Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Fields, Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}

pub struct TaskBmc;

impl DbBmc for TaskBmc {
    const TABLE: &'static str = "task";
}

impl TaskBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, task_c: TaskForCreate) -> Result<i64> {
        base::create::<Self, _>(_ctx, mm, task_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
        base::list::<Self, _>(_ctx, mm).await
    }

    pub async fn update(
        _ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        task_u: TaskForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(_ctx, mm, id, task_u).await
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(_ctx, mm, id).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{_dev_utils, model};
    use serial_test::serial;

    #[allow(unused)]
    use super::*;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title";

        let task_c = TaskForCreate {
            title: fx_title.to_string(),
        };

        let id = TaskBmc::create(&ctx, &mm, task_c).await?;

        let task = TaskBmc::get(&ctx, &mm, id).await?;
        assert_eq!(task.title, fx_title);

        TaskBmc::delete(&ctx, &mm, id).await?;
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> model::Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = TaskBmc::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "task",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_ok() -> model::Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let titles = &["test_list_ok-task title1", "test_list_ok-task title2"];
        _dev_utils::seed_tasks(&ctx, &mm, titles).await?;

        let tasks = TaskBmc::list(&ctx, &mm).await?;
        println!("--> {tasks:?}");

        let tasks: Vec<Task> = tasks
            .into_iter()
            .filter(|task| task.title.starts_with("test_list_ok-task"))
            .collect();
        assert_eq!(2, tasks.len(), "number of tasks not match");

        for task in tasks.iter() {
            TaskBmc::delete(&ctx, &mm, task.id).await?;
        }

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> model::Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_update_ok - task 01";
        let fx_title_new = "test_update_ok - task 01 - new title";

        let fx_task = _dev_utils::seed_tasks(&ctx, &mm, &[fx_title])
            .await?
            .remove(0);

        TaskBmc::update(
            &ctx,
            &mm,
            fx_task.id,
            TaskForUpdate {
                title: Some(fx_title_new.to_string()),
            },
        )
        .await?;

        let task = TaskBmc::get(&ctx, &mm, fx_task.id).await?;
        assert_eq!(task.title, fx_title_new);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_err_not_found() -> model::Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = TaskBmc::delete(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "task",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
