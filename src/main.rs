extern crate actix;
extern crate actix_web;
extern crate bytes;
#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
extern crate env_logger;
extern crate futures;
extern crate json;
extern crate listenfd;
extern crate maplit;
extern crate openssl;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use actix_web::{
    App, http, HttpRequest,
    middleware, Responder, server,
};
use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::frame::IntoBytes;
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use cdrs::types::rows::Row;
use listenfd::ListenFd;
use std::str;

extern crate onlineauction;


fn main() {
    ::std::env::set_var("RUST_Log", "actix_web=info");
    env_logger::init();
        let con=onlineauction::dbconnection::connection();

   /* onlineauction::create_db::create_keyspace(&con);
    println!("keyspace created");
    onlineauction::create_db::create_table(&con);
    println!("table created");*/

    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            //enable logger
            .middleware(middleware::Logger::default())
            .resource("/insert", |r| r.method(http::Method::POST).f(onlineauction::createitem::create::insert_manual))
            .resource("/getItem", |r| r.method(http::Method::GET).f(onlineauction::getItems::ItembyId::get_manual))
            .resource("/getItembyUser", |r| r.method(http::Method::GET).f(onlineauction::getItems::ItemforUser::get_items))
            .resource("/startauction", |r| r.method(http::Method::GET).f(onlineauction::startauction::start_auction::start_auction))
        //.resource("/",|r|  r.f(|r| HttpResponse::Ok()))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:8000").unwrap()
    };

    server.run();
}
