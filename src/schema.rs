// @generated automatically by Diesel CLI.

diesel::table! {
    busbasicinfo (id) {
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
        id -> Int8,
        workpeople -> Int4,
        systemworkdays -> Int4,
        deviceworkcount -> Int4,
    }
}

diesel::table! {
    busbroadcastmanage (id) {
        id -> Int8,
        #[max_length = 200]
        title -> Varchar,
        #[max_length = 2000]
        content -> Nullable<Varchar>,
        #[max_length = 50]
        author -> Nullable<Varchar>,
        #[max_length = 50]
        fileids -> Nullable<Varchar>,
        total -> Int4,
        residuenum -> Int4,
        istop -> Int4,
        #[max_length = 500]
        note -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    buscmanagefzjc (id) {
        id -> Int8,
        #[max_length = 50]
        cmanagetype -> Varchar,
        #[max_length = 100]
        cmanagetitle -> Varchar,
        #[max_length = 255]
        cmanagecontent -> Varchar,
        stime -> Timestamp,
        #[max_length = 500]
        participants -> Varchar,
        #[max_length = 500]
        accnum -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    buscmanagehy (id) {
        id -> Int8,
        #[max_length = 500]
        cmanagetitle -> Nullable<Varchar>,
        #[max_length = 500]
        cmanagecontent -> Varchar,
        stime -> Timestamp,
        #[max_length = 500]
        participants -> Varchar,
        #[max_length = 500]
        accnum -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    buscmanagekp (id) {
        id -> Int8,
        #[max_length = 500]
        cmanagetitle -> Nullable<Varchar>,
        #[max_length = 100]
        cmanageplace -> Nullable<Varchar>,
        #[max_length = 500]
        cmanagecontent -> Varchar,
        #[max_length = 500]
        accnum -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    buscmanageyl (id) {
        id -> Int8,
        #[max_length = 500]
        cmanagetype -> Nullable<Varchar>,
        #[max_length = 500]
        cmanagecontent -> Varchar,
        stime -> Timestamp,
        #[max_length = 500]
        participants -> Varchar,
        #[max_length = 500]
        accnum -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busdevtypemanager (id) {
        id -> Int8,
        #[max_length = 100]
        devtypecode -> Varchar,
        #[max_length = 150]
        devtypename -> Varchar,
        issensor -> Bool,
        #[max_length = 255]
        dataconfig -> Nullable<Varchar>,
        isenable -> Bool,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busemergencyplan (id) {
        id -> Int8,
        #[max_length = 50]
        plannum -> Nullable<Varchar>,
        #[max_length = 200]
        planname -> Nullable<Varchar>,
        #[max_length = 255]
        plancontent -> Varchar,
        #[max_length = 500]
        accnum -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busfaceregmqttcommandhistory (id) {
        id -> Int8,
        #[max_length = 255]
        cmdid -> Nullable<Varchar>,
        #[max_length = 255]
        cmd -> Nullable<Varchar>,
        #[max_length = 255]
        topic -> Nullable<Varchar>,
        #[max_length = 5000]
        commandtext -> Nullable<Varchar>,
        ishasresponse -> Nullable<Bool>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busfaceregmqttresponse (id) {
        id -> Int8,
        #[max_length = 255]
        cmdid -> Nullable<Varchar>,
        #[max_length = 255]
        sn -> Varchar,
        #[max_length = 255]
        code -> Nullable<Varchar>,
        #[max_length = 255]
        cmd -> Varchar,
        #[max_length = 255]
        topic -> Varchar,
        #[max_length = 5000]
        commandtext -> Varchar,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busfactory (id) {
        id -> Int8,
        #[max_length = 200]
        factoryname -> Varchar,
        #[max_length = 200]
        factoryphone -> Nullable<Varchar>,
        #[max_length = 200]
        comment -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busfirealarm (id) {
        id -> Int8,
        #[max_length = 200]
        title -> Varchar,
        #[max_length = 2000]
        content -> Varchar,
        #[max_length = 50]
        author -> Varchar,
        #[max_length = 100]
        source -> Varchar,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busfireanalysis (id) {
        id -> Int8,
        eventid -> Int4,
        firelevel -> Int4,
        #[max_length = 200]
        rrend -> Nullable<Varchar>,
        #[max_length = 200]
        area -> Nullable<Varchar>,
        #[max_length = 200]
        expectloss -> Nullable<Varchar>,
        #[max_length = 200]
        note -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busfirewarning (id) {
        id -> Int8,
        #[max_length = 200]
        title -> Varchar,
        #[max_length = 2000]
        content -> Nullable<Varchar>,
        #[max_length = 50]
        author -> Nullable<Varchar>,
        #[max_length = 100]
        source -> Nullable<Varchar>,
        istop -> Int4,
        typeid -> Int4,
        longitude -> Nullable<Numeric>,
        latitude -> Nullable<Numeric>,
        #[max_length = 200]
        fileids -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busguardperson (id) {
        id -> Int8,
        #[max_length = 255]
        expertname -> Nullable<Varchar>,
        #[max_length = 255]
        expertcompany -> Nullable<Varchar>,
        #[max_length = 255]
        phone -> Varchar,
        #[max_length = 255]
        position -> Nullable<Varchar>,
        #[max_length = 255]
        accnum -> Nullable<Varchar>,
        #[max_length = 255]
        processcontent -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busguardrecord (id) {
        id -> Int8,
        #[max_length = 50]
        guardperson -> Nullable<Varchar>,
        #[max_length = 50]
        guardtype -> Nullable<Varchar>,
        starttime -> Timestamp,
        endtime -> Timestamp,
        #[max_length = 500]
        guardcontent -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    bushazardmanage (id) {
        id -> Int8,
        latitude -> Nullable<Numeric>,
        longitude -> Nullable<Numeric>,
        #[max_length = 500]
        name -> Nullable<Varchar>,
        #[max_length = 500]
        address -> Nullable<Varchar>,
        typeid -> Int4,
        status -> Int4,
        #[max_length = 1000]
        note -> Nullable<Varchar>,
        #[max_length = 1000]
        handleopinion -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    bushikevent (id) {
        id -> Int8,
        #[max_length = 255]
        devid -> Varchar,
        #[max_length = 255]
        eventid -> Varchar,
        #[max_length = 255]
        eventname -> Nullable<Varchar>,
        happentime -> Nullable<Timestamp>,
        #[max_length = 255]
        imageurls -> Nullable<Varchar>,
        isprocessed -> Nullable<Bool>,
        #[max_length = 255]
        processedby -> Nullable<Varchar>,
        processtime -> Nullable<Timestamp>,
        #[max_length = 255]
        processremark -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busjrxdata (id) {
        id -> Int8,
        #[max_length = 255]
        devid -> Varchar,
        v1 -> Float8,
        v2 -> Nullable<Float8>,
        realv1 -> Nullable<Float8>,
        realv2 -> Nullable<Float8>,
        createtime -> Timestamp,
    }
}

diesel::table! {
    busleavemanage (id) {
        id -> Int8,
        typeid -> Int4,
        #[max_length = 200]
        title -> Varchar,
        #[max_length = 500]
        note -> Nullable<Varchar>,
        begindate -> Nullable<Timestamp>,
        enddate -> Nullable<Timestamp>,
        leaveuserid -> Nullable<Int8>,
        approveruserid -> Nullable<Int8>,
        #[max_length = 500]
        approveropinion -> Nullable<Varchar>,
        status -> Int4,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busmasbasestation (id) {
        id -> Int8,
        #[max_length = 255]
        customercode -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        installlocation -> Varchar,
        #[max_length = 255]
        installdate -> Varchar,
        #[max_length = 255]
        x -> Varchar,
        #[max_length = 255]
        y -> Varchar,
        #[max_length = 255]
        z -> Varchar,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busminecavemanager (id) {
        id -> Int8,
        #[max_length = 200]
        minecavename -> Varchar,
        #[max_length = 200]
        svgpath -> Nullable<Varchar>,
        isshow -> Bool,
        #[max_length = 255]
        customercode -> Nullable<Varchar>,
        #[max_length = 255]
        approvedcount -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busminepeoplemanager (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        nikname -> Nullable<Varchar>,
        #[max_length = 255]
        idcardnumber -> Nullable<Varchar>,
        #[max_length = 255]
        gender -> Nullable<Varchar>,
        #[max_length = 255]
        faceimage -> Nullable<Varchar>,
        #[max_length = 5000]
        imagedatabase64 -> Nullable<Varchar>,
        #[max_length = 255]
        usrtype -> Nullable<Varchar>,
        #[max_length = 255]
        authtype -> Nullable<Varchar>,
        #[max_length = 255]
        wiegandnumber -> Nullable<Varchar>,
        #[max_length = 255]
        imageurl -> Nullable<Varchar>,
        stime -> Int8,
        etime -> Int8,
        isfacereg -> Bool,
        #[max_length = 255]
        perid -> Nullable<Varchar>,
        #[max_length = 255]
        faceid -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busmonitordata_20250201 (id) {
        id -> Int8,
        #[max_length = 255]
        devip -> Varchar,
        #[max_length = 255]
        devtypename -> Varchar,
        #[max_length = 255]
        minecavename -> Nullable<Varchar>,
        minecaveid -> Nullable<Int8>,
        devtypeid -> Int8,
        monitorid -> Int8,
        #[max_length = 255]
        v1 -> Varchar,
        #[max_length = 255]
        v2 -> Nullable<Varchar>,
        #[max_length = 255]
        realv1 -> Nullable<Varchar>,
        #[max_length = 255]
        realv2 -> Nullable<Varchar>,
        isprocessed -> Nullable<Bool>,
        #[max_length = 255]
        processremark -> Nullable<Varchar>,
        iswarn -> Bool,
        #[max_length = 255]
        warnremark -> Nullable<Varchar>,
        #[max_length = 255]
        originhexstr -> Nullable<Varchar>,
        createtime -> Timestamp,
        #[max_length = 255]
        customercode -> Nullable<Varchar>,
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

diesel::table! {
    buspatroldetail (id) {
        id -> Int8,
        workforcemanageid -> Int8,
        #[max_length = 200]
        fileids -> Nullable<Varchar>,
        #[max_length = 200]
        position -> Nullable<Varchar>,
        longitude -> Nullable<Numeric>,
        latitude -> Nullable<Numeric>,
        #[max_length = 200]
        person -> Nullable<Varchar>,
        #[max_length = 500]
        description -> Nullable<Varchar>,
        #[max_length = 100]
        weather -> Nullable<Varchar>,
        #[max_length = 500]
        note -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busplanmanage (id) {
        id -> Int8,
        #[max_length = 200]
        title -> Varchar,
        #[max_length = 500]
        planbackground -> Nullable<Varchar>,
        #[max_length = 1000]
        content -> Nullable<Varchar>,
        #[max_length = 500]
        preventivemeasure -> Nullable<Varchar>,
        #[max_length = 500]
        exerciseevaluation -> Nullable<Varchar>,
        #[max_length = 500]
        note -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busrepair (id) {
        id -> Int8,
        #[max_length = 200]
        origin_devid -> Varchar,
        #[max_length = 200]
        new_devid -> Varchar,
        #[max_length = 200]
        image_num -> Nullable<Varchar>,
        #[max_length = 200]
        description -> Nullable<Varchar>,
        #[max_length = 200]
        modifier -> Nullable<Varchar>,
        #[max_length = 200]
        creator -> Nullable<Varchar>,
        modifydate -> Nullable<Timestamp>,
        createdate -> Nullable<Timestamp>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busrescueforces (id) {
        id -> Int8,
        #[max_length = 100]
        phone -> Varchar,
        #[max_length = 200]
        name -> Varchar,
        #[max_length = 2000]
        introduction -> Nullable<Varchar>,
        #[max_length = 200]
        address -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    bussafetyroute (id) {
        id -> Int8,
        #[max_length = 255]
        routename -> Varchar,
        #[max_length = 255]
        routecomment -> Varchar,
        #[max_length = 255]
        routepath -> Varchar,
        #[max_length = 255]
        accnum -> Varchar,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    bussafetysupplies (id) {
        id -> Int8,
        #[max_length = 255]
        suppliesnum -> Varchar,
        #[max_length = 255]
        suppliesname -> Varchar,
        suppliescount -> Int4,
        #[max_length = 255]
        liableperson -> Varchar,
        #[max_length = 255]
        liablephone -> Varchar,
        #[max_length = 255]
        location -> Varchar,
        #[max_length = 255]
        accnum -> Varchar,
        #[max_length = 255]
        remark -> Varchar,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busvehiclemanage (id) {
        id -> Int8,
        #[max_length = 50]
        platenumber -> Nullable<Varchar>,
        #[max_length = 50]
        owner -> Nullable<Varchar>,
        #[max_length = 50]
        phone -> Nullable<Varchar>,
        #[max_length = 500]
        note -> Nullable<Varchar>,
        #[max_length = 100]
        fileids -> Nullable<Varchar>,
        status -> Int4,
        personid -> Nullable<Int4>,
        cardno -> Nullable<Int4>,
        platetype -> Int4,
        platecolor -> Int4,
        vehicletype -> Int4,
        vehiclecolor -> Int4,
        #[max_length = 100]
        regionindexcode -> Nullable<Varchar>,
        #[max_length = 100]
        vehiclegroup -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busvisitormanage (id) {
        id -> Int8,
        typeid -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 50]
        phone -> Nullable<Varchar>,
        #[max_length = 50]
        idcard -> Nullable<Varchar>,
        #[max_length = 500]
        note -> Nullable<Varchar>,
        #[max_length = 50]
        fileid -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    buswaterpatrol (id) {
        id -> Int8,
        #[max_length = 255]
        patrol_name -> Varchar,
        start_time -> Timestamp,
        end_time -> Timestamp,
        #[max_length = 255]
        patrol_person -> Varchar,
        #[max_length = 255]
        person_phone -> Varchar,
        #[max_length = 255]
        patrol_content -> Varchar,
        #[max_length = 255]
        patrol_result -> Varchar,
        #[max_length = 255]
        accnum -> Varchar,
        #[max_length = 255]
        modifier -> Varchar,
        #[max_length = 255]
        creator -> Varchar,
        modifydate -> Timestamp,
        createdate -> Timestamp,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    buswatersupplies (id) {
        id -> Int8,
        #[max_length = 255]
        supplies_name -> Varchar,
        #[max_length = 255]
        supplies_num -> Varchar,
        supplies_count -> Int4,
        #[max_length = 255]
        location -> Varchar,
        #[max_length = 255]
        remark -> Varchar,
        #[max_length = 255]
        liable_phone -> Varchar,
        #[max_length = 255]
        liable_person -> Varchar,
        #[max_length = 255]
        accnum -> Varchar,
        #[max_length = 255]
        modifier -> Varchar,
        #[max_length = 255]
        creator -> Varchar,
        modifydate -> Timestamp,
        createdate -> Timestamp,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    busworkforcemanage (id) {
        id -> Int8,
        #[max_length = 200]
        name -> Varchar,
        begindate -> Nullable<Timestamp>,
        enddate -> Nullable<Timestamp>,
        #[max_length = 200]
        patrolid -> Nullable<Varchar>,
        #[max_length = 500]
        patrolname -> Nullable<Varchar>,
        valid -> Int4,
        status -> Int4,
        #[max_length = 200]
        patrolarea -> Nullable<Varchar>,
        #[max_length = 500]
        note -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    monitordataalarm_20250201 (id) {
        id -> Int8,
        #[max_length = 255]
        devip -> Varchar,
        #[max_length = 255]
        devtypename -> Varchar,
        #[max_length = 255]
        proname -> Nullable<Varchar>,
        proid -> Nullable<Int8>,
        devtypeid -> Int8,
        monitorid -> Int8,
        v1 -> Numeric,
        #[max_length = 255]
        v1unit -> Varchar,
        offsetflag -> Int4,
        #[max_length = 255]
        checkrange -> Varchar,
        minrange -> Nullable<Numeric>,
        maxrange -> Nullable<Numeric>,
        isprocessed -> Bool,
        #[max_length = 255]
        processremark -> Nullable<Varchar>,
        iswarn -> Bool,
        #[max_length = 500]
        originhexstr -> Nullable<Varchar>,
        createtime -> Timestamp,
    }
}

diesel::table! {
    msgsendnotice (id) {
        id -> Int8,
        templateid -> Int8,
        sendtype -> Int4,
        #[max_length = 100]
        sendto -> Varchar,
        #[max_length = 50]
        content -> Varchar,
        status -> Int4,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    msgtemplate (id) {
        id -> Int8,
        #[max_length = 200]
        name -> Varchar,
        #[max_length = 1000]
        content -> Nullable<Varchar>,
        #[max_length = 100]
        url -> Nullable<Varchar>,
        #[max_length = 100]
        accesskey -> Nullable<Varchar>,
        #[max_length = 100]
        code -> Nullable<Varchar>,
        #[max_length = 100]
        accesssecret -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Int4,
        status -> Int4,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    syscodegen (id) {
        id -> Int8,
        #[max_length = 32]
        authorname -> Nullable<Varchar>,
        #[max_length = 8]
        tableprefix -> Nullable<Varchar>,
        #[max_length = 32]
        generatetype -> Nullable<Varchar>,
        #[max_length = 64]
        configid -> Nullable<Varchar>,
        #[max_length = 64]
        dbname -> Nullable<Varchar>,
        #[max_length = 64]
        dbtype -> Nullable<Varchar>,
        #[max_length = 256]
        connectionstring -> Nullable<Varchar>,
        #[max_length = 128]
        tablename -> Nullable<Varchar>,
        #[max_length = 128]
        namespace -> Nullable<Varchar>,
        #[max_length = 128]
        busname -> Nullable<Varchar>,
        menupid -> Int8,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    syscodegenconfig (id) {
        id -> Int8,
        codegenid -> Int8,
        #[max_length = 128]
        columnname -> Varchar,
        #[max_length = 128]
        columncomment -> Nullable<Varchar>,
        #[max_length = 64]
        nettype -> Nullable<Varchar>,
        #[max_length = 64]
        effecttype -> Nullable<Varchar>,
        #[max_length = 64]
        fkentityname -> Nullable<Varchar>,
        #[max_length = 128]
        fktablename -> Nullable<Varchar>,
        #[max_length = 64]
        fkcolumnname -> Nullable<Varchar>,
        #[max_length = 64]
        fkcolumnnettype -> Nullable<Varchar>,
        #[max_length = 64]
        dicttypecode -> Nullable<Varchar>,
        #[max_length = 8]
        whetherretract -> Nullable<Varchar>,
        #[max_length = 8]
        whetherrequired -> Nullable<Varchar>,
        #[max_length = 8]
        querywhether -> Nullable<Varchar>,
        #[max_length = 16]
        querytype -> Nullable<Varchar>,
        #[max_length = 8]
        whethertable -> Nullable<Varchar>,
        #[max_length = 8]
        whetheraddupdate -> Nullable<Varchar>,
        #[max_length = 8]
        columnkey -> Nullable<Varchar>,
        #[max_length = 64]
        datatype -> Nullable<Varchar>,
        #[max_length = 8]
        whethercommon -> Nullable<Varchar>,
        displaycolumn -> Nullable<Text>,
        #[max_length = 128]
        valuecolumn -> Nullable<Varchar>,
        #[max_length = 128]
        pidcolumn -> Nullable<Varchar>,
        orderno -> Int4,
        width -> Nullable<Int4>,
        #[max_length = 255]
        widthtype -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    sysconfig (id) {
        id -> Int8,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 64]
        code -> Nullable<Varchar>,
        #[max_length = 4000]
        value -> Nullable<Varchar>,
        sysflag -> Int4,
        #[max_length = 64]
        groupcode -> Nullable<Varchar>,
        orderno -> Int4,
        #[max_length = 256]
        remark -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    sysdictdata (id) {
        id -> Int8,
        dicttypeid -> Int8,
        #[max_length = 128]
        value -> Varchar,
        #[max_length = 64]
        code -> Varchar,
        orderno -> Int4,
        #[max_length = 128]
        remark -> Nullable<Varchar>,
        status -> Int4,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    sysdicttype (id) {
        id -> Int8,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 64]
        code -> Varchar,
        orderno -> Int4,
        #[max_length = 256]
        remark -> Nullable<Varchar>,
        status -> Int4,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    sysfile (id) {
        id -> Int8,
        #[max_length = 128]
        provider -> Nullable<Varchar>,
        #[max_length = 128]
        bucketname -> Nullable<Varchar>,
        #[max_length = 128]
        filename -> Nullable<Varchar>,
        #[max_length = 16]
        suffix -> Nullable<Varchar>,
        #[max_length = 128]
        filepath -> Nullable<Varchar>,
        #[max_length = 16]
        sizekb -> Nullable<Varchar>,
        #[max_length = 64]
        sizeinfo -> Nullable<Varchar>,
        #[max_length = 512]
        url -> Nullable<Varchar>,
        #[max_length = 128]
        filemd5 -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    sysjobcluster (id) {
        id -> Int8,
        #[max_length = 64]
        clusterid -> Varchar,
        #[max_length = 128]
        description -> Nullable<Varchar>,
        status -> Int4,
        updatedtime -> Nullable<Timestamp>,
    }
}

diesel::table! {
    sysjobdetail (id) {
        id -> Int8,
        #[max_length = 64]
        jobid -> Varchar,
        #[max_length = 128]
        groupname -> Nullable<Varchar>,
        #[max_length = 128]
        jobtype -> Nullable<Varchar>,
        #[max_length = 128]
        assemblyname -> Nullable<Varchar>,
        #[max_length = 128]
        description -> Nullable<Varchar>,
        concurrent -> Bool,
        annotation -> Bool,
        properties -> Nullable<Text>,
        updatedtime -> Nullable<Timestamp>,
        createtype -> Int4,
        scriptcode -> Nullable<Text>,
    }
}

diesel::table! {
    sysjobtrigger (id) {
        id -> Int8,
        #[max_length = 64]
        triggerid -> Varchar,
        #[max_length = 64]
        jobid -> Varchar,
        #[max_length = 128]
        triggertype -> Nullable<Varchar>,
        #[max_length = 128]
        assemblyname -> Nullable<Varchar>,
        #[max_length = 128]
        args -> Nullable<Varchar>,
        #[max_length = 128]
        description -> Nullable<Varchar>,
        status -> Int4,
        starttime -> Nullable<Timestamp>,
        endtime -> Nullable<Timestamp>,
        lastruntime -> Nullable<Timestamp>,
        nextruntime -> Nullable<Timestamp>,
        numberofruns -> Int8,
        maxnumberofruns -> Int8,
        numberoferrors -> Int8,
        maxnumberoferrors -> Int8,
        numretries -> Int4,
        retrytimeout -> Int4,
        startnow -> Bool,
        runonstart -> Bool,
        resetonlyonce -> Bool,
        updatedtime -> Nullable<Timestamp>,
    }
}

diesel::table! {
    syslogaudit (id) {
        id -> Int8,
        #[max_length = 64]
        tablename -> Varchar,
        #[max_length = 64]
        columnname -> Varchar,
        newvalue -> Nullable<Text>,
        oldvalue -> Nullable<Text>,
        operate -> Int4,
        audittime -> Nullable<Timestamp>,
        #[max_length = 32]
        account -> Nullable<Varchar>,
        #[max_length = 32]
        realname -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    syslogdiff (id) {
        id -> Int8,
        beforedata -> Nullable<Text>,
        afterdata -> Nullable<Text>,
        sql -> Nullable<Text>,
        parameters -> Nullable<Text>,
        businessdata -> Nullable<Text>,
        difftype -> Nullable<Text>,
        elapsed -> Nullable<Int8>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    syslogex (id) {
        id -> Int8,
        #[max_length = 32]
        httpmethod -> Nullable<Varchar>,
        requesturl -> Nullable<Text>,
        requestparam -> Nullable<Text>,
        returnresult -> Nullable<Text>,
        eventid -> Nullable<Int4>,
        threadid -> Nullable<Int4>,
        #[max_length = 128]
        traceid -> Nullable<Varchar>,
        exception -> Nullable<Text>,
        message -> Nullable<Text>,
        #[max_length = 256]
        controllername -> Nullable<Varchar>,
        #[max_length = 256]
        actionname -> Nullable<Varchar>,
        #[max_length = 256]
        displaytitle -> Nullable<Varchar>,
        #[max_length = 32]
        status -> Nullable<Varchar>,
        #[max_length = 256]
        remoteip -> Nullable<Varchar>,
        #[max_length = 128]
        location -> Nullable<Varchar>,
        longitude -> Nullable<Float8>,
        latitude -> Nullable<Float8>,
        #[max_length = 1024]
        browser -> Nullable<Varchar>,
        #[max_length = 256]
        os -> Nullable<Varchar>,
        elapsed -> Nullable<Int8>,
        logdatetime -> Nullable<Timestamp>,
        loglevel -> Nullable<Int4>,
        #[max_length = 32]
        account -> Nullable<Varchar>,
        #[max_length = 32]
        realname -> Nullable<Varchar>,
        tenantid -> Nullable<Int8>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    syslogop (id) {
        id -> Int8,
        #[max_length = 32]
        httpmethod -> Nullable<Varchar>,
        requesturl -> Nullable<Text>,
        requestparam -> Nullable<Text>,
        returnresult -> Nullable<Text>,
        eventid -> Nullable<Int4>,
        threadid -> Nullable<Int4>,
        #[max_length = 128]
        traceid -> Nullable<Varchar>,
        exception -> Nullable<Text>,
        message -> Nullable<Text>,
        #[max_length = 256]
        controllername -> Nullable<Varchar>,
        #[max_length = 256]
        actionname -> Nullable<Varchar>,
        #[max_length = 256]
        displaytitle -> Nullable<Varchar>,
        #[max_length = 32]
        status -> Nullable<Varchar>,
        #[max_length = 256]
        remoteip -> Nullable<Varchar>,
        #[max_length = 128]
        location -> Nullable<Varchar>,
        longitude -> Nullable<Float8>,
        latitude -> Nullable<Float8>,
        #[max_length = 1024]
        browser -> Nullable<Varchar>,
        #[max_length = 256]
        os -> Nullable<Varchar>,
        elapsed -> Nullable<Int8>,
        logdatetime -> Nullable<Timestamp>,
        loglevel -> Nullable<Int4>,
        #[max_length = 32]
        account -> Nullable<Varchar>,
        #[max_length = 32]
        realname -> Nullable<Varchar>,
        tenantid -> Nullable<Int8>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    syslogvis (id) {
        id -> Int8,
        #[max_length = 256]
        controllername -> Nullable<Varchar>,
        #[max_length = 256]
        actionname -> Nullable<Varchar>,
        #[max_length = 256]
        displaytitle -> Nullable<Varchar>,
        #[max_length = 32]
        status -> Nullable<Varchar>,
        #[max_length = 256]
        remoteip -> Nullable<Varchar>,
        #[max_length = 128]
        location -> Nullable<Varchar>,
        longitude -> Nullable<Float8>,
        latitude -> Nullable<Float8>,
        #[max_length = 1024]
        browser -> Nullable<Varchar>,
        #[max_length = 256]
        os -> Nullable<Varchar>,
        elapsed -> Nullable<Int8>,
        logdatetime -> Nullable<Timestamp>,
        loglevel -> Nullable<Int4>,
        #[max_length = 32]
        account -> Nullable<Varchar>,
        #[max_length = 32]
        realname -> Nullable<Varchar>,
        tenantid -> Nullable<Int8>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    sysmenu (id) {
        id -> Int8,
        pid -> Int8,
        #[sql_name = "type"]
        type_ -> Int4,
        #[max_length = 64]
        name -> Nullable<Varchar>,
        #[max_length = 128]
        path -> Nullable<Varchar>,
        #[max_length = 128]
        component -> Nullable<Varchar>,
        #[max_length = 128]
        redirect -> Nullable<Varchar>,
        #[max_length = 128]
        permission -> Nullable<Varchar>,
        #[max_length = 64]
        title -> Varchar,
        #[max_length = 128]
        icon -> Nullable<Varchar>,
        isiframe -> Bool,
        #[max_length = 256]
        outlink -> Nullable<Varchar>,
        ishide -> Bool,
        iskeepalive -> Bool,
        isaffix -> Bool,
        orderno -> Int4,
        status -> Int4,
        #[max_length = 256]
        remark -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    sysnotice (id) {
        id -> Int8,
        #[max_length = 32]
        title -> Varchar,
        content -> Text,
        #[sql_name = "type"]
        type_ -> Int4,
        publicuserid -> Int8,
        #[max_length = 32]
        publicusername -> Nullable<Varchar>,
        publicorgid -> Int8,
        #[max_length = 64]
        publicorgname -> Nullable<Varchar>,
        publictime -> Nullable<Timestamp>,
        canceltime -> Nullable<Timestamp>,
        status -> Int4,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    sysnoticeuser (noticeid) {
        noticeid -> Int8,
        userid -> Int8,
        readtime -> Nullable<Timestamp>,
        readstatus -> Int4,
    }
}

diesel::table! {
    sysonlineuser (id) {
        id -> Int8,
        #[max_length = 255]
        connectionid -> Nullable<Varchar>,
        userid -> Int8,
        #[max_length = 32]
        username -> Varchar,
        #[max_length = 32]
        realname -> Nullable<Varchar>,
        time -> Nullable<Timestamp>,
        #[max_length = 256]
        ip -> Nullable<Varchar>,
        #[max_length = 128]
        browser -> Nullable<Varchar>,
        #[max_length = 128]
        os -> Nullable<Varchar>,
        tenantid -> Nullable<Int8>,
    }
}

diesel::table! {
    sysorg (id) {
        id -> Int8,
        pid -> Int8,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 64]
        code -> Nullable<Varchar>,
        level -> Nullable<Int4>,
        #[max_length = 64]
        orgtype -> Nullable<Varchar>,
        orderno -> Int4,
        #[max_length = 128]
        remark -> Nullable<Varchar>,
        status -> Int4,
        tenantid -> Nullable<Int8>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    sysplugin (id) {
        id -> Int8,
        #[max_length = 64]
        name -> Varchar,
        csharpcode -> Text,
        #[max_length = 512]
        assemblyname -> Nullable<Varchar>,
        orderno -> Int4,
        status -> Int4,
        #[max_length = 128]
        remark -> Nullable<Varchar>,
        tenantid -> Nullable<Int8>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    syspos (id) {
        id -> Int8,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 64]
        code -> Nullable<Varchar>,
        orderno -> Int4,
        #[max_length = 128]
        remark -> Nullable<Varchar>,
        status -> Int4,
        tenantid -> Nullable<Int8>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    sysprint (id) {
        id -> Int8,
        #[max_length = 64]
        name -> Varchar,
        template -> Text,
        orderno -> Int4,
        status -> Int4,
        #[max_length = 128]
        remark -> Nullable<Varchar>,
        tenantid -> Nullable<Int8>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    sysprojectmanager (id) {
        id -> Int8,
        #[max_length = 200]
        projectname -> Varchar,
        status -> Int4,
        orgid -> Nullable<Int8>,
        #[max_length = 200]
        address -> Nullable<Varchar>,
        #[max_length = 100]
        contacts -> Nullable<Varchar>,
        #[max_length = 50]
        phone -> Nullable<Varchar>,
        #[max_length = 500]
        note -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    sysregion (id) {
        id -> Int8,
        pid -> Int8,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 32]
        shortname -> Nullable<Varchar>,
        #[max_length = 64]
        mergername -> Nullable<Varchar>,
        #[max_length = 32]
        code -> Nullable<Varchar>,
        #[max_length = 6]
        zipcode -> Nullable<Varchar>,
        #[max_length = 6]
        citycode -> Nullable<Varchar>,
        level -> Int4,
        #[max_length = 128]
        pinyin -> Nullable<Varchar>,
        lng -> Float4,
        lat -> Float4,
        orderno -> Int4,
        #[max_length = 128]
        remark -> Nullable<Varchar>,
    }
}

diesel::table! {
    sysrole (id) {
        id -> Int8,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 64]
        code -> Nullable<Varchar>,
        orderno -> Int4,
        datascope -> Int4,
        #[max_length = 128]
        remark -> Nullable<Varchar>,
        status -> Int4,
        tenantid -> Nullable<Int8>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    sysrolemenu (id) {
        id -> Int8,
        roleid -> Int8,
        menuid -> Int8,
    }
}

diesel::table! {
    sysroleorg (id) {
        id -> Int8,
        roleid -> Int8,
        orgid -> Int8,
    }
}

diesel::table! {
    system_security_info (id) {
        id -> Int4,
        start_date -> Timestamp,
        end_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    systenant (id) {
        id -> Int8,
        userid -> Int8,
        orgid -> Int8,
        #[max_length = 128]
        host -> Nullable<Varchar>,
        tenanttype -> Int4,
        dbtype -> Int4,
        #[max_length = 256]
        connection -> Nullable<Varchar>,
        #[max_length = 64]
        configid -> Nullable<Varchar>,
        orderno -> Int4,
        #[max_length = 128]
        remark -> Nullable<Varchar>,
        status -> Int4,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    sysuser (id) {
        id -> Int8,
        #[max_length = 32]
        account -> Varchar,
        #[max_length = 512]
        password -> Varchar,
        #[max_length = 32]
        realname -> Varchar,
        #[max_length = 32]
        nickname -> Nullable<Varchar>,
        #[max_length = 512]
        avatar -> Nullable<Varchar>,
        sex -> Int4,
        age -> Int4,
        birthday -> Nullable<Timestamp>,
        #[max_length = 32]
        nation -> Nullable<Varchar>,
        #[max_length = 16]
        phone -> Nullable<Varchar>,
        cardtype -> Int4,
        #[max_length = 32]
        idcardnum -> Nullable<Varchar>,
        #[max_length = 64]
        email -> Nullable<Varchar>,
        #[max_length = 256]
        address -> Nullable<Varchar>,
        culturelevel -> Int4,
        #[max_length = 16]
        politicaloutlook -> Nullable<Varchar>,
        #[max_length = 128]
        college -> Nullable<Varchar>,
        #[max_length = 16]
        officephone -> Nullable<Varchar>,
        #[max_length = 32]
        emergencycontact -> Nullable<Varchar>,
        #[max_length = 16]
        emergencyphone -> Nullable<Varchar>,
        #[max_length = 256]
        emergencyaddress -> Nullable<Varchar>,
        #[max_length = 512]
        introduction -> Nullable<Varchar>,
        orderno -> Int4,
        status -> Int4,
        #[max_length = 256]
        remark -> Nullable<Varchar>,
        accounttype -> Int4,
        orgid -> Int8,
        posid -> Int8,
        #[max_length = 32]
        jobnum -> Nullable<Varchar>,
        #[max_length = 32]
        poslevel -> Nullable<Varchar>,
        #[max_length = 32]
        postitle -> Nullable<Varchar>,
        #[max_length = 32]
        expertise -> Nullable<Varchar>,
        #[max_length = 32]
        officezone -> Nullable<Varchar>,
        #[max_length = 32]
        office -> Nullable<Varchar>,
        joindate -> Nullable<Timestamp>,
        #[max_length = 256]
        lastloginip -> Nullable<Varchar>,
        #[max_length = 128]
        lastloginaddress -> Nullable<Varchar>,
        lastlogintime -> Nullable<Timestamp>,
        #[max_length = 128]
        lastlogindevice -> Nullable<Varchar>,
        #[max_length = 512]
        signature -> Nullable<Varchar>,
        tenantid -> Nullable<Int8>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    sysuserextorg (id) {
        id -> Int8,
        userid -> Int8,
        orgid -> Int8,
        posid -> Int8,
        #[max_length = 32]
        jobnum -> Nullable<Varchar>,
        #[max_length = 32]
        poslevel -> Nullable<Varchar>,
        joindate -> Nullable<Timestamp>,
    }
}

diesel::table! {
    sysuserrole (id) {
        id -> Int8,
        userid -> Int8,
        roleid -> Int8,
    }
}

diesel::table! {
    syswechatpay (id) {
        id -> Int8,
        #[max_length = 255]
        merchantid -> Varchar,
        #[max_length = 255]
        appid -> Varchar,
        #[max_length = 255]
        outtradenumber -> Varchar,
        #[max_length = 255]
        transactionid -> Varchar,
        #[max_length = 255]
        tradetype -> Nullable<Varchar>,
        #[max_length = 255]
        tradestate -> Nullable<Varchar>,
        #[max_length = 255]
        tradestatedescription -> Nullable<Varchar>,
        #[max_length = 255]
        banktype -> Nullable<Varchar>,
        total -> Int4,
        payertotal -> Nullable<Int4>,
        successtime -> Nullable<Timestamp>,
        expiretime -> Nullable<Timestamp>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 255]
        scene -> Nullable<Varchar>,
        #[max_length = 255]
        attachment -> Nullable<Varchar>,
        #[max_length = 255]
        goodstag -> Nullable<Varchar>,
        #[max_length = 255]
        settlement -> Nullable<Varchar>,
        #[max_length = 255]
        notifyurl -> Nullable<Varchar>,
        #[max_length = 255]
        remark -> Nullable<Varchar>,
        #[max_length = 255]
        openid -> Nullable<Varchar>,
        #[max_length = 255]
        submerchantid -> Nullable<Varchar>,
        #[max_length = 255]
        subappid -> Nullable<Varchar>,
        #[max_length = 255]
        subopenid -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::table! {
    syswechatuser (id) {
        id -> Int8,
        userid -> Int8,
        platformtype -> Int4,
        #[max_length = 64]
        openid -> Varchar,
        #[max_length = 256]
        sessionkey -> Nullable<Varchar>,
        #[max_length = 64]
        unionid -> Nullable<Varchar>,
        #[max_length = 64]
        nickname -> Nullable<Varchar>,
        #[max_length = 256]
        avatar -> Nullable<Varchar>,
        #[max_length = 16]
        mobile -> Nullable<Varchar>,
        sex -> Nullable<Int4>,
        #[max_length = 64]
        language -> Nullable<Varchar>,
        #[max_length = 64]
        city -> Nullable<Varchar>,
        #[max_length = 64]
        province -> Nullable<Varchar>,
        #[max_length = 64]
        country -> Nullable<Varchar>,
        accesstoken -> Nullable<Text>,
        refreshtoken -> Nullable<Text>,
        expiresin -> Nullable<Int4>,
        #[max_length = 64]
        scope -> Nullable<Varchar>,
        createtime -> Nullable<Timestamp>,
        updatetime -> Nullable<Timestamp>,
        createuserid -> Nullable<Int8>,
        updateuserid -> Nullable<Int8>,
        isdelete -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    busbasicinfo,
    busbroadcastmanage,
    buscmanagefzjc,
    buscmanagehy,
    buscmanagekp,
    buscmanageyl,
    busdevtypemanager,
    busemergencyplan,
    busfaceregmqttcommandhistory,
    busfaceregmqttresponse,
    busfactory,
    busfirealarm,
    busfireanalysis,
    busfirewarning,
    busguardperson,
    busguardrecord,
    bushazardmanage,
    bushikevent,
    busjrxdata,
    busleavemanage,
    busmasbasestation,
    busminecavemanager,
    busminepeoplemanager,
    busmonitordata_20250201,
    busmonitormanager,
    buspatroldetail,
    busplanmanage,
    busrepair,
    busrescueforces,
    bussafetyroute,
    bussafetysupplies,
    busvehiclemanage,
    busvisitormanage,
    buswaterpatrol,
    buswatersupplies,
    busworkforcemanage,
    monitordataalarm_20250201,
    msgsendnotice,
    msgtemplate,
    posts,
    syscodegen,
    syscodegenconfig,
    sysconfig,
    sysdictdata,
    sysdicttype,
    sysfile,
    sysjobcluster,
    sysjobdetail,
    sysjobtrigger,
    syslogaudit,
    syslogdiff,
    syslogex,
    syslogop,
    syslogvis,
    sysmenu,
    sysnotice,
    sysnoticeuser,
    sysonlineuser,
    sysorg,
    sysplugin,
    syspos,
    sysprint,
    sysprojectmanager,
    sysregion,
    sysrole,
    sysrolemenu,
    sysroleorg,
    system_security_info,
    systenant,
    sysuser,
    sysuserextorg,
    sysuserrole,
    syswechatpay,
    syswechatuser,
);
