use crate::configuration::config_database;
use crate::configuration::config_loader::Settings;
use crate::route::routes_column::get_column_routes;
use crate::route::routes_item::get_item_routes;
use crate::route::routes_permission::get_permission_routes;
use crate::route::routes_role::get_role_routes;
use crate::route::routes_user::get_user_routes;
use ::function_name::named;
use async_once::AsyncOnce;
use dao::dao_common;
use log::debug;
use sea_orm::ConnectionTrait;
use sea_orm::DbConn;
use sea_orm::Statement;
use serde_json::json;
use structure::structures::DevBoardErrorType;
use structure::structures::DevBoardGenericError;
use util::util_authentication::Unauthorized;
use util::util_permission::init_permissions;
use warp::hyper::Method;
use warp::hyper::StatusCode;
use warp::reply;
use warp::Filter;
use warp::Rejection;
use warp::Reply;

mod configuration;
mod controller;
mod dao;
mod route;
mod structure;
mod util;
type GenericResult<T> = std::result::Result<T, Rejection>;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SETTINGS: Settings = Settings::init_configuration().unwrap();
    static ref DB_POOL: AsyncOnce<DbConn> = AsyncOnce::new(async {
        let db = config_database::establish_connection().await;
        db.unwrap()
    });
}

#[tokio::main]
async fn main() {
    init_logging();
    init_db().await;
    init_permissions(DB_POOL.get().await).await;
    dao_common::init_admin().await; // default superuser
    dao_common::init_user_role().await; // this role is assigned when a new user is created
    init_test();
    init_server().await;
}

fn init_test() {}

fn init_logging() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
}

#[named]
async fn init_db() {
    println!("FN: {:?}", function_name!());
    debug!("Checking DB connection...");
    let db = DB_POOL.get().await;
    let result = db
        .query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT 1 from db_column limit 1;".to_owned(),
        ))
        .await;
    if result.is_err() {
        debug!("[DB RESULT] Connection to [DB FAILED]: {:?}", result.err());
    } else {
        debug!("[DB RESULT] DB Connection [OK]")
    }
}

async fn init_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let any_origin_3 = warp::cors()
        // .allow_any_origin()
        .allow_origin("http://localhost:3000")
        .allow_headers(vec![
            "Access-Control-Allow-Credentials",
            "Access-Control-Allow-Headers",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Origin",
            "Accept",
            "Content-Type",
            "Accept-Encoding",
            "Accept-Language",
            "Cache-Control",
            "Connection",
            "Host",
            "Pragma",
            "Referer",
            "User-Agent",
            "X-Requested-With",
            "Content-Type",
            "Cookie",
            "sec-ch-ua",
            "sec-ch-ua-mobile",
            "sec-ch-ua-platform",
            "Sec-Fetch-Dest",
            "Sec-Fetch-Mode",
            "Sec-Fetch-Site",
            "Sec-Fetch-User",
            "Sec-WebSocket-Extensions",
            "Sec-WebSocket-Key",
            "Sec-WebSocket-Version",
            "Upgrade-Insecure-Requests",
            "Upgrade",
            "Authorization",
        ])
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
            Method::HEAD,
        ])
        .allow_credentials(true);

    get_column_routes()
        .await
        .or(get_item_routes().await)
        .or(get_user_routes().await)
        .or(get_role_routes().await)
        .or(get_permission_routes().await)
        .recover(handle_rejection)
        .with(&any_origin_3)
        .with(warp::log("api"))
}

async fn init_server() {
    debug!("server run on port {}", SETTINGS.server_port);
    warp::serve(init_routes().await)
        .run(([0, 0, 0, 0], SETTINGS.server_port))
        .await;
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if err.is_not_found() {
        Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else if let Some(e) = err.find::<Unauthorized>() {
        let generic_error = DevBoardGenericError {
            success: false,
            message: e.error_message.to_owned(),
            code: -1,
            err_type: DevBoardErrorType::Error,
        };
        let resp = json!(generic_error);
        let res: String = resp.to_string();
        let boxed = Box::leak(res.into_boxed_str());
        Ok(reply::with_status(boxed, StatusCode::UNAUTHORIZED))
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
