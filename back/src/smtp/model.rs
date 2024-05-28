use crate::{mailbox, types};

pub enum Payload {
    Invite(mailbox::Mailbox),
    _Challenge(types::Id),
    _Match(types::Id),
}
