use rocket::http::Method;
use rocket_cors::{AllOrSome, AllowedHeaders, Cors, CorsOptions, Origins};

pub fn config_cors(allowed_origins: AllOrSome<Origins>, allowed_methods: Vec<Method>) -> Cors {
    // Check for an empty vector of allowed methods
    if allowed_methods.is_empty() {
        eprintln!("Error: Empty vector of allowed methods");
        return CorsOptions::default().to_cors().unwrap();
    }

    // Attempt to create CORS configuration
    let cors = CorsOptions {
        allowed_origins: allowed_origins,
        allowed_methods: allowed_methods.into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        ..CorsOptions::default()
    }
    .to_cors();

    // Check for an error in CORS setup
    match cors {
        Ok(cors) => cors,
        Err(err) => {
            eprintln!("Error in CORS setup: {}", err);
            CorsOptions::default().to_cors().unwrap()
        }
    }
}
