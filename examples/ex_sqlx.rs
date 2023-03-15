use sqlx::{postgres::{PgPool, PgPoolOptions}, Row};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
  let pool = PgPoolOptions::new()
      .max_connections(5)
      .connect("postgresql://postgres:1234@localhost:5432/postgres")
      .await?;
  println!("pool {:?}", pool);
  
  let email = "jackpoon222@gmail.com";
  // Execute a SQL query against the database using SQLx
  let row = sqlx::query(&format!("SELECT id FROM users WHERE email = '{email}'"))
    .fetch_one(&pool)
    .await?;
  let id: i32 = row.get("id");
  let name: String = row.get("name");
  println!("User: {} - {}", id, name);

  Ok(())
}