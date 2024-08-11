use rand::Rng;
use std::sync::{Arc, Mutex};

use askama_axum::Template;
use axum::{
    extract::{Path, State},
    Form,
};
use serde::Deserialize;

use crate::application_core::domain_layer::models::book::Book;

#[derive(Deserialize)]
pub struct RequestBook {
    title: String,
    name: String,
}

#[derive(Deserialize)]
pub struct SearchForm {
    search: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct CreateHomepageTemplate {}

#[derive(Template)]
#[template(path = "list.html")]
pub struct CreateListTemplate {
    books: Vec<Book>,
}

#[derive(Template)]
#[template(path = "components/book.html")]
pub struct CreateBookTemplate {
    id: String,
    title: String,
    author: String,
}

#[derive(Template)]
#[template(path = "edit.html")]
pub struct CreateEditFormTemplate {
    id: String,
    title: String,
    author: String,
}

pub async fn index() -> CreateHomepageTemplate {
    CreateHomepageTemplate {}
}

pub async fn get_books(State(my_reading_list): State<Arc<Mutex<Vec<Book>>>>) -> CreateListTemplate {
    let my_reading_list = my_reading_list.lock().unwrap();
    let books = my_reading_list.clone();
    CreateListTemplate { books: books }
}

pub async fn search_books(
    State(my_reading_list): State<Arc<Mutex<Vec<Book>>>>,
    Form(search_form): Form<SearchForm>,
) -> CreateListTemplate {
    let search_term = search_form.search.to_lowercase();

    let my_reading_list = my_reading_list.lock().unwrap();
    let searched_list = my_reading_list
        .iter()
        .filter(|book| book.title.to_lowercase().contains(search_term.as_str()))
        .cloned()
        .collect();
    CreateListTemplate {
        books: searched_list,
    }
}

pub async fn get_book(
    State(my_reading_list): State<Arc<Mutex<Vec<Book>>>>,
    Path(book_id): Path<i32>,
) -> CreateBookTemplate {
    let books = my_reading_list.lock().unwrap();
    let book = books
        .iter()
        .find(|book| book.id == book_id.to_string())
        .unwrap()
        .clone();

    CreateBookTemplate {
        id: book.id,
        title: book.title,
        author: book.author,
    }
}

pub async fn edit_book(
    State(my_reading_list): State<Arc<Mutex<Vec<Book>>>>,
    Path(book_id): Path<i32>,
) -> CreateEditFormTemplate {
    let books = my_reading_list.lock().unwrap();
    let book = books
        .iter()
        .find(|book| book.id == book_id.to_string())
        .unwrap();

    CreateEditFormTemplate {
        id: book.id.clone(),
        title: book.title.clone(),
        author: book.author.clone(),
    }
}

pub async fn update_book(
    State(my_reading_list): State<Arc<Mutex<Vec<Book>>>>,
    Path(book_id): Path<i32>,
    Form(request_book): Form<RequestBook>,
) -> CreateBookTemplate {
    let mut books = my_reading_list.lock().unwrap();
    let book = books
        .iter_mut()
        .find(|book| book.id == book_id.to_string())
        .unwrap();

    book.title = request_book.title;
    book.author = request_book.name;

    CreateBookTemplate {
        id: book.id.clone(),
        title: book.title.clone(),
        author: book.author.clone(),
    }
}

pub async fn delete_book(
    State(my_reading_list): State<Arc<Mutex<Vec<Book>>>>,
    Path(book_id): Path<i32>,
) {
    let book_id = book_id;
    let mut books = my_reading_list.lock().unwrap();
    books.retain(|book| book.id != book_id.to_string());
}

pub async fn add_book(
    State(my_reading_list): State<Arc<Mutex<Vec<Book>>>>,
    Form(request_book): Form<RequestBook>,
    // ) -> Redirect {
) -> CreateBookTemplate {
    let ran_id: u8 = rand::thread_rng().gen();

    my_reading_list.lock().unwrap().push(Book {
        id: ran_id.to_string(),
        title: request_book.title.clone(),
        author: request_book.name.clone(),
    });

    // Redirect::to(&format!("/books/{ran_id}"))

    CreateBookTemplate {
        id: ran_id.to_string(),
        title: request_book.title.clone(),
        author: request_book.name.clone(),
    }
}
