use warp::Filter;

fn hi_user(param: String, accepts: String) -> std::string::String {
    format!("Hi {}, whose accepts {}", param, accepts)
}

fn main() {
    println!("Hello, world!");

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

    let routes = warp::get2().and(hello.or(hi));
    warp::serve(routes).run(([127, 0, 0, 1], 3030));
}
