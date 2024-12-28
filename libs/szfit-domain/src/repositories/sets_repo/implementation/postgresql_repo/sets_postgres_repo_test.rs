use crate::repositories::sets_repo::implementation::postgresql_repo::sets_postgres_repo::PostgresqlSetsRepository;
use crate::repositories::{ISetsRepository, SetsForCreate};
use sqlx::PgPool;
use crate::entity::Id;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

fn prepare_test(db: PgPool) -> impl ISetsRepository {
    PostgresqlSetsRepository::new(db)
}

// async fn create(&self, sets: Vec<SetsForCreate>) -> RepoResult<()>;
#[sqlx::test(migrator = "MIGRATOR", fixtures(path = "fixtures", scripts("test_create")))]
async fn test_create(db: PgPool) {
    let repo = prepare_test(db);

    repo.create(vec![SetsForCreate {
        weight_kg: 0.0,
        reps: 0,
        user_id: Id(1),
        exercise_id: Id(1),
        training_id: Id(1),
    }])
        .await
        .expect("creating the sets repo failed");
}

// async fn last_max_set(&self, user_id: Id, exercise_id: Id) -> RepoResult<Sets>;
#[sqlx::test(migrator = "MIGRATOR", fixtures(path = "fixtures", scripts("test_last_max_set")))]
async fn test_last_max_set(db: PgPool) {
    let repo = prepare_test(db);

    let set = repo.last_max_set(Id(1), Id(1))
        .await
        .expect(
            "last max sets should be stored in Postgres",
        );

    assert_eq!(*set.id, 3);
    // assert_eq!(set.reps, 1);
}