use dbconnection::*;
use cdrs::query::QueryExecutor;
pub fn create_keyspace(session: &CurrentSession) {
    let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS auction WITH REPLICATION = { \
                                 'class' : 'SimpleStrategy', 'replication_factor' : 1 };";
    session.query(create_ks).expect("Keyspace creation error");
}

pub fn create_table(session: &CurrentSession) {
    let create_table_cql =
        "CREATE TABLE auction.items (
    id text,
    creator text,
    title text,
    description text,
    currency_id text,
    increment float,
    reserve_price float,
    price float,
    status text,
    auction_duration text,
    auction_start text,
    auction_end text,
    auction_winner text,
    PRIMARY KEY (id,creator))";
    session
        .query(create_table_cql)
        .expect("Table creation error");
}

