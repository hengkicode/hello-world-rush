use anyhow::Error;
use odbc_api::{buffers::TextRowSet, ConnectionOptions, Environment, ResultSetMetadata, Cursor};
use std::collections::HashMap;
use std::env;

const BATCH_SIZE: usize = 5000;

pub async fn fetch_stock_data() -> Result<Vec<HashMap<String, String>>, Error> {
    dotenv::dotenv().ok();
    let connection_string = env::var("DATABASE_URL")
        .expect("DATABASE_URL harus diatur di file .env");
    let query = env::var("QUERY").unwrap_or("SELECT top 1000 * FROM stock".to_string());

    let environment = Environment::new()?;
    let connection = environment.connect_with_connection_string(
        &connection_string,
        ConnectionOptions::default(),
    )?;

    let mut results = Vec::new();
    if let Some(mut cursor) = connection.execute(&query, ())? {
        let column_names: Vec<String> = cursor.column_names()?
            .map(|name_result| name_result.map(|name| name.to_string()))
            .collect::<Result<Vec<String>, _>>()?;

        let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
        let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;

        while let Some(batch) = row_set_cursor.fetch()? {
            println!("Fetched {} rows", batch.num_rows());
            for row_index in 0..batch.num_rows() {
                let mut row_data = HashMap::new();
                for (col_index, col_name) in column_names.iter().enumerate() {
                    let col_value = String::from_utf8_lossy(batch.at(col_index, row_index).unwrap_or(&[])).to_string();
                    row_data.insert(col_name.clone(), col_value);
                }
                results.push(row_data);
            }
        }
    }

    println!("Total rows fetched: {}", results.len());
    Ok(results)
}