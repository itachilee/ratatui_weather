diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    busmonitormanager (id) {
        id -> Int8,
        projectid -> Int8,
        minecaveid -> Int4,
        #[max_length = 200]
        devicename -> Varchar,
        #[max_length = 200]
        distributionboxname -> Nullable<Varchar>,
        #[max_length = 200]
        switchname -> Nullable<Varchar>,
        monitorlong -> Nullable<Numeric>,
        monitorlat -> Nullable<Numeric>,
        monitorx -> Nullable<Numeric>,
        monitory -> Nullable<Numeric>,
        devtypeid -> Int8,
        #[max_length = 12]
        devtypecode -> Nullable<Varchar>,
        #[max_length = 50]
        devnum -> Varchar,
        #[max_length = 50]
        simnum -> Nullable<Varchar>,
        #[max_length = 50]
        devip -> Varchar,
        #[max_length = 50]
        accnum -> Nullable<Varchar>,
        onlinestate -> Bool,
        #[max_length = 2000]
        devicearea -> Nullable<Varchar>,
        #[max_length = 500]
        location -> Nullable<Varchar>,
        #[max_length = 50]
        cameraindexcode -> Nullable<Varchar>,
        #[max_length = 500]
        remark -> Nullable<Varchar>,
        lastheartbeattime -> Timestamp,
        isenable -> Bool,
        ishaswarn -> Bool,
        #[max_length = 255]
        faceregsn -> Nullable<Varchar>,
        #[max_length = 255]
        customercode -> Nullable<Varchar>,
        #[max_length = 255]
        configjson -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}
