use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Post {
  id: i32,
  title: String,
  body: String,
}

#[handler]

pub fn get_posts(req: &mut Request, res: &mut Response) {
  res.render(Text::Plain("Get all posts"));
}
