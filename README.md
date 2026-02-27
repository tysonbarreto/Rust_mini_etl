---

# **Mini ETL Pipeline in Rust (CSV → Transform → SQLite)**

This project implements a fully modular **Extract–Transform–Load (ETL)** pipeline in Rust. It reads user data from a CSV file, transforms and validates the records, and loads the cleaned data into a SQLite database.

The project is designed as a learning tool to demonstrate:

- Modular Rust architecture  
- Traits, generics, and lifetimes  
- Boxed trait objects  
- Iterator‑driven ETL  
- Error handling with `thiserror`  
- CSV parsing with `csv`  
- SQLite loading with `rusqlite`  
- Unit tests and integration tests  

---

## **Project Structure**

```
api_etl/
│
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── errors.rs
│   ├── models.rs
│   └── etl/
│       ├── mod.rs
│       ├── traits.rs
│       ├── extract.rs
│       ├── transform.rs
│       └── load.rs
└── tests/
    └── pipeline_test.rs
```

Each module has a single responsibility:

- **extract.rs** — Reads CSV rows into `RawUser`
- **transform.rs** — Cleans and validates data into `CleanUser`
- **load.rs** — Inserts cleaned data into SQLite
- **traits.rs** — Defines the generic ETL traits
- **mod.rs** — Assembles the pipeline
- **tests/** — Integration tests for the full pipeline

---

## **Data Flow**

```
CSV File
   │
   ▼
Extractor (CsvExtractor)
   │  produces RawUser
   ▼
Transformer (UserTransformer)
   │  produces CleanUser
   ▼
Loader (SqliteLoader)
   │  inserts into SQLite
   ▼
SQLite Database
```

---

## **Traits Overview**

The ETL pipeline is generic and extensible thanks to three core traits:

### **Extractor<T>**
Produces an iterator of `AppResult<T>`.

```rust
fn extract<'a>(&'a mut self)
    -> Box<dyn Iterator<Item = AppResult<T>> + 'a>;
```

### **Transformer<I, O>**
Maps input type `I` to output type `O`.

```rust
fn transform(&self, input: I) -> Option<O>;
```

### **Loader<T>**
Consumes cleaned data and persists it.

```rust
fn load(&mut self, item: &T) -> AppResult<()>;
```

---

## **Pipeline Structure**

The pipeline ties the three stages together using generics and `PhantomData`:

```rust
pub struct Pipeline<E, T, L, I, O>
where
    E: Extractor<I>,
    T: Transformer<I, O>,
    L: Loader<O>,
{
    extractor: E,
    transformer: T,
    loader: L,
    _marker: PhantomData<(I, O)>,
}
```

This ensures type‑safety and allows the pipeline to work with any compatible extractor, transformer, and loader.

---

## **Running the ETL Pipeline**

### 1. Create a CSV file:

`users.csv`:

```
name,age,country
alice,30,uk
bob,17,us
,25,de
charlie,200,fr
```

### 2. Run the project:

```bash
cargo run
```

### 3. Inspect the SQLite database:

```bash
sqlite3 users.db "SELECT * FROM users;"
```

Expected rows:

```
1|Alice|30|UK
2|Bob|17|US
```

Invalid rows are skipped during transformation.

---

## **Testing**

The project includes both **unit tests** and **integration tests**.

### **Unit Tests (inside module files)**

- `etl/transform.rs` — tests data cleaning logic  
- `etl/extract.rs` — tests CSV parsing  
- `etl/load.rs` — tests SQLite inserts  

Run all unit tests:

```bash
cargo test
```

### **Integration Test (tests/pipeline_test.rs)**

Runs the full ETL pipeline end‑to‑end:

- Creates a temporary CSV  
- Runs Extract → Transform → Load  
- Verifies SQLite contents  

Run integration tests:

```bash
cargo test --test pipeline_test
```
