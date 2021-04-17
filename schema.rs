table! {
    id_maps (key) {
        id -> Int4,
        key -> Varchar,
        value -> Int4,
    }
}

table! {
    id_maps_psql (id) {
        id -> Int4,
        key -> Varchar,
        value -> Int4,
    }
}

table! {
    maps (key) {
        key -> Varchar,
        value -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    id_maps,
    id_maps_psql,
    maps,
);
