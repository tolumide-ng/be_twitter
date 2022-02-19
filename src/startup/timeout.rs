use std::error::Error;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use futures::Future;
use tower::{Service, Layer};

pub struct Timeout<T> {
    pub inner: T,
    timeout: Duration
}

// Our timeout service, which wraps another service and 
// adds a timeout to its response future.
impl<T> Timeout<T> {
    pub fn new(inner: T, timeout: Duration) -> Timeout<T> {
        Self {inner, timeout}
    }
}


// the eror returned if processing a request timed out
#[derive(Debug, derive_more::Display)]
#[display(fmt = "expired")]
pub struct Expired;

impl Error for Expired {}

// we can implement Service for Timeout<T> if T is a Service
impl<T, Request> Service<Request> for Timeout<T>
where
    T: Service<Request>,
    T::Future: 'static,
    T::Error: Into<Box<dyn Error + Send + Sync>> + 'static,
    T::Response: 'static,
{
    // timeout doesn't modify the response type, so we use `T`'s response type
    type Response = T::Response;
    type Error = Box<dyn Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let timeout = tokio::time::sleep(self.timeout);
        let fut = self.inner.call(req);

        let f = async move {
            tokio::select! {
                res = fut => {
                    res.map_err(|err| err.into())
                },
                _ = timeout => {
                    Err(Box::new(Expired) as Box<dyn Error + Send + Sync>)
                },
            }
        };

        Box::pin(f)
    }
}

// A layer of wrapping services in `Timeout`
pub struct TimeoutLayer(Duration);

impl TimeoutLayer {
    pub fn new(delay: Duration) -> Self {
        TimeoutLayer(delay)
    }
}

impl<S> Layer<S> for TimeoutLayer {
    type Service = Timeout<S>;

    fn layer(&self, service: S) -> Timeout<S> {
        Timeout::new(service, self.0)
    }
}