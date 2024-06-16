use crate::mailbox;

pub enum Payload {
    Invite(mailbox::Mailbox),
    InviteOutcome {
        inviter: mailbox::Proto,
        invitee: mailbox::Proto,
        accepted: bool,
    },
}
