use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

use hey_listen::sync::{
    ParallelDispatcher as Dispatcher,
    ParallelDispatcherRequest as DispatcherRequest,
};
use serenity::{model::prelude::*, prelude::*};
use white_rabbit::Scheduler;

#[derive(Clone)]
pub enum DispatchEvent {
    ReactEvent(MessageId, UserId),
}

impl PartialEq for DispatchEvent {
    fn eq(&self, other: &DispatchEvent) -> bool {
        match (self, other) {
            (
                DispatchEvent::ReactEvent(self_message_id, self_user_id),
                DispatchEvent::ReactEvent(other_message_id, other_user_id),
            ) => self_message_id == other_message_id && self_user_id == other_user_id,
        }
    }
}

impl Eq for DispatchEvent {}

impl Hash for DispatchEvent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            DispatchEvent::ReactEvent(msg_id, user_id) => {
                msg_id.hash(state);
                user_id.hash(state);
            },
        }
    }
}
pub struct DispatcherKey;
impl TypeMapKey for DispatcherKey {
    type Value = Arc<RwLock<Dispatcher<DispatchEvent>>>;
}
pub struct SchedulerKey;
impl TypeMapKey for SchedulerKey {
    type Value = Arc<RwLock<Scheduler>>;
}
