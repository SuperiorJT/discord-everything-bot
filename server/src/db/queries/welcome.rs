use sqlx::{Executor, Sqlite, SqlitePool};

use crate::modules::welcome::{WelcomeContent, WelcomeExpanded, WelcomeExpandedRow, WelcomeJoin, WelcomeJoinContent, WelcomeJoinDmContent, WelcomeJoinRolesContent, WelcomeLeaveContent};

#[derive(sqlx::FromRow, serde::Serialize, Debug)]
pub struct WelcomeModule {
    pub id: i64,
    pub guild_id: String,
    pub enabled: bool,
}

pub struct WelcomeModuleInsert {
    pub guild_id: u64,
    pub enabled: bool,
}

pub struct WelcomeQueries {
    pool: SqlitePool
}
impl WelcomeQueries {
    pub fn new(pool: SqlitePool) -> Self {
        WelcomeQueries {
            pool
        }
    }

    pub async fn module_expanded_fetch_by_guild_id(&self, guild_id: u64) -> sqlx::Result<WelcomeExpanded> {
        let guild_id = guild_id.to_string();
        let module: WelcomeExpandedRow = sqlx::query_as::<_, WelcomeExpandedRow>(
            r#"
            SELECT
                welcome.*,
                "join".enabled AS join_enabled,
                "join".message_type AS join_message_type,
                "join".channel_id AS join_channel_id,
                "join".content AS join_content,
                "join".embed AS join_embed,
                join_dm.enabled AS join_dm_enabled,
                join_dm.message_type AS join_dm_message_type,
                join_dm.content AS join_dm_content,
                join_dm.embed AS join_dm_embed,
                join_roles.enabled AS join_roles_enabled,
                leave.enabled AS leave_enabled
            FROM welcome
            LEFT JOIN welcome_join "join" ON "join".welcome_id = welcome.id
            LEFT JOIN welcome_join_dm join_dm ON join_dm.welcome_id = welcome.id
            LEFT JOIN welcome_join_roles join_roles ON join_roles.welcome_id = welcome.id
            LEFT JOIN welcome_leave leave ON leave.welcome_id = welcome.id
			WHERE welcome.guild_id = ?
            "#)
            .bind(guild_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(module.into())
    }

    pub async fn module_join_fetch_by_guild_id(&self, guild_id: u64) -> sqlx::Result<WelcomeJoin> {
        let guild_id = guild_id.to_string();
        let module: WelcomeJoin = sqlx::query_as::<_, WelcomeJoin>(
            r#"
            SELECT "join".*
            FROM welcome
            LEFT JOIN welcome_join "join" ON "join".welcome_id = welcome.id
			WHERE welcome.guild_id = ?
            "#)
            .bind(guild_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(module.into())
    }

    pub async fn module_insert(&self, data: WelcomeModuleInsert) -> sqlx::Result<WelcomeModule> {
        let row: (i64,) = sqlx::query_as(r#"INSERT INTO welcome (guild_id, enabled) VALUES (?, ?) RETURNING id"#).bind(data.guild_id.to_string()).bind(data.enabled).fetch_one(&self.pool).await?;

        Ok(WelcomeModule {
            id: row.0,
            guild_id: data.guild_id.to_string(),
            enabled: data.enabled
        })
    }

    pub async fn module_upsert_with(&self, guild_id: u64, data: &WelcomeContent, exec: impl Executor<'_, Database = Sqlite>) -> sqlx::Result<()> {
        sqlx::query("
            INSERT INTO welcome (guild_id, enabled) VALUES (?, ?)
            ON CONFLICT(guild_id)
            DO UPDATE SET
                enabled=excluded.enabled
            ")
            .bind(guild_id.to_string())
            .bind(data.enabled.or(Some(false)))
            .execute(exec)
            .await?;
        Ok(())
    }

    pub async fn module_join_upsert_with(&self, welcome_id: i64, data: &WelcomeJoinContent, exec: impl Executor<'_, Database = Sqlite>) -> sqlx::Result<()> {
        sqlx::query("
            INSERT INTO welcome_join (welcome_id, enabled, message_type, channel_id, content, embed) VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(welcome_id)
            DO UPDATE SET
                enabled=excluded.enabled,
                message_type=excluded.message_type,
                channel_id=excluded.channel_id,
                content=excluded.content,
                embed=excluded.embed
            ")
            .bind(welcome_id)
            .bind(data.enabled.or(Some(false)))
            .bind(data.message_type.clone().or(Some("text".to_string())))
            .bind(data.channel_id.clone())
            .bind(data.content.clone())
            .bind(data.embed.clone().and_then(|val| serde_json::to_string(&val).ok()))
            .execute(exec)
            .await?;
        Ok(())
    }

    pub async fn module_join_dm_upsert_with(&self, welcome_id: i64, data: &WelcomeJoinDmContent, exec: impl Executor<'_, Database = Sqlite>) -> sqlx::Result<()> {
        sqlx::query("
            INSERT INTO welcome_join_dm (welcome_id, enabled, message_type, content, embed) VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(welcome_id)
            DO UPDATE SET
                enabled=excluded.enabled,
                message_type=excluded.message_type,
                content=excluded.content,
                embed=excluded.embed
            ")
            .bind(welcome_id)
            .bind(data.enabled.or(Some(false)))
            .bind(data.message_type.clone().or(Some("text".to_string())))
            .bind(data.content.clone())
            .bind(data.embed.clone().and_then(|val| serde_json::to_string(&val).ok()))
            .execute(exec)
            .await?;
        Ok(())
    }

    pub async fn module_join_roles_upsert_with(&self, welcome_id: i64, data: &WelcomeJoinRolesContent, exec: impl Executor<'_, Database = Sqlite>) -> sqlx::Result<()> {
        sqlx::query("
            INSERT INTO welcome_join_roles (welcome_id, enabled) VALUES (?, ?)
            ON CONFLICT(welcome_id)
            DO UPDATE SET
                enabled=excluded.enabled
            ")
            .bind(welcome_id)
            .bind(data.enabled.or(Some(false)))
            .execute(exec)
            .await?;
        Ok(())
    }

    pub async fn module_leave_update_with(&self, welcome_id: i64, data: &WelcomeLeaveContent, exec: impl Executor<'_, Database = Sqlite>) -> sqlx::Result<()> {
        sqlx::query("
            INSERT INTO welcome_leave (welcome_id, enabled) VALUES (?, ?)
            ON CONFLICT(welcome_id)
            DO UPDATE SET
                enabled=excluded.enabled
            ")
            .bind(welcome_id)
            .bind(data.enabled.or(Some(false)))
            .execute(exec)
            .await?;
        Ok(())
    }
}