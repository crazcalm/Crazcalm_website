use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Serves directory for static items
    let static_items = warp::path("static").and(warp::fs::dir("../html/static"));

    // Main page for site
    let main_page = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("../html/index.html"));

    // Setting up my routes
    let routes = main_page.or(static_items);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
