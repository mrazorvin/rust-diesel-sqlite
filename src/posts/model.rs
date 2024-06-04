use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct Post {
    #[allow(unused)]
    id: i32,
    pub(crate) title: String,
    pub(crate) body: String,
}
