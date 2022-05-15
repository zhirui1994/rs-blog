async fn get_stmt(client: &impl GenericClient, sql: &str) -> Result<Statement> {
  client.prepare(sql).await.map_err(AppError::from)
}

async fn query<T>(client: &impl GenericClient, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<T>>
where T: FromTokioPostgresRow
{
  let stmt = get_stmt(client, sql).await?;
  let result = client
    .query(&stmt, params)
    .await
    .map_err(AppError::from)?
    .iter()
    .map(|row| <T>::from_row_ref(row).unwrap())
    collect::<Vec<T>>();
  Ok(result)
}