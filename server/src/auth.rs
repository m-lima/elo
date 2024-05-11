const X_USER: hyper::header::HeaderName = hyper::header::HeaderName::from_static("x-user");

#[derive(Debug, Clone)]
pub struct Auth {
    store: store::Store,
}

impl Auth {
    async fn auth<B>(&self, request: &hyper::Request<B>) -> Option<types::Id> {
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

        match self.store.users().id_for(user).await {
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
}

impl<I> tower_layer::Layer<I> for Auth {
    type Service = Middleware<I>;

    fn layer(&self, inner: I) -> Self::Service {
        Middleware {
            auth: self.clone(),
            inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Middleware<I> {
    auth: Auth,
    inner: I,
}

impl<B, I> tower_service::Service<hyper::Request<B>> for Middleware<I>
where
    B: 'static,
    I: tower_service::Service<hyper::Request<B>, Response = axum::response::Response>,
    I::Future: Unpin,
{
    type Response = I::Response;
    type Error = I::Error;
    // type Future = Future<I::Future, I::Error>;
    type Future =
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<I::Response, I::Error>>>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: hyper::Request<B>) -> Self::Future {
        let auth = self.auth.clone();
        let future = async move {
            let Some(user) = auth.auth(&request).await else {
                return Ok(axum::response::IntoResponse::into_response(
                    hyper::StatusCode::FORBIDDEN,
                ));
            };

            request.extensions_mut().insert(user);
            self.inner.call(request).await
        };

        Box::pin(future)

        // let inner = self.inner.call(request);
        // let Some(user) = pre_auth(&request) else {
        //     let users = Box::pin(self.store.users().get(""));
        //     return Future {
        //         state: State::Forbidden,
        //         users,
        //         inner,
        //     };
        // };
        //
        // let users = Box::pin(self.store.users().get(user.as_str()));
        // return Future {
        //     state: State::Forbidden,
        //     users,
        //     inner,
        // };
    }
}

fn pre_auth<B>(request: &hyper::Request<B>) -> Option<String> {
    let header = X_USER;

    let Some(user_header) = request.headers().get(&header) else {
        tracing::warn!(%header, "Header is missing");
        return None;
    };

    match user_header.to_str() {
        Ok(user) => Some(String::from(user)),
        Err(error) => {
            tracing::warn!(%header, %error, "Header is not parseable as a String");
            return None;
        }
    }

    // match store.users().get(user).await {
    //     Ok(user) => Some(user),
    //     Err(store::Error::NotFound) => {
    //         tracing::warn!(%header, %user, "User is not authorized");
    //         None
    //     }
    //     Err(error) => {
    //         tracing::warn!(%header, %user, %error, "Could not query for user");
    //         None
    //     }
    // }
}

pub struct Future<I, E>
where
    I: std::future::Future<Output = Result<axum::response::Response, E>> + Unpin,
{
    state: State,
    users: std::pin::Pin<Box<dyn std::future::Future<Output = Result<types::User, store::Error>>>>,
    inner: I,
}

enum State {
    PreAuth,
    Pass,
    Forbidden,
}

impl<I, E> std::future::Future for Future<I, E>
where
    I: std::future::Future<Output = Result<axum::response::Response, E>> + Unpin,
{
    type Output = I::Output;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match self.state {
            State::PreAuth => {
                let this = self.get_mut();
                let users = std::pin::Pin::new(&mut this.users);
                cx.waker().wake_by_ref();
                match std::task::ready!(users.poll(cx)) {
                    Ok(_) => {
                        this.state = State::Pass;
                        std::task::Poll::Pending
                    }
                    Err(store::Error::NotFound) => {
                        this.state = State::Forbidden;
                        std::task::Poll::Pending
                    }
                    Err(error) => todo!(),
                }
            }
            State::Pass => {
                let this = self.get_mut();
                let inner = std::pin::Pin::new(&mut this.inner);
                inner.poll(cx)
            }
            State::Forbidden => {
                let response =
                    axum::response::IntoResponse::into_response(hyper::StatusCode::FORBIDDEN);
                std::task::Poll::Ready(Ok(response))
            }
        }
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
