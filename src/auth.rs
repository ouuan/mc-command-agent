use crate::config::Config;
use actix_web::dev::ServiceRequest;
use actix_web::web::Data;
use actix_web_httpauth::extractors::{
    basic::{self, BasicAuth},
    AuthenticationError,
};
use std::cell::RefCell;
use std::sync::Arc;

pub type CommandsData = Data<RefCell<Vec<String>>>;

pub async fn auth(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> std::result::Result<ServiceRequest, (actix_web::error::Error, ServiceRequest)> {
    if let Some(password) = credentials.password() {
        let config = req.app_data::<Data<Arc<Config>>>().unwrap();
        if let Some(user) = config.auth(credentials.user_id(), password) {
            req.app_data::<CommandsData>()
                .unwrap()
                .replace(config.commands_for_user(user));
            return Ok(req);
        }
    }
    let auth_config = req.app_data::<basic::Config>().cloned().unwrap_or_default();
    Err((AuthenticationError::from(auth_config).into(), req))
}
