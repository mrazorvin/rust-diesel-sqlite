use diesel::{
    dsl::{AsSelect, Select},
    query_dsl::methods::SelectDsl,
    sqlite::Sqlite,
    BoxableExpression, SelectableHelper, TextExpressionMethods,
};

use super::Post;
use crate::schema::posts;

pub(crate) fn get_posts() -> Select<posts::table, AsSelect<Post, Sqlite>> {
    posts::table.select(Post::as_select())
}

pub(crate) fn by_title<'a>(
    name: &str,
) -> Box<dyn BoxableExpression<posts::table, Sqlite, SqlType = diesel::sql_types::Bool> + 'a> {
    Box::new(posts::dsl::title.like(format!("%{name}%")))
}
