use diesel::prelude::*;

#[derive(serde::Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use crate::db::schema::posts;
#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(serde::Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::busmonitormanager)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BusMonitorManager {
    pub id: i64,
    pub devtypeid: i64,
    pub devip: String,
    pub isdelete: bool,
}

#[derive(serde::Serialize, Queryable, Selectable, Default, Clone)]
#[diesel(table_name = crate::db::schema::busdevtypemanager)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BusDevTypeManager {
    pub id: i64,
    pub issensor: bool,
    pub isdelete: bool,
    pub devtypename: String,
    pub dataconfig: Option<String>,
}
