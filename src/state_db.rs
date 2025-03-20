use sqlx::SqlitePool;

pub mod models {
    pub struct Post {
        pub id: i64,
        pub content: String,
        pub replies: Vec<String>,
    }
}

#[derive(Clone)]
pub struct StateDB {
    pub pool: SqlitePool,
}

#[derive(thiserror::Error, Debug)]
pub enum DBError {
    #[error("Inner sqlx error: {0}")]
    Inner(#[from] sqlx::Error),
    #[error("Requested generation does not exist")]
    MissingGeneration,
}

impl StateDB {
    pub async fn write_generation(&self, content: String) -> Result<i64, DBError> {
        sqlx::query!(
            "INSERT INTO generations (content) VALUES (?) RETURNING id",
            content
        )
        .fetch_one(&self.pool)
        .await
        .map(|r| r.id)
        .map_err(DBError::Inner)
    }

    pub async fn write_post(&self, generation_id: i64) -> Result<(), DBError> {
        sqlx::query!(
            "INSERT INTO posts (generation_id) VALUES (?)",
            generation_id,
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(DBError::Inner)
    }

    pub async fn write_reply(&self, generation_id: i64, post_id: i64) -> Result<(), DBError> {
        sqlx::query!(
            "INSERT INTO replies (generation_id, post_id) VALUES (?, ?)",
            generation_id,
            post_id
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(DBError::Inner)
    }

    pub async fn get_content_by_post_id(&self, id: i64) -> Result<String, DBError> {
        sqlx::query!(
            "SELECT generations.content FROM posts LEFT JOIN generations ON posts.generation_id = generations.id WHERE posts.id = ?",
            id
        )
        .fetch_one(&self.pool)
        .await
        .map(|r| r.content.ok_or(DBError::MissingGeneration))?
    }

    // TODO this should return Result<Vec<Result<String, DBError>, DBError>
    pub async fn get_replies_by_post_id(&self, id: i64) -> Result<Vec<String>, DBError> {
        sqlx::query!("SELECT replies.id, generations.content FROM replies LEFT JOIN generations ON generations.id = replies.generation_id WHERE replies.post_id = ?", id)
                    .fetch_all(&self.pool)
                    .await?
                    .into_iter()
                    .map(|r| r.content.ok_or(DBError::MissingGeneration))
                    .collect()
    }

    pub async fn get_latest_posts(&self) -> Result<Vec<models::Post>, DBError> {
        futures::future::join_all(
            sqlx::query!(
                "
SELECT
  posts.id,
  generations.content
FROM posts
LEFT JOIN generations ON generations.id = posts.generation_id
ORDER BY timestamp DESC
LIMIT 10"
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(async |r| {
                Ok(models::Post {
                    id: r.id,
                    content: r.content.ok_or(DBError::MissingGeneration)?,
                    replies: self.get_replies_by_post_id(r.id).await?,
                })
            }),
        )
        .await
        .into_iter()
        .collect()
    }

    pub async fn get_posts_after_id(&self, after_id: i64) -> Result<Vec<models::Post>, DBError> {
        futures::future::join_all(
            sqlx::query!(
                "
SELECT
  posts.id,
  generations.content
FROM posts
LEFT JOIN generations ON generations.id = posts.generation_id
WHERE posts.id > ?
ORDER BY posts.timestamp DESC",
                after_id
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(async |r| {
                Ok(models::Post {
                    id: r.id,
                    content: r.content.ok_or(DBError::MissingGeneration)?,
                    replies: self.get_replies_by_post_id(r.id).await?,
                })
            }),
        )
        .await
        .into_iter()
        .collect()
    }
}
