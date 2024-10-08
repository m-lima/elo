#[derive(Debug, Copy, Clone)]
pub struct Logger;

impl<S> tower_layer::Layer<S> for Logger {
    type Service = Middleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Middleware { inner }
    }
}

#[derive(Debug, Clone)]
pub struct Middleware<S> {
    inner: S,
}

impl<B, S> tower_service::Service<hyper::Request<B>> for Middleware<S>
where
    S: tower_service::Service<hyper::Request<B>, Response = axum::response::Response>,
    S::Error: std::fmt::Display,
    S::Future: Unpin,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Future<S::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: hyper::Request<B>) -> Self::Future {
        let start = std::time::Instant::now();
        let method = String::from(request.method().as_str());
        let path = String::from(request.uri().path());
        let length = get_length(request.headers());
        let user = request
            .headers()
            .get(crate::X_USER)
            .and_then(|l| l.to_str().ok())
            .map(String::from);

        let span = tracing::span!(
            tracing::Level::INFO,
            "request",
            %method,
            %path
        );

        Future {
            span,
            start,
            method,
            path,
            user,
            length,
            inner: self.inner.call(request),
        }
    }
}

pub struct Future<F> {
    span: tracing::Span,
    start: std::time::Instant,
    method: String,
    path: String,
    user: Option<String>,
    length: Option<usize>,
    inner: F,
}

impl<F, E> std::future::Future for Future<F>
where
    F: std::future::Future<Output = Result<axum::response::Response, E>> + Unpin,
    E: std::fmt::Display,
{
    type Output = F::Output;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();
        let _span = this.span.enter();
        let future = &mut this.inner;

        let output = std::task::ready!(std::pin::Pin::new(future).poll(cx));

        match output {
            Ok(response) => {
                log_ok(this, &response);
                std::task::Poll::Ready(Ok(response))
            }
            Err(error) => {
                log_err(this, &error);
                std::task::Poll::Ready(Err(error))
            }
        }
    }
}

fn log_ok<F>(future: &Future<F>, response: &axum::response::Response) {
    let latency = future.start.elapsed();
    let length = get_length(response.headers());
    let (status, spacer, reason) = get_status_tuple(response.status());

    macro_rules! log {
        ($level: expr) => {
            match (future.user.as_ref(), future.length, length) {
                (Some(user), Some(incoming), Some(outgoing)) => {
                    log!(
                        $level,
                        %user,
                        incoming,
                        outgoing,
                    );
                }
                (Some(user), None, Some(outgoing)) => {
                    log!(
                        $level,
                        %user,
                        outgoing,
                    );
                }
                (None, Some(incoming), Some(outgoing)) => {
                    log!(
                        $level,
                        incoming,
                        outgoing,
                    );
                }
                (None, None, Some(outgoing)) => {
                    log!(
                        $level,
                        outgoing,
                    );
                }
                (Some(user), Some(incoming), None) => {
                    log!(
                        $level,
                        %user,
                        incoming,
                    );
                }
                (Some(user), None, None) => {
                    log!(
                        $level,
                        %user,
                    );
                }
                (None, Some(incoming), None) => {
                    log!(
                        $level,
                        incoming,
                    );
                }
                (None, None, None) => {
                    log!(
                        $level,
                    );
                }
            }
        };
        ($level: expr, $($args: tt)*) => {
            tracing::event!(
                $level,
                method = %future.method,
                path = %future.path,
                $($args)*
                ?latency,
                "{status}{spacer}{reason}"
            );
        }
    }

    match status {
        0..=399 => log!(tracing::Level::DEBUG),
        400..=499 => log!(tracing::Level::WARN),
        500.. => log!(tracing::Level::ERROR),
    }
}

fn log_err<F, E: std::fmt::Display>(future: &Future<F>, error: &E) {
    match (future.user.as_ref(), future.length) {
        (Some(user), Some(incoming)) => {
            tracing::error!(method = future.method, path = future.path, user, incoming, %error, "Unexpected error while serving request");
        }
        (Some(user), None) => {
            tracing::error!(method = future.method, path = future.path, user, %error, "Unexpected error while serving request");
        }
        (None, Some(incoming)) => {
            tracing::error!(method = future.method, path = future.path, incoming, %error, "Unexpected error while serving request");
        }
        (None, None) => {
            tracing::error!(method = future.method, path = future.path, %error, "Unexpected error while serving request");
        }
    }
}

fn get_status_tuple(status: hyper::StatusCode) -> (u16, &'static str, &'static str) {
    match status.canonical_reason() {
        Some(reason) => (status.as_u16(), " ", reason),
        None => (status.as_u16(), "", ""),
    }
}

fn get_length(headers: &hyper::header::HeaderMap) -> Option<usize> {
    headers
        .get(hyper::header::CONTENT_LENGTH)
        .and_then(|l| l.to_str().ok())
        .and_then(|l| l.parse().ok())
        .filter(|l| *l > 0)
}
