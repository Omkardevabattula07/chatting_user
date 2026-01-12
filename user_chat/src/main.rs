 use actix_web::{App, HttpServer};
 use actix_files as fs;


 mod handlers;
 mod routes;


 #[actix_web::main]

 async fn main()-> std::io::Result<()>{


    println!("Starting server at http://");
    HttpServer::new(||{


        App::new()
        .service(fs::Files::new("/static", "./static").show_files_listing())
        .configure(routes::config_routes)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await      
 }