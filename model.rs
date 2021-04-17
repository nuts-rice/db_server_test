#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::id_maps;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "id_maps"]
pub struct Map {
    pub id: i32,
    pub key: String,
    pub value: i32,
}
#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "id_maps"]
pub struct NewMap {
    pub key: String,
    pub value: i32,
}
