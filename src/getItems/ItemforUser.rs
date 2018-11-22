use actix_web::{
    HttpRequest, Responder
};
use cdrs::query::*;
use cdrs::types::rows::Row;
use dbconnection;
use items::item::Item;
use serde_json;

use cdrs::{self,types::prelude::*};
use cdrs::frame::TryFromRow;

fn view(status: String,creator: String) -> Vec<Row>
{
    let session = dbconnection::connection();
    println!("{} {}",status,creator);
    let select_struct_cql = "Select * from auction.items where status = ? and creator = ? [ALLOW FILTERING]";
    let row = session
        .query_with_values(select_struct_cql, query_values!(status,creator))
        .expect("query")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into row");

    return row;
}

pub fn get_items(req: &HttpRequest) -> impl Responder
{
    let rows = view(req.query().get("status").unwrap().parse().unwrap(),req.query().get("creator").unwrap().parse().unwrap());
//println!("{}",rows.len());
    let mut my_row: Item = Item {
        id: String::new(),
        creator: String::new(),
        title: String::new(),
        description: String::new(),
        currency_id: String::new(),
        increment: 0.0,
        reserve_price: 0.0,
        price: 0.0,
        status: String::new(),
        auction_duration: String::new(),
        auction_winner: String::new(),
        auction_end: String::new(),
        auction_start: String::new(),
    };



    for row in rows {
        my_row = Item::try_from_row(row).expect("into Item Struct");
        println!("Struct got :{:?}", my_row);
    }

    let jsonstring = serde_json::to_string(&my_row);

    return jsonstring;
}
