#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table="categories")]
pub struct Category {
  pub id:i32,
  pub name:String,
  pub is_del:bool,
}

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table="categories")]
pub struct CategoryID {
  pub id:i32,
}
