//extern crate cdrs;

use cdrs::{self,types::prelude::*};
use std::str;
use cdrs::frame::TryFromRow;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::frame::IntoBytes;
#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub creator: String,
    pub title: String,
    pub description: String,
    pub currency_id: String,
    pub increment: f32,
    pub reserve_price: f32,
    pub price: f32,
    pub status: String,
    pub auction_duration: String,
    pub auction_start: String,
    pub auction_end: String,
    pub auction_winner: String,
}
