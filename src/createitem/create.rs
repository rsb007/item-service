use items::item::Item;
use dbconnection;

use cdrs::query::QueryExecutor;


use actix_web::{
    AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse,
};

use futures::{Future, Stream};
use json::JsonValue;
use serde_json;
use std;
use json;

fn insert(row: Item)
{
    let session = dbconnection::connection();

    let insert_struct_cql = "INSERT INTO auction.items \
                           (id, creator, title, description, currency_id, increment, reserve_price, price, status, auction_duration, auction_start, auction_end, auction_winner) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
    session
        .query_with_values(insert_struct_cql, query_values!(row.id, row.creator, row.title, row.description, row.currency_id, row.increment, row.reserve_price, row.price, row.status, row.auction_duration, row.auction_start, row.auction_end, row.auction_winner))
        .expect("insert error ");
}

pub fn insert_manual(
    req: &HttpRequest
) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.payload()
        .concat2()
        .from_err()
        .and_then(|body| {
            // body is loaded, now we can deserialize json-rust
            let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
            /*let injson: JsonValue = match result {
                Ok(v) => v,
                Err(e) => object! {"err" => e.to_string() } ,
            };*/
            let injson: JsonValue = result.unwrap();
            let item: Item = serde_json::from_str(&injson.to_string())?;
            insert(item);
            //println!("{}",emp.emp_name);
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(injson.dump()))
        })
        .responder()
}