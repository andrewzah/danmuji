use serenity::{
    client::Context,
    framework::standard::{
        Args, CommandResult,
        macros::{command},
    },
    model::{channel::Message},
};

use crate::dispatch::*;

#[command]
#[aliases("add")]
fn set_reminder(context: &mut Context, msg: &Message, mut args: Args) -> CommandResult {

    let time: u64 = args.single()?;
    let repeat: bool = args.single()?;
    let args = args.rest().to_string();

    let scheduler = {
        let mut context = context.data.write();
        context.get_mut::<SchedulerKey>().expect("Expected Scheduler.").clone()
    };

    let dispatcher = {
        let mut context = context.data.write();
        context.get_mut::<DispatcherKey>().expect("Expected Dispatcher.").clone()
    };

    let http = context.http.clone();
    let msg = msg.clone();

    let mut scheduler = scheduler.write();
}
