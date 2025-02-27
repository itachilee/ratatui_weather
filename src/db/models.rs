use std::hash::Hash;

use chrono::NaiveDateTime;
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

#[derive(serde::Serialize, Queryable, Selectable, Clone, PartialEq, Eq)]
#[diesel(table_name = crate::db::schema::busmonitormanager)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BusMonitorManager {
    pub id: i64,
    pub devtypeid: i64,
    pub devip: String,
    pub devicename: String,
    pub isdelete: bool,
    pub onlinestate: bool,
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

// 将 `WarningReason` 映射到 `WarningReasonCode`
#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::warnings)]
pub struct WarningInsert<'a> {
    pub sensor_type: &'a str,
    pub dev_ip: &'a str,
    pub value: f64,
    pub threshold: f64,
    pub reason: i32,
    pub description: &'a str,
    pub timestamp: chrono::NaiveDateTime, // 或者使用 chrono::DateTime<Utc>
}

#[derive(serde::Serialize, serde::Deserialize, Queryable, Selectable)]
#[diesel(table_name =crate::db::schema::warnings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WarningQuery {
    pub id: i32,
    pub sensor_type: String,
    pub dev_ip: String,
    pub value: f64,
    pub threshold: f64,
    pub reason: i32,
    pub description: String,
    pub timestamp: chrono::NaiveDateTime, // 或者使用 chrono::DateTime<Utc>
}

#[derive(Queryable)]
struct DeviceOnlineCount {
    devtypeid: i64,
    count: i64,
}
