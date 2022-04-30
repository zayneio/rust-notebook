extern crate rust_notebook;
extern crate diesel;

// The use rust_notebook::schema::posts::dsl::* line imports a bunch of aliases 
// so that we can say posts instead of posts::table, and published instead of posts::published. 
// It’s useful when we’re only dealing with a single table, but that’s not always what we want.
use self::rust_notebook::*;
use self::models::*;
use self::diesel::prelude::*;


// We can run our script with `cargo run --bin show_posts`.
fn main() {
  use rust_notebook::schema::posts::dsl::*;

  let connection = establish_connection();
  let results = posts.filter(published.eq(true))
      .limit(5)
      .load::<Post>(&connection)
      .expect("Error loading posts");

  println!("Displaying {} posts", results.len());
  
  for post in results {
    println!("{}", post.title);
    println!("----------\n");
    println!("{}", post.body);
  }
}