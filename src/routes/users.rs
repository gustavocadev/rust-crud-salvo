use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  name: String,
}

#[derive(Serialize, Debug)]
pub struct BaseResponse {
  results: Vec<User>,
}

#[handler]
pub async fn get_users(res: &mut Response) {
  let users = BaseResponse {
    results: vec![User {
      name: "Yashira Mancilla".to_string(),
    }],
  };
  // print
  println!("{:?}", users);

  res.render(Json(users))
}

#[handler]
pub async fn create_user(req: &mut Request, res: &mut Response) {
  let user = req.parse_body::<User>().await.unwrap();

  println!("{:?}", user);

  res.render(Json(user));
}
