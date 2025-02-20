use std::hash::Hash;

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Timestamp;
use serde::{Deserialize, Serialize};
use std::io::Write;
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

#[derive(serde::Serialize, Queryable, Selectable, Clone, PartialEq, Eq)]
#[diesel(table_name = crate::db::schema::busmonitormanager)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BusMonitorManager {
    pub id: i64,
    pub devtypeid: i64,
    pub devip: String,
    pub devicename: String,
    pub isdelete: bool,
}

impl Hash for BusMonitorManager {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
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

#[derive(serde::Serialize, serde::Deserialize, Queryable, Selectable)]
#[diesel(table_name =crate::db::schema::system_security_info)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SystemSecurityInfo {
    pub id: i32,
    pub start_date: NaiveDateTime,
    pub end_date: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::system_security_info)]
pub struct NewSystemSecurityInfo {
    pub start_date: NaiveDateTime,
    pub end_date: Option<NaiveDateTime>,
}
