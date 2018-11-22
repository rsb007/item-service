use actix_web::{
    HttpRequest, Responder,
};
use cdrs::query::*;
use cdrs::types::rows::Row;
use dbconnection;
use items::item::Item;
use serde_json;

use cdrs::{self, types::prelude::*};
use cdrs::frame::TryFromRow;
use serde_json::to_string;


fn get_status(id: String) -> Vec<Row> {
    let session = dbconnection::connection();
    println!("{}", id);
    let select_struct_cql = "Select * from auction.items where id = ?";
    let row = session
        .query_with_values(select_struct_cql, query_values!(id))
        .expect("query")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into row");

    return row;
}

fn create() {
    println!("your auction is created");
}

fn bidding() {
    println!("bidding is started on your auction");
}

fn check_status(status: String) {

/*
    match status {
        "created".to_string() => create(),
        "completed".to_string() => println!("auction is completed"),
        "auction".to_string() => bidding(),
        "cancelled".to_string() => println!("your auction has been cancelled"),
    }*/
}


pub fn start_auction(req: &HttpRequest) -> impl Responder {
    let id = req.query().get("id").unwrap().to_string();

    let rows = get_status(id);

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
        auction_start: String::new(),
        auction_end: String::new(),
        auction_winner: String::new(),
    };

    for row in rows {
        my_row = Item::try_from_row(row).expect("into Item Struct");
        println!("Struct got :{:?}", my_row);
    }

    check_status((&my_row.status).to_string());

    let jsonstring = serde_json::to_string(&my_row);

    return jsonstring;
}

