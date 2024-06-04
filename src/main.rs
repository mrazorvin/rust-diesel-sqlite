use diesel::prelude::*;
use std::{env, error::Error};

mod posts;
mod schema;

fn main() -> Result<(), Box<dyn Error>> {
    let Some(title_filter) = env::args().skip(1).next() else {
        panic!("title filter missed try to use `cargo run \"um\"`");
    };

    let mut conn = SqliteConnection::establish("file:db.sqlite")?;

    let posts: Vec<posts::Post> =
        posts::get_posts().filter(posts::by_title(&title_filter)).load(&mut conn)?;

    for post in posts {
        println!("Title: {}", post.title);
        println!("Body : {}", post.body);
        println!("{:-<1$}", "", 30);
    }

    Ok(())
}
