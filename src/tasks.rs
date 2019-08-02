use std::{env, sync::Arc};

use hey_listen::sync::ParallelDispatcher as Dispatcher;
use log::{debug, error, info};
use serde_json::json;
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
    //let http = ctx.http.clone();
    let mut scheduler = scheduler.write();
    let http = ctx.http.clone();
    let chan_id = 500839254306455553_u64;

    scheduler.add_task_duration(interval, move |_| {
        info!("Running ratio task.");

        //let ratio_list = match db::get_ratio_list() {
            //Ok(list) => list,
            //Err(err) => {
                //error!("Unable to get ratio list: {}", err);
                //return DateResult::Repeat(Utc::now() + interval);
            //},
        //};

        let channel_id = ChannelId(chan_id);

        //match channel_id.send_message(&http, |m| m.content(ratio_list.pretty_print(&ctx.http))) {
        //Ok(_) => {}
        //Err(err) => error!("err sending msg: {}", err),
        //};

        info!("Finished ratio task.");
        DateResult::Repeat(Utc::now() + interval)
    });
}
