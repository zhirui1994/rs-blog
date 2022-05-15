use tokio_postgres::Client;

pub async fn create(client: Client form: &form::CreateCategory) -> Result<CategoryID> {
  let n = super::count(
    client,
    "SELECT COUNT(*) FROM categories WHERE name=$1",
    &[&form.name]],
  ).await?;

  
}