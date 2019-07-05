use hey_listen::sync::ParallelDispatcher as Dispatcher;
use log::{debug, error, info};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::standard::{DispatchError, StandardFramework},
    model::id::UserId,
    prelude::*,
};
use white_rabbit::Scheduler;

pub fn init_tasks() {}
