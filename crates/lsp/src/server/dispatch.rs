use anyhow::Result;
use lsp_server::{ErrorCode, Notification, Request, RequestId, Response};
use serde::de::DeserializeOwned;

pub struct NotificationDispatcher {
    not: Option<Notification>,
}

impl NotificationDispatcher {
    pub fn new(not: Notification) -> Self {
        Self { not: Some(not) }
    }

    pub fn on<N, F>(mut self, handler: F) -> Result<Self>
    where
        N: lsp_types::notification::Notification,
        N::Params: DeserializeOwned,
        F: FnOnce(N::Params) -> Result<()>,
    {
        if let Some(not) = self.not {
            match not.extract::<N::Params>(N::METHOD) {
                Ok(params) => {
                    handler(params)?;
                    self.not = None;
                }
                Err(lsp_server::ExtractError::MethodMismatch(not)) => {
                    self.not = Some(not);
                }
                Err(lsp_server::ExtractError::JsonError { .. }) => {
                    self.not = None;
                }
            };
        }
        Ok(self)
    }

    pub fn default(self) {
        if let Some(not) = &self.not {
            // log::warn!("Unknown notification: {}", not.method);
        }
    }
}

pub struct RequestDispatcher {
    req: Option<Request>,
}

impl RequestDispatcher {
    pub fn new(req: Request) -> Self {
        Self { req: Some(req) }
    }

    pub fn on<R, F>(mut self, handler: F) -> Result<Self>
    where
        R: lsp_types::request::Request,
        R::Params: DeserializeOwned,
        F: FnOnce(RequestId, R::Params) -> Result<()>,
    {
        if let Some(req) = self.req {
            match req.extract::<R::Params>(R::METHOD) {
                Ok((id, params)) => {
                    handler(id, params)?;
                    self.req = None;
                }
                Err(lsp_server::ExtractError::MethodMismatch(req)) => {
                    self.req = Some(req);
                }
                Err(lsp_server::ExtractError::JsonError { .. }) => {
                    self.req = None;
                }
            }
        }
        Ok(self)
    }

    pub fn default(self) -> Option<Response> {
        self.req.map(|req| {
            // log::warn!("Unknown request: {}", req.method);
            Response::new_err(
                req.id,
                ErrorCode::MethodNotFound as i32,
                "method not found".to_string(),
            )
        })
    }
}
