use crate::{
    entity::{Exercise, Id},
    repositories::{error::RepoResult, ExerciseForCreate, IExerciseRepository},
    store,
};
use dill::component;

#[component]
pub struct PostgresqlExerciseRepository {
    db: store::Db,
}

impl PostgresqlExerciseRepository {
    #[allow(unused)]
    pub fn new(db: store::Db) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl IExerciseRepository for PostgresqlExerciseRepository {
    async fn find_by_id(&self, exercise_id: Id) -> RepoResult<Exercise> {
        Ok(sqlx::query_as!(
            Exercise,
            r#"SELECT id, name
                    FROM exercise
                    WHERE id = $1"#,
            *exercise_id
        )
        .fetch_one(&self.db)
        .await?)
    }

    async fn list(
        &self, limit: usize, offset: usize,
    ) -> RepoResult<Vec<Exercise>> {
        Ok(sqlx::query_as!(
            Exercise,
            r#"SELECT id, name
                    FROM exercise
                    OFFSET $1
                    LIMIT $2"#,
            offset as i64,
            limit as i64
        )
        .fetch_all(&self.db)
        .await?)
    }

    async fn create(
        &self, exercise_for_insert: ExerciseForCreate,
    ) -> RepoResult<Exercise> {
        Ok(sqlx::query_as!(
            Exercise,
            "INSERT INTO exercise(name) \
                    VALUES ($1) \
                    RETURNING id, name",
            exercise_for_insert.name
        )
        .fetch_one(&self.db)
        .await?)
    }
}
