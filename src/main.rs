mod models;
mod errors;
mod etl;

use etl::extract::CsvExtractor;
use etl::transform::UserTransformer;
use etl::load::SqlLiteLoader;
use etl::Pipeline;
use errors::AppResult;


fn main()->AppResult<()> {
    println!("Starting ETL...");

    let extractor = CsvExtractor::from_path("data.csv")?;
    let transformer = UserTransformer::new();
    let loader = SqlLiteLoader::new("users.db")?;

    let mut pipeline = Pipeline::new(extractor, transformer, loader);
    pipeline.run();
    println!("Done");
    Ok(())
}
