use crate::aggregate::WorkoutPlan;
use crate::entity::{Exercise, Id, Workout};
use crate::repositories::error::{RepoResult, RepositoryError};
use crate::repositories::workout_plan_repo::workout_plan_repository::IWorkoutPlanRepository;
use crate::store;
use async_trait::async_trait;
use dill::component;
use sqlx::{query, QueryBuilder};

#[component]
pub struct PostgresqlWorkoutPlanRepo {
    db: store::Db,
}

impl PostgresqlWorkoutPlanRepo {
    pub fn new(db: store::Db) -> Self {
        Self {
            db,
        }
    }

    pub fn db(&self) -> &store::Db {
        &self.db
    }
}

#[async_trait]
impl IWorkoutPlanRepository for PostgresqlWorkoutPlanRepo {
    async fn list_with_limit_offset(
        &self,
        user_id: Id,
        limit: usize,
        offset: usize,
    ) -> RepoResult<Vec<Workout>> {
        Ok(sqlx::query_as!(
            Workout,
            r#"SELECT id, name
                    FROM workout_plan
                    WHERE user_id = $1
                    LIMIT $2
                    OFFSET $3"#,
            *user_id,
            limit as i64,
            offset as i64
        )
        .fetch_all(self.db())
        .await?)
    }

    async fn create(
        &self,
        user_id: Id,
        workout_name: String,
    ) -> RepoResult<Workout> {
        Ok(sqlx::query_as!(
            Workout,
            "INSERT INTO workout_plan(name, user_id)
                    VALUES ($1, $2)
                    RETURNING id, name",
            workout_name,
            *user_id
        )
        .fetch_one(self.db())
        .await?)
    }

    async fn delete(&self, workout_id: Id) -> RepoResult<()> {
        sqlx::query!(
            "DELETE FROM workout_plan WHERE id = $1",
            *workout_id
        )
        .execute(self.db())
        .await?;
        Ok(())
    }

    async fn update(
        &self,
        workout_id: Id,
        workout_name: String,
    ) -> RepoResult<Workout> {
        Ok(sqlx::query_as!(
            Workout,
            "UPDATE workout_plan
                    SET name = $1
                    WHERE id = $2
                    RETURNING id, name",
            workout_name,
            *workout_id
        )
        .fetch_one(self.db())
        .await?)
    }

    async fn find_by_id(&self, workout_id: Id) -> RepoResult<WorkoutPlan> {
        let res = query!(
            r#"SELECT workout_plan.id,
                        workout_plan.name,
                        exercise.id as exercise_id,
                        exercise.name as exercise_name
                    FROM workout_to_exercise
                    INNER JOIN exercise ON exercise.id = workout_to_exercise.exercise_id
                    INNER JOIN workout_plan ON workout_plan.id = workout_to_exercise.workout_plan_id
                    WHERE workout_plan.id = $1"#,
            *workout_id
        )
        .fetch_all(self.db())
        .await?;

        let mut plan_opt: Option<WorkoutPlan> = Option::default();
        for row in res {
            let plan = plan_opt.get_or_insert_with(|| WorkoutPlan {
                workout_id: Id(row.id),
                workout_name: row.name,
                exercise_list: Default::default(),
            });
            plan.exercise_list.push(Exercise::new(
                Id(row.exercise_id),
                row.exercise_name,
            ));
        }
        plan_opt.ok_or(RepositoryError::EntityNotFound)
    }

    async fn add_exercises(
        &self,
        workout_id: Id,
        exercise_ids: Vec<Id>,
    ) -> RepoResult<WorkoutPlan> {
        let mut builder = QueryBuilder::new(
            r#"INSERT INTO workout_to_exercise (workout_plan_id, exercise_id) "#,
        );
        builder.push_values(exercise_ids, |mut b, id| {
            b.push_bind(*workout_id).push_bind(*id);
        });
        let query = builder.build();
        query.execute(self.db()).await?;

        self.find_by_id(workout_id).await
    }

    async fn exercises_for_workout(
        &self,
        workout_id: Id,
    ) -> RepoResult<Vec<Exercise>> {
        Ok(sqlx::query_as!(
            Exercise,
            r#"SELECT exercise.id, name
                    FROM exercise
                    INNER JOIN workout_to_exercise AS wte
                        ON wte.exercise_id = exercise.id
                    WHERE wte.workout_plan_id = $1"#,
            *workout_id
        )
        .fetch_all(self.db())
        .await?)
    }
}
