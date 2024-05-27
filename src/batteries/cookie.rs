use iron;

pub use hyper::header::{CookiePair as Cookie, CookieJar};
use backend::{self, Request};
use server::header;
use ::{Extensible};

struct CookieJarKey;

impl ::typemap::Key for CookieJarKey {
    type Value = CookieJar<'static>;
}

pub trait CookieExt {
    fn find_cookie_jar(&mut self) -> Option<&mut CookieJar<'static>>;
    fn store_cookie_jar(&mut self, jar: CookieJar<'static>);
    fn cookies<'a>(&'a mut self) -> &'a mut CookieJar<'static> {
        self.find_cookie_jar().unwrap()
    }
}

impl<'r> CookieExt for (backend::Request + 'r) {
    fn find_cookie_jar<'a>(&'a mut self) -> Option<&'a mut CookieJar<'static>> {
        self.ext_mut().get_mut::<CookieJarKey>()
    }

    fn store_cookie_jar(&mut self, jar: CookieJar<'static>) {
        self.ext_mut().insert::<CookieJarKey>(jar);
    }
}

pub struct CookieDecodeMiddleware {
    secret_token: Vec<u8>
}

impl iron::BeforeMiddleware for CookieDecodeMiddleware {
    fn before(&self, req: &mut iron::Request) -> iron::IronResult<()> {
        let token = &self.secret_token;
        let jar = req.headers().get::<header::Cookie>()
            .map(|cookies| cookies.to_cookie_jar(token))
            .unwrap_or_else(|| CookieJar::new(token));

        req.ext_mut().insert::<CookieJarKey>(jar);
        Ok(())
    }
}

#[allow(missing_copy_implementations)]
pub struct CookieEncodeMiddleware;

impl iron::AfterMiddleware for CookieEncodeMiddleware {
    fn after(&self, req: &mut iron::Request, mut res: iron::Response) -> iron::IronResult<iron::Response> {
        let maybe_jar = (req as &mut backend::Request).find_cookie_jar();
        match maybe_jar {
            Some(jar) => {
                res.headers.set(header::SetCookie::from_cookie_jar(jar));
            },
            None => ()
        }

        Ok(res)
    }
}

pub fn new(secret_token: &[u8]) -> (CookieDecodeMiddleware, CookieEncodeMiddleware) {
    (
        CookieDecodeMiddleware{secret_token: secret_token.to_vec()},
        CookieEncodeMiddleware
    )
}