use crate::{mailbox, server, store, types};

#[derive(Debug, Clone)]
pub struct User<A>
where
    A: Access,
{
    id: types::Id,
    name: String,
    email: String,
    _access: std::marker::PhantomData<A>,
}

impl<A> User<A>
where
    A: Access,
{
    pub fn id(&self) -> types::Id {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn make_proto(&self) -> mailbox::Proto {
        mailbox::Proto {
            name: self.name.clone(),
            email: self.email.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Dynamic {
    Regular(User<Regular>),
    Pending(User<Pending>),
}

pub trait Access: sealed::Access {}

#[derive(Debug, Clone)]
pub struct Regular;
#[derive(Debug, Clone)]
pub struct Pending;

mod sealed {
    pub trait Access {}

    impl<A> super::Access for A where A: Access {}

    impl Access for super::Regular {}
    impl Access for super::Pending {}
}

#[derive(Debug, Clone)]
pub struct Auth {
    store: store::Store,
}

impl Auth {
    pub fn new(store: store::Store) -> Self {
        Self { store }
    }
}

impl server::auth::Provider for Auth {
    type Ok = Dynamic;
    type Error = store::Error;

    async fn auth(&self, user: &str) -> Result<Option<Self::Ok>, Self::Error> {
        if let Some(user) = self.store.players().auth(user).await? {
            return Ok(Some(Dynamic::Regular(User {
                id: user.id,
                name: user.name,
                email: user.email,
                _access: std::marker::PhantomData,
            })));
        }

        if let Ok(Some(user)) = self.store.invites().auth(user).await {
            return Ok(Some(Dynamic::Pending(User {
                id: user.id,
                name: user.name,
                email: user.email,
                _access: std::marker::PhantomData,
            })));
        }

        Ok(None)
    }
}
