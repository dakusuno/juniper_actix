use juniper::http::playground::playground_source;
use actix_web::{web,Error,HttpResponse};
use std::sync::Arc;
use juniper::http::GraphQLRequest;
use crate::db::PostgresPool;
use crate::graphql::Schema;
use crate::context::GraphQLContext;
use crate::graphql::create_schema;

pub fn graphql_endpoints(config:&mut web::ServiceConfig){
    config
        .data(create_schema())
        .route("/graphql",web::post().to(graphql))
        .route("/graphql",web::get().to(graphql_playground));

}
async fn graphql_playground()->HttpResponse{
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/graphql",None))
}
pub async fn graphql(
    pool: web::Data<PostgresPool>,
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = GraphQLContext {
        pool: pool.get_ref().to_owned(),
    };
    let res = web::block(move || {
        let res = data.execute_sync(&schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
        .await
        .map_err(Error::from)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}

