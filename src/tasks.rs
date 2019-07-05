use std::{env, sync::Arc};

use hey_listen::sync::ParallelDispatcher as Dispatcher;
use log::{debug, error, info};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::standard::{DispatchError, StandardFramework},
    model::prelude::*,
    prelude::*,
};
use white_rabbit::{DateResult, Duration, Scheduler, Utc};

use crate::{
    db::{self, *},
    dispatch::*,
};

pub fn init_tasks(ctx: &Context) {
    populate_users(ctx);
}

fn build_scheduler_dispatcher(
    ctx: &Context,
) -> (
    Arc<RwLock<Scheduler>>,
    Arc<RwLock<Dispatcher<DispatchEvent>>>,
) {
    let clone_ctx = ctx.clone();

    let scheduler = {
        let mut context = clone_ctx.data.write();
        context
            .get_mut::<SchedulerKey>()
            .expect("Expected Scheduler.")
            .clone()
    };

    let dispatcher = {
        let mut context = clone_ctx.data.write();
        context
            .get_mut::<DispatcherKey>()
            .expect("Expected Dispatcher.")
            .clone()
    };

    (scheduler, dispatcher)
}

fn populate_users(ctx: &Context) {
    let (scheduler, dispatcher) = build_scheduler_dispatcher(&ctx);

    //let interval = Duration::milliseconds(300_000 as i64); // 5 minutes in ms
    let interval = Duration::milliseconds(10_000 as i64);
    let http = ctx.http.clone();
    let mut scheduler = scheduler.write();

    scheduler.add_task_duration(interval, move |_| {
        info!("Running task.");
        
        let messages = db::get_messages(&ctx);

        info!("Finished task.");
        DateResult::Repeat(Utc::now() + interval)
    });
}
