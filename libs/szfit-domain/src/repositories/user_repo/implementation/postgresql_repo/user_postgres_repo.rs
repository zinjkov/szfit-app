use crate::{
    entity::{Id, User},
    repositories::{
        error::{RepoResult, RepositoryError},
        IUserRepository,
    },
    store,
};
use dill::component;

#[component]
pub struct PostgresqlUserRepo {
    db: store::Db,
}

impl PostgresqlUserRepo {
    #[allow(unused)]
    pub fn new(db: store::Db) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl IUserRepository for PostgresqlUserRepo {
    async fn find_by_tg(&self, telegram_id: Id) -> RepoResult<User> {
        sqlx::query_as!(
            User,
            "SELECT *
                    FROM users
                    WHERE users.telegram_id = $1",
            *telegram_id
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or(RepositoryError::EntityNotFound)
    }

    async fn find_by_id(&self, id: Id) -> RepoResult<User> {
        sqlx::query_as!(
            User,
            "SELECT *
                    FROM users
                    WHERE users.id = $1",
            *id
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or(RepositoryError::EntityNotFound)
    }

    async fn find_by_tg_or_create(&self, telegram_id: Id) -> RepoResult<User> {
        let user = self.find_by_tg(telegram_id).await;
        if user.is_err() {
            let user = self.create(telegram_id).await?;
            Ok(user)
        } else {
            user
        }
    }

    async fn create(&self, telegram_id: Id) -> RepoResult<User> {
        let user = sqlx::query_as!(
            User,
            r#"INSERT INTO users (telegram_id)
                 VALUES ($1)
                 RETURNING *;"#,
            *telegram_id
        )
        .fetch_one(&self.db)
        .await?;
        Ok(user)
    }
}
