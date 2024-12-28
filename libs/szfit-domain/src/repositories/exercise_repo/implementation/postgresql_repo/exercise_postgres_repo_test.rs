use crate::repositories::exercise_repo::implementation::postgresql_repo::exercise_postgres_repo::PostgresqlExerciseRepository;
use crate::repositories::{ExerciseForCreate, IExerciseRepository};
use sqlx::PgPool;
use crate::entity::Id;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

fn prepare_test(db: PgPool) -> impl IExerciseRepository {
    PostgresqlExerciseRepository::new(db)
}
#[sqlx::test(migrator = "MIGRATOR", fixtures(path = "fixtures", scripts("test_list")))]
async fn test_list(db: PgPool) {
    let repo = prepare_test(db);

    let first = repo.list(1, 0)
        .await
        .expect("first fetch failed");
    assert_eq!(first.len(), 1);
    assert_eq!(*first[0].id, 1);
    assert_eq!(first[0].name, "exercise 1");

    let second = repo.list(1, 1)
        .await
        .expect("second fetch failed");

    assert_eq!(second.len(), 1);
    assert_eq!(*second[0].id, 2);
    assert_eq!(second[0].name, "exercise 2");
}

#[sqlx::test(migrator = "MIGRATOR", fixtures(path = "fixtures", scripts("test_list")))]
async fn test_find_by_id(db: PgPool) {
    let repo = prepare_test(db);

    let first = repo.find_by_id(Id(1))
        .await
        .expect("first fetch failed");

    assert_eq!(*first.id, 1);
    assert_eq!(first.name, "exercise 1");
}


#[sqlx::test(migrator = "MIGRATOR")]
async fn test_create(db: PgPool) {
    let repo = prepare_test(db);

    let exercise = repo.create(ExerciseForCreate {
        name: "exercise 1".to_string(),
    })
        .await
        .expect("first fetch failed");

    assert_eq!(*exercise.id, 1);
    assert_eq!(exercise.name, "exercise 1");

    let exercise = repo.find_by_id(Id(1))
        .await
        .expect("first fetch failed");

    assert_eq!(*exercise.id, 1);
    assert_eq!(exercise.name, "exercise 1");
}