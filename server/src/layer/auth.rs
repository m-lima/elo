#[derive(Debug, Clone)]
pub struct Auth {
    store: store::Store,
}

impl Auth {
    pub fn new(store: store::Store) -> Self {
        Self { store }
    }

    #[tracing::instrument(target = "auth", skip_all)]
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
                tracing::warn!(target: "auth", $($arg)*);
                return Ok(axum::response::IntoResponse::into_response(
                    hyper::StatusCode::FORBIDDEN,
                ));
            };
        }

        let header = crate::X_USER;

        let Some(user_header) = request.headers().get(&header) else {
            forbid!(%header, "Header is missing");
        };

        let user = match user_header.to_str() {
            Ok(user) => user,
            Err(error) => {
                forbid!(%header, %error, "Header is not parseable as a String");
            }
        };

        match self.store.users().by_email(user).await {
            Ok(Some(user)) => {
                request.extensions_mut().insert(user);
            }
            Ok(None) => {
                forbid!(%user, "User is not authorized");
            }
            Err(error) => {
                forbid!(%user, %error, "Could not query for user");
            }
        };

        inner.call(request).await
    }
}

impl<I> tower_layer::Layer<I> for Auth {
    type Service = Middleware<I>;

    fn layer(&self, inner: I) -> Self::Service {
        Middleware {
            auth: self.clone(),
            inner,
            ready_inner: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Middleware<I> {
    auth: Auth,
    inner: I,
    ready_inner: Option<I>,
}

impl<B, I> tower_service::Service<hyper::Request<B>> for Middleware<I>
where
    B: 'static + Send,
    I::Future: Send,
    I: 'static
        + Clone
        + Send
        + tower_service::Service<hyper::Request<B>, Response = axum::response::Response>,
{
    type Response = I::Response;
    type Error = I::Error;
    type Future =
        std::pin::Pin<Box<dyn Send + std::future::Future<Output = Result<I::Response, I::Error>>>>;

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
