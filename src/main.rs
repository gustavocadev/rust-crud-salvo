use salvo::prelude::*;

// this way we can use the routes from the routes folder which turn out to be the way to go
mod routes;
use routes::{posts, users};

#[tokio::main]
async fn main() {
  let app = Router::new().push(
    Router::with_path("/api")
      .push(Router::with_path("/posts").get(posts::get_posts))
      .push(
        Router::with_path("/users")
          .get(users::get_users)
          .post(users::create_user),
      ),
  );

  Server::new(TcpListener::bind("127.0.0.1:7878"))
    .serve(app)
    .await;
}
