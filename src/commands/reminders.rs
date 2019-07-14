use std::{
    collections::HashSet,
    env,
    hash::{Hash, Hasher},
    sync::Arc,
};

use hey_listen::sync::{
    ParallelDispatcher as Dispatcher,
    ParallelDispatcherRequest as DispatcherRequest,
};
use log::info;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    http::Http,
    model::prelude::*,
};
use white_rabbit::{DateResult, Duration, Scheduler, Utc};

use crate::dispatch::*;

fn thanks_for_reacting(
    http: Arc<Http>,
    channel: ChannelId,
) -> Box<Fn(&DispatchEvent) -> Option<DispatcherRequest> + Send + Sync> {
    Box::new(move |_| {
        if let Err(why) = channel.say(&http, "Thanks for reacting!") {
            println!("Could not send message: {:?}", why);
        }

        Some(DispatcherRequest::StopListening)
    })
}

#[command]
#[aliases("add")]
fn add_reminder(context: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let name: String = args.single()?;
    let time: u64 = args.single()?;
    let repeat: bool = args.single()?;
    let args = args.rest().to_string();

    let scheduler = {
        let mut context = context.data.write();
        context
            .get_mut::<SchedulerKey>()
            .expect("Expected Scheduler.")
            .clone()
    };

    let dispatcher = {
        let mut context = context.data.write();
        context
            .get_mut::<DispatcherKey>()
            .expect("Expected Dispatcher.")
            .clone()
    };

    let http = context.http.clone();
    let msg = msg.clone();
    let mut scheduler = scheduler.write();

    if repeat {
        scheduler.add_task_duration(Duration::milliseconds(time as i64), move |_| {
            let bot_msg = match msg.channel_id.say(&http, &args) {
                Ok(msg) => msg,
                Err(why) => {
                    println!("Error sending message: {:?}.", why);
                    return DateResult::Repeat(Utc::now() + Duration::milliseconds(5000));
                },
            };

            let http = http.clone();

            dispatcher.write().add_fn(
                DispatchEvent::ReactEvent(bot_msg.id, msg.author.id),
                thanks_for_reacting(http, bot_msg.channel_id),
            );

            DateResult::Repeat(Utc::now() + Duration::milliseconds(time as i64))
        });
    } else {
        scheduler.add_task_duration(Duration::milliseconds(time as i64), move |_| {
            let bot_msg = match msg.channel_id.say(&http, &args) {
                Ok(msg) => msg,
                Err(why) => {
                    println!("Error sending message: {:?}.", why);
                    return DateResult::Repeat(Utc::now() + Duration::milliseconds(5000));
                },
            };

            let http = http.clone();

            dispatcher.write().add_fn(
                DispatchEvent::ReactEvent(bot_msg.id, msg.author.id),
                thanks_for_reacting(http, bot_msg.channel_id),
            );

            // The task is done and that's it, we do not to repeat it.
            DateResult::Done
        });
    }

    Ok(())
}
