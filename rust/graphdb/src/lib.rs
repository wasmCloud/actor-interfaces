pub mod generated;
extern crate wapc_guest as guest;
use generated::*;
use guest::prelude::*;

#[no_mangle]
pub fn wapc_init() {
    Handlers::register_query(query);
    Handlers::register_delete(delete);
}

fn query(_query: String, _graph_name: String) -> HandlerResult<QueryResponse> {
    Ok(QueryResponse::default()) // TODO: Provide implementation.
}

fn delete(_graph_name: String) -> HandlerResult<DeleteResponse> {
    Ok(DeleteResponse::default()) // TODO: Provide implementation.
}
