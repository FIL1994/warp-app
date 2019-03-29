extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::{env, str};

use warp::{reject, Filter};

fn hi_user(param: String, accepts: String) -> std::string::String {
    format!("Hi {}, whose accepts {}", param, accepts)
}

type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;
type PooledSqlite = PooledConnection<ConnectionManager<SqliteConnection>>;

fn sqlite_pool() -> SqlitePool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in env file");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::new(manager).expect("Postgres connection pool could not be created")
}

#[derive(Deserialize, Serialize)]
struct Employee {
    name: String,
    rate: u32,
}

fn main() {
    // hello/:string
    let hello = warp::path("hello")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| format!("Hello {}, whose agent is {}", param, agent));

    // hi/:string
    let hi = warp::path("hi")
        .and(warp::path::param())
        .and(warp::header("accept"))
        .map(hi_user);

    let pool = sqlite_pool();
    let sq = warp::any()
        .map(move || pool.clone())
        .and_then(|pool: SqlitePool| match pool.get() {
            Ok(conn) => Ok(conn),
            Err(_) => Err(reject::server_error()),
        });

    let from_db = warp::path::end().and(sq).map(|db: PooledSqlite| "Get Data");

    let get_json = warp::path("json").map(|| {
        let ids = vec![1, 2, 3];
        warp::reply::json(&ids)
    });

    let post_json = warp::path("json")
        .and(warp::body::json())
        .map(|employee: Employee| warp::reply::json(&employee));

    let list = warp::get2().and(from_db.or(hello).or(hi).or(get_json));
    let create = warp::post2().and(post_json);

    println!("Starting server");
    let routes = list.or(create);
    warp::serve(routes).run(([127, 0, 0, 1], 3030));
}
