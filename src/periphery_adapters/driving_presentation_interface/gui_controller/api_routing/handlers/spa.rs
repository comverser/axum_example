use rand::Rng;
use serde::Deserialize;
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use askama_axum::Template;
use axum::{
    extract::{Path, State},
    Form,
};

use crate::application_core::domain_layer::models::article::Article;

#[derive(Deserialize)]
pub struct UploadArticleRequest {
    pub name: String,
    pub body: String,
}

#[derive(Template)]
#[template(path = "spa_index.html")]
pub struct CreateHomepageTemplate {
    title: String,
    articles: Vec<Article>,
}

#[derive(Template)]
#[template(path = "partials/spa_list.html")]
pub struct CreateArticleListTemplate {
    articles: Vec<Article>,
}

#[derive(Template)]
#[template(path = "partials/article.html")]
pub struct CreateArticleTemplate {
    title: String,
    article: Article,
}

pub async fn spa_index(
    State(spa_articles): State<Arc<Mutex<Vec<Article>>>>,
) -> CreateHomepageTemplate {
    let articles = spa_articles.lock().unwrap().clone();
    CreateHomepageTemplate {
        title: String::from("Product Listing"),
        articles,
    }
}

pub async fn get_article(
    State(spa_articles): State<Arc<Mutex<Vec<Article>>>>,
    Path(article_id): Path<u32>,
) -> CreateArticleTemplate {
    let articles = spa_articles.lock().unwrap().clone();

    let article = articles
        .iter()
        .find(|article| article.id == article_id)
        .unwrap()
        .clone();

    CreateArticleTemplate {
        title: article.name.clone(),
        article,
    }
}

pub async fn upload_article(
    State(spa_articles): State<Arc<Mutex<Vec<Article>>>>,
    Form(upload_article_request): Form<UploadArticleRequest>,
) -> CreateArticleListTemplate {
    let mut articles = spa_articles.lock().unwrap();

    let rand_id: u32 = rand::thread_rng().gen();
    let article = Article {
        id: rand_id,
        name: upload_article_request.name,
        body: upload_article_request.body,
    };

    sleep(Duration::from_secs(2));

    articles.push(article);

    CreateArticleListTemplate {
        articles: articles.clone(),
    }
}
