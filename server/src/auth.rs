const X_USER: hyper::header::HeaderName = hyper::header::HeaderName::from_static("x-user");

#[derive(Debug, Clone)]
pub struct Auth {
    store: store::Store,
}

impl<I> tower_layer::Layer<I> for Auth {
    type Service = Middleware<I>;

    fn layer(&self, inner: I) -> Self::Service {
        Middleware {
            store: self.store.clone(),
            inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Middleware<I> {
    store: store::Store,
    inner: I,
}

impl<B, I> tower_service::Service<hyper::Request<B>> for Middleware<I>
where
    B: 'static,
    I: tower_service::Service<hyper::Request<B>, Response = axum::response::Response>,
{
    type Response = I::Response;
    type Error = I::Error;
    type Future =
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: hyper::Request<B>) -> Self::Future {
        let future = async {
            let Some(user) = auth(&self.store, &request).await else {
                return Ok(axum::response::IntoResponse::into_response(
                    hyper::StatusCode::FORBIDDEN,
                ));
            };

            self.inner.call(request).await
        };

        Box::pin(future)
    }
}

async fn auth<B>(
    store: &store::Store,
    request: &hyper::Request<B>,
) -> Option<types::User<store::Id>> {
    let header = X_USER;

    let Some(user_header) = request.headers().get(&header) else {
        tracing::warn!(%header, "Header is missing");
        return None;
    };

    let user = match user_header.to_str() {
        Ok(user) => user,
        Err(error) => {
            tracing::warn!(%header, %error, "Header is not parseable as a String");
            return None;
        }
    };

    match store.users().get(user).await {
        Ok(user) => Some(user),
        Err(store::Error::NotFound) => {
            tracing::warn!(%header, %user, "User is not authorized");
            None
        }
        Err(error) => {
            tracing::warn!(%header, %user, %error, "Could not query for user");
            None
        }
    }
}

pub enum Future<B, I> {
    PreAuth {
        request: hyper::Request<B>,
        store: store::Store,
        inner: I,
    },
    Forbidden,
    Pass,
}

impl std::future::Future for Future {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

// #[tracing::instrument(target = "layer", skip_all)]
// fn pre_auth<'a, B>(request: &'a hyper::Request<B>, service: &S) -> Option<(&'a str, S)> {
//     let header = X_USER;
//
//     let Some(user_header) = request.headers().get(&header) else {
//         tracing::warn!(%header, "Header is missing");
//         return None;
//     };
//
//     let user = match user_header.to_str() {
//         Ok(user) => user,
//         Err(error) => {
//             tracing::warn!(%header, %error, "Header is not parseable as a String");
//             return None;
//         }
//     };
//
//     services.get(user).map(|s| (user, s.clone()))
// }
//
// pub enum Future<F> {
//     Forbidden,
//     Pass(F, tracing::Span),
// }
//
// impl<F, E> std::future::Future for Future<F>
// where
//     F: std::future::Future<Output = Result<axum::response::Response, E>> + Unpin,
//     E: std::fmt::Display,
// {
//     type Output = F::Output;
//
//     fn poll(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Self::Output> {
//         match self.get_mut() {
//             Self::Forbidden => {
//                 let response =
//                     axum::response::IntoResponse::into_response(hyper::StatusCode::FORBIDDEN);
//                 std::task::Poll::Ready(Ok(response))
//             }
//             Self::Pass(f, span) => {
//                 let _span = span.enter();
//                 std::pin::Pin::new(f).poll(cx)
//             }
//         }
//     }
// }
