use std::fmt;
use std::io;
use std::net;
use url;

use framework::media;

use server::method;
use server::header;
use super::super::errors;

pub trait Body: io::Read { }

impl Body for Box<io::Read + 'static> { }

pub trait AsUrl {
    fn scheme(&self) -> &str;
    fn host(&self) -> url::Host<&str>;
    fn port(&self) -> u16;
    fn path(&self) -> Vec<&str>;
    fn username(&self) -> Option<&str>;
    fn password(&self) -> Option<&str>;
    fn query(&self) -> Option<&str>;
    fn fragment(&self) -> Option<&str>;
}

pub trait Request: fmt::Debug + ::Extensible {
    fn remote_addr(&self) -> &net::SocketAddr;
    fn headers(&self) -> &header::Headers;
    fn method(&self) -> &method::Method;
    fn url(&self) -> &AsUrl;
    fn body(&self) -> &Body;
    fn body_mut(&mut self) -> &mut Body;

    fn read_to_end(&mut self) -> Result<Option<String>, Box<errors::Error + Send>>;

    fn is_json_body(&self) -> bool {
        self.headers().get::<header::ContentType>().map_or(false, |ct| media::is_json(&ct.0))
    }

    fn is_urlencoded_body(&self) -> bool {
        self.headers().get::<header::ContentType>().map_or(false, |ct| media::is_urlencoded(&ct.0))
    }

    fn is_form_data_body(&self) -> bool {
        self.headers().get::<header::ContentType>().map_or(false, |ct| media::is_form_data(&ct.0))
    }
}
