// TODO: This is completely independent. Move this to boile_rs?

pub trait Provider: 'static + Clone + Send {
    type Ok: Clone + Send + Sync;
    type Error: std::fmt::Display;

    fn auth(
        &self,
        user: &str,
    ) -> impl std::future::Future<Output = Result<Option<Self::Ok>, Self::Error>> + Send;
}

#[derive(Debug, Clone)]
pub struct Auth<P>
where
    P: Provider,
{
    provider: P,
}

impl<P> Auth<P>
where
    P: Provider,
{
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    #[tracing::instrument(skip_all)]
    async fn auth<B, I>(
        self,
        mut request: hyper::Request<B>,
        mut inner: I,
    ) -> Result<I::Response, I::Error>
    where
        I: tower_service::Service<hyper::Request<B>, Response = axum::response::Response>,
    {
        macro_rules! forbid {
            ($($arg: tt)*) => {
                tracing::warn!($($arg)*);
                return Ok(axum::response::IntoResponse::into_response(
                    hyper::StatusCode::FORBIDDEN,
                ));
            };
        }

        #[cfg(feature = "local")]
        let user = {
            let header = crate::X_USER;
            request
                .headers()
                .get(&header)
                .and_then(|h| h.to_str().ok())
                .unwrap_or(crate::consts::mock::USER_EMAIL)
        };

        #[cfg(not(feature = "local"))]
        let user = {
            let header = crate::X_USER;

            let Some(user_header) = request.headers().get(&header) else {
                forbid!(%header, "Header is missing");
            };

            match user_header.to_str() {
                Ok(user) => user,
                Err(error) => {
                    forbid!(%header, %error, "Header is not parseable as a String");
                }
            }
        };

        match self.provider.auth(user).await {
            Ok(Some(user)) => {
                request.extensions_mut().insert(user);
            }
            Ok(None) => {
                forbid!(%user, "User is not authorized");
            }
            Err(error) => {
                forbid!(%user, %error, "Could not query for user");
            }
        }

        inner.call(request).await
    }
}

mod tower {
    use super::{Auth, Provider};

    impl<I, P> tower_layer::Layer<I> for Auth<P>
    where
        P: Provider,
    {
        type Service = Middleware<P, I>;

        fn layer(&self, inner: I) -> Self::Service {
            Middleware {
                auth: self.clone(),
                inner,
                ready_inner: None,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Middleware<P, I>
    where
        P: Provider,
    {
        auth: Auth<P>,
        inner: I,
        ready_inner: Option<I>,
    }

    impl<B, P, I> tower_service::Service<hyper::Request<B>> for Middleware<P, I>
    where
        B: 'static + Send,
        P: Provider,
        I::Future: Send,
        I: 'static
            + Clone
            + Send
            + tower_service::Service<hyper::Request<B>, Response = axum::response::Response>,
    {
        type Response = I::Response;
        type Error = I::Error;
        type Future = std::pin::Pin<
            Box<dyn Send + std::future::Future<Output = Result<I::Response, I::Error>>>,
        >;

        fn poll_ready(
            &mut self,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Result<(), Self::Error>> {
            let inner = self.ready_inner.get_or_insert_with(|| self.inner.clone());
            inner.poll_ready(cx)
        }

        fn call(&mut self, request: hyper::Request<B>) -> Self::Future {
            let inner = self
                .ready_inner
                .take()
                .expect("Received a `call` in Auth without a `poll_ready`");

            Box::pin(self.auth.clone().auth(request, inner))
        }
    }
}
