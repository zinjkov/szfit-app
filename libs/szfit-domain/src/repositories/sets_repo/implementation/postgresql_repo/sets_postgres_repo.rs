use crate::{
    entity::{Id, Sets},
    repositories::{
        error::RepoResult,
        sets_repo::sets_repository::{ISetsRepository, SetsForCreate},
    },
    store,
};
use dill::component;
use sqlx::QueryBuilder;

#[component]
pub struct PostgresqlSetsRepository {
    db: store::Db,
}

impl PostgresqlSetsRepository {
    #[allow(unused)]
    pub fn new(db: store::Db) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl ISetsRepository for PostgresqlSetsRepository {
    async fn create(
        &self, sets_for_insert: Vec<SetsForCreate>,
    ) -> RepoResult<()> {
        let mut builder = QueryBuilder::new(
            "INSERT INTO sets(weight_kg, reps, exercise_id, user_id, training_id)",
        );
        builder.push_values(sets_for_insert, |mut b, sfi| {
            b.push_bind(sfi.weight_kg)
                .push_bind(sfi.reps)
                .push_bind(*sfi.exercise_id)
                .push_bind(*sfi.user_id)
                .push_bind(*sfi.training_id);
        });
        let query = builder.build();
        query.execute(&self.db).await?;

        Ok(())
    }

    async fn last_max_set(
        &self, user_id: Id, exercise_id: Id,
    ) -> RepoResult<Sets> {
        Ok(sqlx::query_as!(
            Sets,
            r#"SELECT id, weight_kg, reps, exercise_id, user_id, created_at, training_id
                FROM sets
                WHERE exercise_id = $1 AND user_id = $2
                ORDER BY created_at DESC, reps * weight_kg DESC
                LIMIT 1;"#,
            *exercise_id,
            *user_id
        )
        .fetch_one(&self.db)
        .await?)
    }
}
