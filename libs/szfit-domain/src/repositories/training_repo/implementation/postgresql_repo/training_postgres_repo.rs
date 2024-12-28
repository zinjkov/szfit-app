use crate::entity::{Id, Training};
use crate::repositories::error::{RepoResult, RepositoryError};
use crate::repositories::{ITrainingRepository, TrainingForCreate, TrainingForUpdate};
use crate::store;
use chrono::Utc;
use dill::component;
use sqlx::postgres::PgRow;
use sqlx::{Execute, QueryBuilder, Row};

#[component]
pub(crate) struct PostgresqlTrainingRepository {
    db: store::Db,
}

impl PostgresqlTrainingRepository {
    pub fn new(db: store::Db) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl ITrainingRepository for PostgresqlTrainingRepository {
    async fn create(&self, tfc: TrainingForCreate) -> RepoResult<Training> {
        let name = tfc.name.unwrap_or(Utc::now().date_naive().to_string());
        Ok(sqlx::query_as!(Training,
                "INSERT INTO training (name, user_id, workout_plan_id) \
                    VALUES ($1, $2, $3) \
                    RETURNING *;",
                name,
                *tfc.user_id,
                *tfc.workout_plan_id)
            .fetch_all(&self.db)
            .await?
            .into_iter()
            .nth(0)
            .ok_or(RepositoryError::Other)?)
    }

    async fn list(&self, user_id: Id, limit: usize, offset: usize) -> RepoResult<Vec<Training>> {
        Ok(sqlx::query_as!(Training,
                r#"SELECT *
                    FROM training
                    WHERE user_id = $1
                    OFFSET $3
                    LIMIT $2"#,
                *user_id, limit as i64, offset as i64)
            .fetch_all(&self.db)
            .await?)
    }

    async fn update(&self, training_id: Id, tfu: TrainingForUpdate) -> RepoResult<Training> {
        let mut builder = QueryBuilder::new("UPDATE training SET");
        if let Some(name) = tfu.name{
            builder.push(" name = ");
            builder.push_bind(name);
        }
        if let Some(plan_id) = tfu.workout_plan_id {
            builder.push(", workout_plan_id = ");
            builder.push_bind(plan_id);
        }

        if let Some(finished_at) = tfu.finished_at {
            builder.push(", finished_at = ");
            builder.push_bind(sqlx::types::chrono::NaiveDateTime::from(finished_at));
        }
        builder.push(" WHERE id = ");
        builder.push_bind(*training_id);
        builder.push(" RETURNING * ");

        let query = builder.build();
        println!("q {:?}", query.sql());
        let training = query
            .try_map(|row: PgRow| {
                let id = row.try_get_unchecked::<i64, _>(0usize)?.into();
                let name = row.try_get_unchecked::<String, _>(1usize)?.into();
                let workout_plan_id = row.try_get_unchecked::<i64, _>(2usize)?.into();
                let user_id = row.try_get_unchecked::<i64, _>(3usize)?.into();
                let created_at = row.try_get_unchecked::<chrono::NaiveDateTime, _>(4usize)?.into();
                let finished_at = row.try_get_unchecked::<Option<chrono::NaiveDateTime>, _>(5usize)?.into();
                Ok(Training {
                    id,
                    name,
                    workout_plan_id,
                    user_id,
                    created_at,
                    finished_at,
                })
            }).fetch_one(&self.db).await?;
        Ok(training)
    }
}