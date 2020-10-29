table! {
    homicides (record_id) {
        record_id -> Int4,
        agency_code -> Nullable<Varchar>,
        agency_name -> Nullable<Varchar>,
        agency_type -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        state -> Nullable<Varchar>,
        year -> Nullable<Varchar>,
        month -> Nullable<Varchar>,
        incident -> Nullable<Varchar>,
        crime_type -> Nullable<Varchar>,
        crime_solved -> Nullable<Varchar>,
        victim_sex -> Nullable<Varchar>,
        victim_age -> Nullable<Varchar>,
        victim_race -> Nullable<Varchar>,
        victim_ethnicity -> Nullable<Varchar>,
        perpetrator_sex -> Nullable<Varchar>,
        perpetrator_age -> Nullable<Varchar>,
        perpetrator_race -> Nullable<Varchar>,
        perpetrator_ethnicity -> Nullable<Varchar>,
        relationship -> Nullable<Varchar>,
        weapon -> Nullable<Varchar>,
        victim_count -> Nullable<Varchar>,
        perpetrator_count -> Nullable<Varchar>,
        record_source -> Nullable<Varchar>,
    }
}