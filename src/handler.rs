use log::{debug, error, info};
use serenity::{model::prelude::*, prelude::*};

use crate::{db, dispatch::*, errors::*, models::message::NewMessage, tasks, utils, BotData};

const BOT_ID: u64 = 592184706896756736;

pub struct Handler;
impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        ctx.set_activity(Activity::playing("danmuji.andrewzah.com"));
        //TODO - finish up scheduled tasks/etc
        //tasks::init_tasks(&ctx);
        info!("Finished initializing startup tasks!");
    }

    fn message(&self, ctx: Context, msg: Message) {
        if message_filter(&ctx, &msg) == false {
            match NewMessage::from_msg(&msg) {
                Ok(message) => message.insert(),
                Err(err) => error!("err creating msg: {}", err),
            }
        };

        check_reply(&ctx, &msg);
    }

    fn resume(&self, _: Context, resume: ResumedEvent) {
        debug!("Resumed; trace: {:?}", resume.trace);
    }

    fn reaction_add(&self, context: Context, reaction: Reaction) {
        let dispatcher = {
            let mut context = context.data.write();
            context
                .get_mut::<DispatcherKey>()
                .expect("Expected Dispatcher.")
                .clone()
        };

        dispatcher
            .write()
            .dispatch_event(&DispatchEvent::ReactEvent(
                reaction.message_id,
                reaction.user_id,
            ));
    }
}

/// Checks incoming message to determine if
/// it should continue or not.
fn message_filter(ctx: &Context, msg: &Message) -> bool {
    let data = ctx.data.read();
    let bot_data = data.get::<BotData>().expect("Expected BotData");

    // ignore disabled channels
    if bot_data
        .read()
        .disabled_channel_ids
        .contains(msg.channel_id.as_u64())
    {
        return true;
    }

    // ignore self output
    if msg.author.id == BOT_ID {
        return true;
    }

    // ignore self input commands
    if msg.content.starts_with("yi ") {
        return true;
    }

    // ignore reply commands
    if msg.content.starts_with(">") {
        return true;
    }

    // ignore other command messages
    if let Some(c) = msg.content.chars().next() {
        if c.is_ascii_punctuation() {
            return true;
        }
    }

    false
}

fn check_reply(ctx: &Context, msg: &Message) {
    if let Some(guild_id) = msg.guild_id {
        if msg.content.starts_with(">") {
            if let Some(tag) = utils::parse_tag(&msg.content) {
                if let Some(reply) = db::get_reply(&tag, &guild_id.to_string()).ok() {
                    info!("reply: {:?}", &reply);
                    msg.channel_id.say(&ctx, &reply.url);
                }
            }
        };
    }
}
