use sqlx::pool;
use sqlx::postgres::any::AnyConnectionBackend;
use sqlx::prelude::FromRow;
use sqlx::Connection;
use sqlx::PgPool;
use sqlx::Row;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let url: &str = "postgres://dbuser:dbpass@postgres-host:5432/mydb";
    let url: &str = "postgres://postgres:postgres@postgres-host:5432/mydb";
    let mut conn = sqlx::postgres::PgConnection::connect(url).await?;

    let res = sqlx::query("SELECT 1 + 1 as suma")
        .fetch_one(&mut conn)
        .await?;

    let suma: i32 = res.get("suma");
    println!("1+1 en database = {}", suma);

    // Mejor usar el pool
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    let res_pool = sqlx::query("SELECT 1 + 1 as suma").fetch_one(&pool).await?;

    let suma_pool: i32 = res_pool.get("suma");
    println!("1+1 en database = {}", suma_pool);

    //sqlx::migrate!("./migrations").run(&pool).await?;
    let book = Book {
        title: "libro 1".to_string(),
        author: "daviaaad".to_string(),
        isbn: "aaa".to_string(),
    };
    //create(&book, &pool).await?;
    //update(&book,&book.isbn, &pool).await?;

    let libro_res: Result<Option<Book>, Box<dyn Error>> = read_optional(&pool).await;
    let libro: Option<Book> = libro_res?;
    println!("{:?}", libro);

    println!();

    let libros: Vec<Book> = read_all(&pool).await?;
    println!("{:?}", libros);

    Ok(())
}

#[derive(Debug, FromRow)]
struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn create(book: &Book, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let query: &str = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

async fn update(book: &Book, isbn: &str, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    let query: &str = "UPDATE book SET title = '%1', author = '$2' WHERE isbn = '$3'";
    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&isbn)
        .execute(pool)
        .await?;

    Ok(())
}

async fn read(conn: &sqlx::PgPool) -> Result<Book, Box<dyn Error>> {
    let q: &str = "Select title, author, isbn FROM book WHERE isbn = 'aaa'";
    let query = sqlx::query(q);

    let row = query.fetch_one(conn).await?;
    let book: Book = Book {
        title: row.get("title"),
        author: row.get("author"),
        isbn: row.get("isbn"),
    };
    Ok(book)
}

async fn read_optional(conn: &PgPool) -> Result<Option<Book>, Box<dyn Error>> {
    let q: &str = "Select title, author, isbn FROM book WHERE isbn = 'asfd'";
    let query = sqlx::query(q);

    let maybe_row = query.fetch_optional(conn).await?;
    let book: Option<Book> = maybe_row.map(|row| Book {
        title: row.get("title"),
        author: row.get("author"),
        isbn: row.get("isbn"),
    });
    Ok(book)
}

async fn read_all(conn: &PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let q: &str = "Select title, author, isbn FROM book";
    let query = sqlx::query(q);

    let rows = query.fetch_all(conn).await?;
    let books: Vec<Book> = rows
        .iter()
        .map(|row| Book {
            title: row.get("title"),
            author: row.get("author"),
            isbn: row.get("isbn"),
        })
        .collect();
    Ok(books)
}

async fn read_all_from_row(conn: &PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let q: &str = "Select title, author, isbn FROM book";

    let query = sqlx::query_as::<_, Book>(q);

    let books: Vec<Book> = query.fetch_all(conn).await?;
    Ok(books)
}

async fn insert_book(book: Book, conn: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    //transacción
    let mut txn = conn.begin().await?;

    let autor_q: &str = r"
        INSERT INTO author(name) VALUES ($1) RETURNING id
    ";
    let book_q: &str = r"
        INSERT INTO book(title, author, isbn) 
        VALUES ($1, $2, $3)
    ";
    let autor_id = sqlx::query_as(autor_q)
        .bind(&book.author)
        .fetch_one(&mut *txn)
        .await?;

    sqlx::query(book_q)
    .bind(&book.title)
    .bind(&book.author)
    .bind(&book.isbn)
    .execute(&mut *txn)
    .await?;

    txn.commit().await?;
    Ok(())
}
