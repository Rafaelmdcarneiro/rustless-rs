use rustless::server::status;
use rustless::{Nesting};

#[test]
fn it_allows_to_create_namespace() {

    let app = app!(|api| {
        api.prefix("api");

        api.namespace("ns1", |ns| edp_stub!(ns));
        api.group("ns2", |ns| edp_stub!(ns));
        api.resource("ns3", |ns| edp_stub!(ns));
        api.resources("ns4", |ns| edp_stub!(ns));
        api.segment("ns5", |ns| edp_stub!(ns));

    });

    let response = call_app!(app, Get, "http://127.0.0.1:3000/api/ns1/info").ok().unwrap();
    assert_eq!(response.status, status::StatusCode::Ok);

    let response = call_app!(app, Get, "http://127.0.0.1:3000/api/ns2/info").ok().unwrap();
    assert_eq!(response.status, status::StatusCode::Ok);

    let response = call_app!(app, Get, "http://127.0.0.1:3000/api/ns3/info").ok().unwrap();
    assert_eq!(response.status, status::StatusCode::Ok);

    let response = call_app!(app, Get, "http://127.0.0.1:3000/api/ns4/info").ok().unwrap();
    assert_eq!(response.status, status::StatusCode::Ok);

    let response = call_app!(app, Get, "http://127.0.0.1:3000/api/ns5/info").ok().unwrap();
    assert_eq!(response.status, status::StatusCode::Ok);

}

#[test]
fn it_allows_nested_namespaces() {

    let app = app!(|api| {
        api.prefix("api");

        api.namespace("ns1", |ns1| {
            ns1.group("ns2", |ns2| {
                ns2.resource("ns3", |ns3| {
                    ns3.resources("ns4", |ns4| {
                        ns4.segment("ns5", |ns5| edp_stub!(ns5));
                    })
                })
            })
        })

    });

    let response = call_app!(app, Get, "http://127.0.0.1:3000/api/ns1/ns2/ns3/ns4/ns5/info").ok().unwrap();
    assert_eq!(response.status, status::StatusCode::Ok);

}

#[test]
fn it_allows_grouping_with_zero_path() {

    let app = app!(|api| {
        api.prefix("api");

        api.namespace("ns1", |ns1| {
            ns1.get("", |edp| {
                edp.handle(|client, _params| {
                    client.text("Some usefull info".to_string())
                })
            });
            ns1.post("", |edp| {
                edp.handle(|client, _params| {
                    client.text("Some usefull info".to_string())
                })
            })
        })

    });

    let response = call_app!(app, Get, "http://127.0.0.1:3000/api/ns1").ok().unwrap();
    assert_eq!(response.status, status::StatusCode::Ok);

    let response = call_app!(app, Post, "http://127.0.0.1:3000/api/ns1").ok().unwrap();
    assert_eq!(response.status, status::StatusCode::Ok);

    let err_resp = call_app!(app, Delete, "http://127.0.0.1:3000/api/ns1").err().unwrap();
    assert_eq!(err_resp.response.status, status::StatusCode::NotFound);

}