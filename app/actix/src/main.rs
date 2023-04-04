use actix_web::{error, web, App, HttpResponse, HttpServer, Responder};
use r2d2_sqlite::{self, SqliteConnectionManager};
use serde::Deserialize;

use type_ahead_db::{execute, Pool, Query};

// TODO move the handlers out
async fn hello() -> impl Responder {
    // TODO: load demo index.html page
    HttpResponse::Ok().body("Hello there...")
}

#[derive(Deserialize)]
struct TypeAheadParams {
    prefix: String,
    limit: i32,
}

async fn execute_query(pool: &Pool, query: Query) -> Result<HttpResponse, actix_web::Error> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get()).await?.unwrap(); // TODO: flat_map or similiar

    let result = web::block(move || execute(conn, query))
        .await?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

async fn find_cities_starting_with(
    pool: web::Data<Pool>,
    query_params: web::Query<TypeAheadParams>,
) -> Result<HttpResponse, actix_web::Error> {
    let query = Query::CityNamesStartingWith(query_params.prefix.to_string(), query_params.limit);

    execute_query(&pool, query).await
}

async fn find_countries_starting_with(
    pool: web::Data<Pool>,
    query_params: web::Query<TypeAheadParams>,
) -> Result<HttpResponse, actix_web::Error> {
    let query =
        Query::CountryNamesStartingWith(query_params.prefix.to_string(), query_params.limit);

    execute_query(&pool, query).await
}

async fn find_states_starting_with(
    pool: web::Data<Pool>,
    query_params: web::Query<TypeAheadParams>,
) -> Result<HttpResponse, actix_web::Error> {
    let query = Query::StateNamesStartingWith(query_params.prefix.to_string(), query_params.limit);

    execute_query(&pool, query).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: config all the things
    // TODO: logging
    let manager = SqliteConnectionManager::file("../artifacts/data.db");
    let pool = Pool::new(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // TODO: build the routes separately
            .service(web::resource("/").route(web::get().to(hello)))
            // TODO: /v1/type-aheads discoverability endpoint
            // TODO: can the /v1/type-ahead endpoints be grouped?
            .service(
                web::resource("/v1/type-ahead/cities")
                    .route(web::get().to(find_cities_starting_with)),
            )
            .service(
                web::resource("/v1/type-ahead/countries")
                    .route(web::get().to(find_countries_starting_with)),
            )
            .service(
                web::resource("/v1/type-ahead/states")
                    .route(web::get().to(find_states_starting_with)),
            )
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
