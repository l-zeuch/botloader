use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use twilight_model::id::{marker::GuildMarker, Id};

#[derive(Debug, Error)]
pub enum TimerStoreError {
    #[error("inner error occurred: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

pub type TimerStoreResult<T> = Result<T, TimerStoreError>;

pub struct GetGuildTasksFilter {
    pub scope: ScopeSelector,
    pub namespace: Option<String>,
}

pub enum ScopeSelector {
    All,
    Guild,
    Plugin(u64),
}

#[async_trait::async_trait]
pub trait TimerStore: Send + Sync {
    async fn get_all_guild_interval_timers(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> TimerStoreResult<Vec<IntervalTimer>>;
    async fn update_interval_timer(
        &self,
        guild_id: Id<GuildMarker>,
        timer: IntervalTimer,
    ) -> TimerStoreResult<IntervalTimer>;
    // async fn update_interval_timers(&self, guild_id: Id<GuildMarker>);
    async fn del_interval_timer(
        &self,
        guild_id: Id<GuildMarker>,
        plugin_id: Option<u64>,
        timer_name: String,
    ) -> TimerStoreResult<bool>;

    async fn create_task(
        &self,
        guild_id: Id<GuildMarker>,
        plugin_id: Option<u64>,
        name: String,
        unique_key: Option<String>,
        data: serde_json::Value,
        at: DateTime<Utc>,
    ) -> TimerStoreResult<ScheduledTask>;

    async fn get_task_by_id(
        &self,
        guild_id: Id<GuildMarker>,
        id: u64,
    ) -> TimerStoreResult<Option<ScheduledTask>>;
    async fn get_task_by_key(
        &self,
        guild_id: Id<GuildMarker>,
        plugin_id: Option<u64>,
        name: String,
        key: String,
    ) -> TimerStoreResult<Option<ScheduledTask>>;
    async fn get_guild_tasks(
        &self,
        guild_id: Id<GuildMarker>,
        filter: GetGuildTasksFilter,
        id_after: u64,
        limit: usize,
    ) -> TimerStoreResult<Vec<ScheduledTask>>;

    /// Delete a task by the global unique ID
    async fn del_task_by_id(&self, guild_id: Id<GuildMarker>, id: u64) -> TimerStoreResult<u64>;

    /// Delete one or more tasks by their (guild_id, plugin_id, name) and unique key
    /// (does nothing to key = null tasks)
    async fn del_task_by_key(
        &self,
        guild_id: Id<GuildMarker>,
        plugin_id: Option<u64>,
        name: String,
        key: String,
    ) -> TimerStoreResult<u64>;

    /// Delete all tasks on a guild, optionally filtered by name and plugin
    async fn del_all_tasks(
        &self,
        guild_id: Id<GuildMarker>,
        plugin_id: Option<u64>,
        name: Option<String>,
    ) -> TimerStoreResult<u64>;

    // async fn get_next_task_time(
    //     &self,
    //     guild_id: Id<GuildMarker>,
    // ) -> TimerStoreResult<Option<DateTime<Utc>>>;
    // async fn get_triggered_tasks(
    //     &self,
    //     guild_id: Id<GuildMarker>,
    //     t: DateTime<Utc>,
    // ) -> TimerStoreResult<Vec<ScheduledTask>>;

    async fn get_task_count(&self, guild_id: Id<GuildMarker>) -> TimerStoreResult<u64>;

    async fn get_next_task_time(
        &self,
        guild_id: Id<GuildMarker>,
        ignore_ids: &[u64],
        buckets: &[TaskBucket],
    ) -> TimerStoreResult<Option<DateTime<Utc>>>;

    async fn get_triggered_tasks(
        &self,
        guild_id: Id<GuildMarker>,
        t: DateTime<Utc>,
        ignore_ids: &[u64],
        buckets: &[TaskBucket],
    ) -> TimerStoreResult<Vec<ScheduledTask>>;

    async fn delete_guild_timer_data(&self, guild_id: Id<GuildMarker>) -> TimerStoreResult<()>;
}

#[derive(Clone)]
pub struct IntervalTimer {
    pub name: String,
    pub interval: IntervalType,
    pub last_run: DateTime<Utc>,
    // pub script_id: u64,
    pub plugin_id: Option<u64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum IntervalType {
    Minutes(u64),
    Cron(String),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ScheduledTask {
    pub id: u64,
    pub name: String,
    pub plugin_id: Option<u64>,

    pub unique_key: Option<String>,

    pub data: serde_json::Value,
    pub execute_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskBucket {
    pub name: String,
    pub plugin_id: Option<u64>,
}
