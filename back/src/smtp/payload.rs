use crate::mailbox;

#[derive(Debug, Clone)]
pub enum Payload {
    Invite(mailbox::Mailbox),
    InviteOutcome {
        inviter: mailbox::Proto,
        invitee: mailbox::Proto,
        accepted: bool,
    },
}
