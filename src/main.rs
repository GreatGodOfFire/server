use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use starwars::{QueryRoot, StarWars, StarWarsSchema};


async fn index(schema: web::Data<StarWarsSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();

    HttpServer::new(move || {
        let mut app = App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index));
        println!("Playground: http://localhost:8000");
        if cfg!(debug_assertions) {
            app = app.service(web::resource("/").guard(guard::Get()).to(index_playground));
        }
        app
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
