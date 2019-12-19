extern crate cfg_if;
extern crate juniper;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use juniper_from_schema::graphql_schema_from_file;
use serde_json;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

graphql_schema_from_file!("src/schema.graphql");

pub struct Context;
pub struct Query;

impl QueryFields for Query {
    fn field_ping(
        &self,
        _executor: &juniper::Executor<'_, Context>,
    ) -> juniper::FieldResult<Option<String>> {
        let result = String::from("pong");
        Ok(Some(result))
    }
}

#[wasm_bindgen]
pub fn execute_query(query: String) -> String {
    let ctx = Context;
    let schema = Schema::new(Query, juniper::EmptyMutation::new());
    let request = juniper::http::GraphQLRequest::new(query, None, None);
    let result = request.execute(&schema, &ctx);

    serde_json::to_string(&result).unwrap()
}
