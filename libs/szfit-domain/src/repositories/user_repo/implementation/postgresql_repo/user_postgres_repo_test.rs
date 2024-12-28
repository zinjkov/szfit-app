use crate::repositories::user_repo::implementation::postgresql_repo::user_postgres_repo::PostgresqlUserRepo;
use sqlx::PgPool;
use crate::entity::Id;
use crate::repositories::IUserRepository;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

fn prepare_test(db: PgPool) -> impl IUserRepository {
    PostgresqlUserRepo::new(db)
}

// async fn find_by_tg(&self, telegram_id: Id) -> RepoResult<User>;,
#[sqlx::test(migrator = "MIGRATOR", fixtures(path = "fixtures", scripts("test_find")))]
async fn test_find_by_tg(db: PgPool) {
    let repo = prepare_test(db);

    let user = repo.find_by_tg(Id(11)).await.expect("user not found");
    assert_eq!(*user.telegram_id, 11);
    assert_eq!(*user.id, 1);
}

// async fn find_by_id(&self, id: Id) -> RepoResult<User>;
#[sqlx::test(migrator = "MIGRATOR", fixtures(path = "fixtures", scripts("test_find")))]
async fn test_find_by_id(db: PgPool) {
    let repo = prepare_test(db);

    let user = repo.find_by_id(Id(1)).await.expect("user not found");
    assert_eq!(*user.telegram_id, 11);
    assert_eq!(*user.id, 1);
}
// async fn find_by_tg_or_create(&self, telegram_id: Id) -> RepoResult<User>;
#[sqlx::test(migrator = "MIGRATOR", fixtures(path = "fixtures", scripts("test_find")))]
async fn test_find_by_tg_or_create_with_exist_user(db: PgPool) {
    let repo = prepare_test(db);

    let user = repo.find_by_tg_or_create(Id(11)).await.expect("user not found");
    assert_eq!(*user.telegram_id, 11);
    assert_eq!(*user.id, 1);
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn test_find_by_tg_or_create_with_create(db: PgPool) {
    let repo = prepare_test(db);

    let user = repo.find_by_tg_or_create(Id(11)).await.expect("user not created");
    assert_eq!(*user.telegram_id, 11);
    assert_eq!(*user.id, 1);
}

// async fn create(&self, telegram_id: Id) -> RepoResult<User>;
#[sqlx::test(migrator = "MIGRATOR")]
async fn test_create(db: PgPool) {
    let repo = prepare_test(db);

    let user = repo.create(Id(11)).await.expect("Failed to create user");
    assert_eq!(*user.telegram_id, 11);
    assert_eq!(*user.id, 1);
}
