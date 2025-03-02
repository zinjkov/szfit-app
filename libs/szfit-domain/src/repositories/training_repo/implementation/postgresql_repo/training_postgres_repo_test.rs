use crate::entity::Id;
use crate::repositories::training_repo::implementation::postgresql_repo::training_postgres_repo::PostgresqlTrainingRepository;
use crate::repositories::{ITrainingRepository, TrainingForCreate, TrainingForUpdate};
use chrono::Utc;
use sqlx::PgPool;

pub static MIGRATOR: sqlx::migrate::Migrator =
    sqlx::migrate!("../../migrations");

fn prepare_test(db: PgPool) -> impl ITrainingRepository {
    PostgresqlTrainingRepository::new(db)
}

// async fn create(&self, tfc: TrainingForCreate) -> RepoResult<Training>;
#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "fixtures", scripts("test_create"))
)]
async fn test_create(db: PgPool) {
    let repo = prepare_test(db);

    let training = repo
        .create(TrainingForCreate {
            name: Some("training 1".to_string()),
            user_id: Id(1),
            workout_plan_id: Id(1),
        })
        .await
        .expect(
            "Error creating training-postgres-test repository for test",
        );

    assert_eq!(*training.id, 1);
    assert_eq!(training.name, "training 1");
    assert_eq!(*training.user_id, 1);
}
// async fn list(&self, user_id: Id) -> RepoResult<Vec<Training>>;
#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "fixtures", scripts("test_list"))
)]
async fn test_list(db: PgPool) {
    let repo = prepare_test(db);

    let first = repo.list(Id(1), 1, 0).await.expect(
        "Error fetching list training-postgres-test repository for test",
    );

    assert_eq!(first.len(), 1);
    assert_eq!(*first[0].id, 1);
    assert_eq!(first[0].name, "training 1");
    assert_eq!(*first[0].workout_plan_id, 1);
    assert_eq!(*first[0].user_id, 1);
    assert_eq!(first[0].finished_at, None);

    let second = repo.list(Id(1), 1, 1).await.expect(
        "Error fetching list training-postgres-test repository for test",
    );

    assert_eq!(second.len(), 1);
    assert_eq!(*second[0].id, 2);
    assert_eq!(second[0].name, "training 2");
    assert_eq!(*second[0].workout_plan_id, 1);
    assert_eq!(*second[0].user_id, 1);
    assert_eq!(second[0].finished_at, None);
}

// async fn update(&self, training_id: Id, tfu: TrainingForUpdate) -> RepoResult<Training>;
#[sqlx::test(
    migrator = "MIGRATOR",
    fixtures(path = "fixtures", scripts("test_list"))
)]
async fn test_update(db: PgPool) {
    let repo = prepare_test(db);
    let updated = TrainingForUpdate {
        name: Some("updated training 1".to_string()),
        workout_plan_id: Some(1),
        finished_at: Some(Utc::now().naive_utc()),
    };
    let training = repo.update(Id(1), updated).await.expect(
        "Error fetching list training-postgres-test repository for test",
    );

    assert_eq!(*training.id, 1);
    assert_eq!(training.name, "updated training 1");
    assert!(training.finished_at.is_some());
}
