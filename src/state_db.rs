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
    pub async fn write_generation(&self, gen_group: i64, content: String) -> Result<i64, DBError> {
        sqlx::query!(
            "INSERT INTO generations (generation_group_id, content) VALUES (?, ?) RETURNING id",
            gen_group,
            content
        )
        .fetch_one(&self.pool)
        .await
        .map(|r| r.id)
        .map_err(DBError::Inner)
    }

    pub async fn get_new_generation_group(&self) -> Result<i64, DBError> {
        sqlx::query!("INSERT INTO generation_groups DEFAULT VALUES RETURNING id",)
            .fetch_one(&self.pool)
            .await
            .map(|r| r.id)
            .map_err(DBError::Inner)
    }

    pub async fn group_is_used(&self, generation_id: i64) -> Result<bool, DBError> {
        sqlx::query!(
            "
SELECT used FROM generation_groups
LEFT JOIN generations on generation_groups.id = generations.generation_group_id
WHERE generations.id = ?
",
            generation_id
        )
        .fetch_one(&self.pool)
        .await
        .map(|r| r.used)
        .map_err(DBError::Inner)
    }

    pub async fn write_post(&self, generation_id: i64) -> Result<(), DBError> {
        sqlx::query!(
            "
BEGIN TRANSACTION;

UPDATE generation_groups
SET used = 1
WHERE id = (SELECT generation_group_id FROM generations WHERE id = ?);

INSERT INTO posts (generation_id) VALUES (?);

COMMIT TRANSACTION;
",
            generation_id,
            generation_id,
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(DBError::Inner)
    }

    pub async fn write_reply(&self, generation_id: i64, post_id: i64) -> Result<(), DBError> {
        sqlx::query!(
            "
BEGIN TRANSACTION;

UPDATE generation_groups
SET used = 1
WHERE id = (SELECT generation_group_id FROM generations WHERE id = ?);

INSERT INTO replies (generation_id, post_id) VALUES (?, ?);

COMMIT TRANSACTION;
",
            generation_id,
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
            "
SELECT generations.content
FROM posts
LEFT JOIN generations ON posts.generation_id = generations.id
WHERE posts.id = ?",
            id
        )
        .fetch_one(&self.pool)
        .await
        .map(|r| r.content.ok_or(DBError::MissingGeneration))?
    }

    pub async fn get_content_by_generation_id(
        &self,
        generation_id: i64,
    ) -> Result<String, DBError> {
        sqlx::query!(
            "
SELECT content
FROM generations
WHERE id = ?",
            generation_id
        )
        .fetch_one(&self.pool)
        .await
        .map(|r| r.content)
        .map_err(DBError::Inner)
    }

    // TODO this should return Result<Vec<Result<String, DBError>, DBError>
    pub async fn get_replies_by_post_id(&self, id: i64) -> Result<Vec<String>, DBError> {
        sqlx::query!(
            "
SELECT replies.id, generations.content
FROM replies
LEFT JOIN generations ON generations.id = replies.generation_id
WHERE replies.post_id = ?",
            id
        )
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

    pub async fn get_posts_before_id(&self, before_id: i64) -> Result<Vec<models::Post>, DBError> {
        futures::future::join_all(
            sqlx::query!(
                "
SELECT
  posts.id,
  generations.content
FROM posts
LEFT JOIN generations ON generations.id = posts.generation_id
WHERE posts.id < ?
ORDER BY posts.timestamp DESC
LIMIT 10",
                before_id
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
