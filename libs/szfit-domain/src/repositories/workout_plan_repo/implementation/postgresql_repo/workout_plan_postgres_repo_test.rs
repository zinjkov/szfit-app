use crate::entity::Id;
use crate::repositories::workout_plan_repo::implementation::postgresql_repo::workout_plan_postgres_repo::PostgresqlWorkoutPlanRepo;

use crate::repositories::workout_plan_repo::workout_plan_repository::IWorkoutPlanRepository;
use sqlx::PgPool;
use std::env;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

fn prepare_test(db: PgPool) -> impl IWorkoutPlanRepository {
    PostgresqlWorkoutPlanRepo::new(db)
}
#[sqlx::test(migrator = "MIGRATOR", fixtures(path = "fixtures", scripts("for_update")))]
async fn test_update(db: PgPool) {
    let repo = prepare_test(db);
    const UPDATED_NAME: &str = "updated_name";
    let res = repo.update(Id(1), UPDATED_NAME.to_string())
        .await
        .expect("update failed");

    assert_eq!(*res.id, 1);
    assert_eq!(res.name, UPDATED_NAME.to_string());
}

#[sqlx::test(migrator = "MIGRATOR", fixtures(path = "fixtures", scripts("workout")))]
async fn test_list_with_limit(db: PgPool) {
    println!("{:?}", db.options());
    let var = env::var("DATABASE_URL").unwrap();
    println!("{:?}", var);
    let repo = PostgresqlWorkoutPlanRepo::new(db);
    let res = repo.list_with_limit(Id(1), 1)
        .await
        .expect("workout list not fetched");
    assert_eq!(res.len(), 1);
}

#[sqlx::test(migrator = "MIGRATOR", fixtures(path = "fixtures", scripts("for_delete")))]
async fn test_delete(db: PgPool) {
    let repo = prepare_test(db);
    repo.delete(Id(1))
        .await
        .expect("delete failed");
}

#[sqlx::test(migrator = "MIGRATOR", fixtures(path = "fixtures", scripts("for_create")))]
async fn test_create(db: PgPool) {
    let repo = prepare_test(db);
    let res = repo.create(Id(1), "test 1".into())
        .await
        .expect("create failed");
    assert_eq!(*res.id, 1);
    assert_eq!(res.name, "test 1");
}

#[sqlx::test(migrator = "MIGRATOR", fixtures(path = "fixtures", scripts("test_find_by_id")))]
async fn test_find_by_id(db: PgPool) {
    let res = prepare_test(db)
        .find_by_id(Id(1))
        .await
        .expect("find by failed");

    assert_eq!(*res.workout_id, 1);
    assert_eq!(res.workout_name, "test 1");
    assert_eq!(res.exercise_list.len(), 1);
}

#[sqlx::test(migrator = "MIGRATOR", fixtures(path = "fixtures", scripts("test_find_by_id")))]
async fn test_exercise_list_for_workout(db: PgPool) {
    let repo = prepare_test(db);
    let res = repo.exercises_for_workout(Id(1))
        .await
        .expect("exercise list not fetched");
    assert_eq!(res.len(), 1);
}

#[sqlx::test(migrator = "MIGRATOR", fixtures(path = "fixtures", scripts("add_exercise")))]
async fn test_add_exercise(db: PgPool) {
    let repo = PostgresqlWorkoutPlanRepo::new(db);
    let plan = repo.add_exercises(Id(1), vec![Id(1)])
        .await
        .expect("workout list not fetched");

    assert_eq!(*plan.workout_id, 1);
    assert_eq!(plan.workout_name, "test 1");
    assert_eq!(plan.exercise_list.len(), 1);

    let ex = plan.exercise_list.first().unwrap();
    assert_eq!(*ex.id, 1);
    assert_eq!(ex.name, "exercise 1");
}
