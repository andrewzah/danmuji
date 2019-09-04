use log::{debug, error, info};
use serenity::{model::prelude::*, prelude::*};

use crate::{db, dispatch::*, models::message::NewMessage, utils, BotData};

//const BOT_ID: u64 = 592184706896756736;

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
        if let Some(guild_id) = msg.guild_id {
            if message_filter(&ctx, &msg) == false {
                info!("filter not activated: {}", &msg.content);
                match NewMessage::from_msg(&msg) {
                    Ok(message) => {
                        let data_lock = ctx
                            .data
                            .read()
                            .get::<BotData>()
                            .cloned()
                            .expect("Unable to get BotData");
                        let mut bot_data = data_lock.lock();

                        bot_data.message_queue.push(message);
                        bot_data.insert_messages();
                    },
                    Err(err) => error!("err creating msg: {}", err),
                }
            }

            check_reply(guild_id, &ctx, &msg);
        }
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

fn message_filter(ctx: &Context, msg: &Message) -> bool {
    // ignore all bots
    if msg.author.bot == true {
        return true;
    }

    let data = ctx.data.read();
    let mutex = data.get::<BotData>().expect("Expected BotData mutex");
    let bot_data = mutex.lock();

    // ignore disabled channels
    if bot_data
        .disabled_channel_ids
        .contains(msg.channel_id.as_u64())
    {
        return true;
    }

    content_filter(&msg.content)
}

/// Checks incoming message to determine if
/// it should continue or not.
fn content_filter(content: &str) -> bool {
    if utils::starts_with_link(&content) {
        return true;
    }

    // ignore self input commands
    if content.starts_with("yi ") {
        return true;
    }

    // TODO remove when deploying!
    if content.starts_with("di ") {
        return true;
    }

    // ignore reply commands
    if content.starts_with(">") {
        return true;
    }

    // ignore other command messages
    // don't ignore stuff like quotes/parens
    if let Some(c) = content.chars().next() {
        if c == '"' || c == '\'' || c == '(' || c == '-' {
            return false;
        }
        if c.is_ascii_punctuation() {
            return true;
        }
    }

    false
}

fn check_reply(guild_id: GuildId, ctx: &Context, msg: &Message) {
    if msg.content.starts_with(">") {
        if let Some(tag) = utils::parse_tag(&msg.content) {
            if let Some(reply) = db::get_reply(&tag, &guild_id.to_string()).ok() {
                let _ = utils::say(&msg.channel_id, &ctx, &reply.url);
            }
        }
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_filters_links_at_start_messages() {
        assert_eq!(true, content_filter("https://google.com/"));
        assert_eq!(true, content_filter("https://twitter.com/andrew_zah/status/1121706223919722496"));
        assert_eq!(true, content_filter("https://www.reddit.com/r/linux test"));
        assert_eq!(false, content_filter("test https://www.reddit.com/r/linux test"));
    }

    #[test]
    fn it_filters_or_doesnt_correctly_in_general() {
        assert_eq!(false, content_filter("50% is accurate."));
    }
}
