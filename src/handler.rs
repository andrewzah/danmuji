use log::{debug, error, info};
use serenity::{model::prelude::*, prelude::*};

use crate::{db, dispatch::*, models::NewMessage, tasks, utils};

const BOT_ID: u64 = 592184706896756736;

pub struct Handler;
impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        tasks::init_tasks(&ctx);
    }

    fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == BOT_ID {
            return;
        }

        let m = NewMessage::from_msg(msg);
        m.write_to_db(&ctx);

        //parse_content(&formatted_content);
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

#[allow(dead_code)]
fn parse_content(content: &str) {
    let mut non_hangeul = 0;
    let mut hangeul = 0;
    let blocks = content.trim().split("");

    for block in blocks {
        for character in block.chars() {
            if utils::is_hangeul(character) {
                hangeul += 1;
            } else {
                non_hangeul += 1;
            }
        }
    }
}
