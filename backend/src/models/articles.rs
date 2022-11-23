use {
    super::from_model::FromModel,
    crate::{schema::articles, API_URL},
    serde::Serialize,
    diesel::{PgConnection, QueryDsl, RunQueryDsl},

};

// Struct for the model from SQL

#[derive(Identifiable, Debug, Serialize, Queryable, Clone, AsChangeset)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub pub_date: chrono::NaiveDateTime,
    pub published: bool,
    pub headline: String,
    pub image: String,
    pub content: String,
}


// Creating a struct corresponding to the actual respresentation of an article 
// that might contain additional fields

#[derive(Debug, Serialize)]
pub struct ArticleRepresentation {
    pub id: i32,
    pub title: String,
    pub pub_date: chrono::NaiveDateTime,
    pub published: bool,
    pub headline: String,
    pub image: String,
    pub content: String,
}

// Two Endpoints

// Creating model from the schema to allow for a picture url
impl FromModel<Article> for ArticleRepresentation {
    fn from_model(article: Article) -> ArticleRepresentation {
        ArticleRepresentation {
            id: article.id,
            title: article.title,
            pub_date: article.pub_date,
            published: article.published,
            headline: article.headline,
            image: API_URL.to_owned() + &article.image,
            content: article.content,
        }
    }
}

impl Article {
    // Grabs an article by id
    pub fn get (
        id: &i32,
        connection: &PgConnection,
    ) -> Result<ArticleRepresentation, diesel::result::Error> {
        let article = articles::table.find(id).first::<Article>(connection)?;
        Ok(ArticleRepresentation::from_model(article)) // converts the article to representation version
    }

    pub fn list(
        connection: &PgConnection,
    ) -> Result<Vec<ArticleRepresentation>, diesel::result::Error> {
        let articles = articles::table
            .load::<Article>(connection)
            .expect("Could not load articles");
            let results = articles
            .into_iter()
            .map(ArticleRepresentation::from_model)
            .collect();

        Ok(results)
    }
}