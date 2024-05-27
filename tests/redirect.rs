use rustless::server::header;
use rustless::server::status;
use rustless::{Nesting};

#[test]
fn it_allows_redirect() {

    let app = app!(|api| {
        api.prefix("api");
        api.post("redirect_me/:href", |endpoint| {
            endpoint.handle(|client, params| {
                client.redirect(params.find(&"href".to_string()).unwrap().as_str().unwrap())
            })
        });
    });

    let response = call_app!(app, Post, "http://127.0.0.1:3000/api/redirect_me/google.com").ok().unwrap();
    assert_eq!(response.status, status::StatusCode::Found);
    let &header::Location(ref location) = response.headers.get::<header::Location>().unwrap();
    assert_eq!(location, "google.com")

}

#[test]
fn it_allows_permanent_redirect() {

    let app = app!(|api| {
        api.prefix("api");
        api.post("redirect_me/:href", |endpoint| {
            endpoint.handle(|client, params| {
                client.permanent_redirect(params.find(&"href".to_string()).unwrap().as_str().unwrap())
            })
        });
    });

    let response = call_app!(app, Post, "http://127.0.0.1:3000/api/redirect_me/google.com").ok().unwrap();
    assert_eq!(response.status, status::StatusCode::MovedPermanently);
    let &header::Location(ref location) = response.headers.get::<header::Location>().unwrap();
    assert_eq!(location, "google.com")

}