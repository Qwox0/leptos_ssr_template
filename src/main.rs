const SOCKET_ADDRESS: &str = "0.0.0.0:33080";

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_ssr_template::app::*;
    use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    ssl_builder.set_private_key_file("privkey16.pem", SslFiletype::PEM)?;
    ssl_builder.set_certificate_chain_file("fullchain16.pem")?;

    let conf = get_configuration(Some("./Cargo.toml")).await.unwrap();
    let addr = conf.leptos_options.site_address;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|cx| view! { cx, <App/> });

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        App::new()
            .wrap(middleware::Compress::default())
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                |cx| view! { cx, <App/> },
            )
            .service(Files::new("/", &leptos_options.site_root))
    })
    .bind(&addr)?
    .bind_openssl(SOCKET_ADDRESS, ssl_builder)?
    .run()
    .await
}
