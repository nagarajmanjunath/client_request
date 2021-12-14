pub mod client;
pub mod config;
pub mod error;
pub mod request;
use crate::request::HttpRequest;

pub trait ClientAPI: HttpRequest + Sync + Send + 'static {}
impl<T: HttpRequest + Sync + Send + 'static> ClientAPI for T {}
