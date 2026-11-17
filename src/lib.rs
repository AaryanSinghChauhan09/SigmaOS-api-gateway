//! Simple API gateway for SigmaOS.
//!
//! Route-based request routing, middleware chain, and request/response
//! abstractions. Dependency-free.

use std::collections::HashMap;
use std::fmt;

/// HTTP-like request representation.
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new(method: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            method: method.into(),
            path: path.into(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }
}

/// HTTP-like response representation.
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status: u16) -> Self {
        Self { status, headers: HashMap::new(), body: Vec::new() }
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    pub fn ok() -> Self {
        Self::new(200)
    }

    pub fn not_found() -> Self {
        Self::new(404)
    }

    pub fn internal_error() -> Self {
        Self::new(500)
    }
}

/// Errors for gateway operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GatewayError {
    NotFound(String),
    InvalidPath(String),
    MiddlewareError(String),
}

impl fmt::Display for GatewayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(p) => write!(f, "route not found: {p}"),
            Self::InvalidPath(p) => write!(f, "invalid path: {p}"),
            Self::MiddlewareError(e) => write!(f, "middleware error: {e}"),
        }
    }
}

impl std::error::Error for GatewayError {}

type Handler = fn(&Request) -> Result<Response, GatewayError>;
type Middleware = fn(&mut Request) -> Result<(), GatewayError>;

/// A route entry.
#[derive(Debug, Clone)]
struct Route {
    pattern: String,
    handler: Handler,
}

/// Simple API gateway with routing and middleware.
#[derive(Debug, Clone)]
pub struct ApiGateway {
    routes: Vec<Route>,
    middleware: Vec<Middleware>,
}

impl ApiGateway {
    pub fn new() -> Self {
        Self { routes: Vec::new(), middleware: Vec::new() }
    }

    pub fn add_middleware(&mut self, mw: Middleware) {
        self.middleware.push(mw);
    }

    pub fn add_route(&mut self, pattern: impl Into<String>, handler: Handler) {
        self.routes.push(Route { pattern: pattern.into(), handler });
    }

    fn match_route(&self, path: &str) -> Option<&Route> {
        self.routes.iter().find(|r| path_match(&r.pattern, path))
    }

    pub fn handle(&mut self, mut req: Request) -> Result<Response, GatewayError> {
        for mw in &self.middleware {
            mw(&mut req)?;
        }
        let route = self.match_route(&req.path).ok_or_else(|| GatewayError::NotFound(req.path.clone()))?;
        (route.handler)(&req)
    }
}

fn path_match(pattern: &str, path: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if pattern.ends_with('*') {
        let prefix = &pattern[..pattern.len() - 1];
        return path.starts_with(prefix);
    }
    pattern == path
}

#[cfg(test)]
mod tests {
    use super::*;

    fn static_handler(req: &Request) -> Result<Response, GatewayError> {
        if req.path == "/ok" {
            Ok(Response::ok().with_body(b"OK".to_vec()))
        } else {
            Ok(Response::not_found())
        }
    }

    fn uppercase_middleware(req: &mut Request) -> Result<(), GatewayError> {
        req.method = req.method.to_uppercase();
        Ok(())
    }

    #[test]
    fn test_route_matching_aaa() {
        // Arrange
        let mut gw = ApiGateway::new();
        gw.add_route("/ok", static_handler);
        gw.add_route("/api/*", static_handler);

        // Act
        let _req1 = Request::new("GET", "/ok");
        let _req2 = Request::new("GET", "/api/users");
        let _req3 = Request::new("GET", "/other");

        // Assert
        assert!(gw.match_route("/ok").is_some());
        assert!(gw.match_route("/api/users").is_some());
        assert!(gw.match_route("/other").is_none());
    }

    #[test]
    fn test_middleware_chain_aaa() {
        // Arrange
        let mut gw = ApiGateway::new();
        gw.add_middleware(uppercase_middleware);
        gw.add_route("/ok", static_handler);
        let req = Request::new("get", "/ok");

        // Act
        let res = gw.handle(req);

        // Assert
        assert!(res.is_ok());
    }

    #[test]
    fn test_not_found_error_aaa() {
        // Arrange
        let mut gw = ApiGateway::new();
        gw.add_route("/ok", static_handler);
        let req = Request::new("GET", "/missing");

        // Act
        let res = gw.handle(req);

        // Assert
        assert!(matches!(res, Err(GatewayError::NotFound(_))));
    }
}
