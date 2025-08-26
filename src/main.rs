use actix_web::{App, HttpServer, middleware::Logger};
use env_logger::Env;
use infrastructure::http::routes::config as routes_config;

mod application {
    pub mod transacao_service;
}

mod domain {
    pub mod transacao;
}

mod infrastructure {
    pub mod http {
        pub mod handlers;
        pub mod routes;
    }
    pub mod db {
        pub mod transacao_repo;
    }

    pub mod error;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let server =
        HttpServer::new(move || App::new().wrap(Logger::default()).configure(routes_config))
            .bind(("127.0.0.1", 8080))?
            .run();
    println!("Server running at http://localhost:{}/", "8080");

    server.await
}
