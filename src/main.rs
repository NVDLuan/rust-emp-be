use emp_be::app;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run().await
}
