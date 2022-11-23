use {
    crate::{errors::database_error::DatabaseError, models::articles::Article, Pool},
    actix_web::{web, Error, HttpResponse},
};

//* Handlers
//    Each one will have a corresponding function that will fetch data from the database.
//      - Comparable to a view in Django  



// Fetching the article by id
// /articles/{id}/
pub async fn get(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    Ok(web::block(move || Article::get(&id, &connection))
        .await
        .map(|article| HttpResponse::Ok().json(article))
        .map_err(DatabaseError)?)
}

// Fetch all articles
// /articles/
pub async fn list(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().unwrap();
    Ok(web::block(move || Article::list(&connection))
        .await
        .map(|articles| HttpResponse::Ok().json(articles))
        .map_err(DatabaseError)?)
}