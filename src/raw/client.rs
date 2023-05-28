//! This module houses the foundation of the SDK's HTTP handling for outgoing requests to 1Password.

use std::time::Duration;

use ureq::{
    Agent, AgentBuilder, Error, Middleware, MiddlewareNext, Request, Response,
};

/// `HeaderTokenMiddleware` injects the `Authorization` HTTP header into all outgoing requests,
/// using the API token passed during `Agent` initialisation.
struct HeaderTokenMiddleware(String);

/// `APIContentTypeHeaderMiddleware` injects the API-required `Content-Type` header into all
/// outgoing requests
struct APIContentTypeHeaderMiddleware;

impl Middleware for HeaderTokenMiddleware {
    fn handle(
        &self,
        request: Request,
        next: MiddlewareNext,
    ) -> Result<Response, Error> {
        next.handle(request.set(
            "Authorization",
            format!("Bearer {}", self.0.as_str()).as_str(),
        ))
    }
}

impl Middleware for APIContentTypeHeaderMiddleware {
    fn handle(
        &self,
        request: Request,
        next: MiddlewareNext,
    ) -> Result<Response, Error> {
        // set default `Content-Type` header for Connect API.
        next.handle(request.set("Content-Type", "application/json"))
    }
}

/// `Client` acts as a holder of `ureq::Agent` for making calls to 1Password Connect.
#[derive(Debug, Clone)]
pub struct Client {
    /// This field is for the `ureq::Agent` struct once initialized by the `Client::new`
    /// function. It is not designed for manual initialization.
    #[allow(dead_code)]
    http: Agent,
}

impl Client {
    /// This method of the `Client` implementation initialises a new `ureq::Agent` for the SDK
    /// HTTP handling module.
    #[must_use]
    #[allow(dead_code)]
    pub fn new(
        token: &str,
        user_agent: &str,
        timeout: Option<Duration>,
    ) -> Self {
        Self {
            http: AgentBuilder::new()
                .middleware(HeaderTokenMiddleware(String::from(token)))
                .middleware(APIContentTypeHeaderMiddleware)
                .timeout(timeout.unwrap_or(Duration::from_secs(10))) // default to 10 seconds
                .user_agent(user_agent)
                .build(),
        }
    }
}
