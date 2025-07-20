use crate::{http::{Request, Response}, Error, Result};
use http::Method;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;

pub type Handler = Arc<dyn Fn(Request) -> Result<Response> + Send + Sync>;

#[derive(Clone)]
pub struct Route {
    pub method: Method,
    pub pattern: String,
    pub regex: String,
    pub param_names: Vec<String>,
    pub handler: Handler,
}

impl std::fmt::Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Route")
            .field("method", &self.method)
            .field("pattern", &self.pattern)
            .field("regex", &self.regex)
            .field("param_names", &self.param_names)
            .field("handler", &"<function>")
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn get<F>(&mut self, pattern: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Result<Response> + Send + Sync + 'static,
    {
        self.add_route(Method::GET, pattern, handler);
        self
    }

    pub fn post<F>(&mut self, pattern: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Result<Response> + Send + Sync + 'static,
    {
        self.add_route(Method::POST, pattern, handler);
        self
    }

    pub fn put<F>(&mut self, pattern: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Result<Response> + Send + Sync + 'static,
    {
        self.add_route(Method::PUT, pattern, handler);
        self
    }

    pub fn delete<F>(&mut self, pattern: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Result<Response> + Send + Sync + 'static,
    {
        self.add_route(Method::DELETE, pattern, handler);
        self
    }

    pub fn options<F>(&mut self, pattern: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Result<Response> + Send + Sync + 'static,
    {
        self.add_route(Method::OPTIONS, pattern, handler);
        self
    }

    pub fn add_route<F>(&mut self, method: Method, pattern: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Result<Response> + Send + Sync + 'static,
    {
        let (regex_pattern, param_names) = Self::compile_pattern(pattern);
        let route = Route {
            method,
            pattern: pattern.to_string(),
            regex: regex_pattern,
            param_names,
            handler: Arc::new(handler),
        };
        self.routes.push(route);
        self
    }

    pub fn handle(&self, request: Request) -> Result<Response> {
        for route in &self.routes {
            if route.method == request.method {
                if let Some(params) = self.match_route(route, request.path()) {
                    let mut request_with_params = request;
                    request_with_params.params = params;
                    return (route.handler)(request_with_params);
                }
            }
        }
        Err(Error::RouteNotFound(request.path().to_string()))
    }

    fn compile_pattern(pattern: &str) -> (String, Vec<String>) {
        let mut param_names = Vec::new();
        let mut regex_pattern = String::new();
        let mut in_param = false;
        let mut param_name = String::new();

        for ch in pattern.chars() {
            match ch {
                '{' => {
                    in_param = true;
                    param_name.clear();
                }
                '}' => {
                    if in_param {
                        param_names.push(param_name.clone());
                        regex_pattern.push_str("([^/]+)");
                        in_param = false;
                    }
                }
                _ => {
                    if in_param {
                        param_name.push(ch);
                    } else {
                        regex_pattern.push(ch);
                    }
                }
            }
        }

        (format!("^{}$", regex_pattern), param_names)
    }

    fn match_route(&self, route: &Route, path: &str) -> Option<HashMap<String, String>> {
        if let Ok(regex) = Regex::new(&route.regex) {
            if let Some(captures) = regex.captures(path) {
                let mut params = HashMap::new();
                for (i, param_name) in route.param_names.iter().enumerate() {
                    if let Some(capture) = captures.get(i + 1) {
                        params.insert(param_name.clone(), capture.as_str().to_string());
                    }
                }
                Some(params)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
} 