
use crate::database::{ JsonDatabase, DatabaseEntry, BudgetDatabase, AdvancedQuery,
    DatabaseError};
use crate::budget::{BudgetEntry};
use serde::{Deserialize};
use std::fmt::{Display, Formatter};
use actix_web::{web, Responder, http, ResponseError, HttpResponse, error};
use actix_web::body::{BoxBody};
use anyhow::{anyhow};

#[derive(Debug)]
pub struct CustomError {
    status_code: u16,
    err: anyhow::Error
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.err).unwrap();
        Ok(())
    }
}

impl ResponseError for CustomError {
    fn status_code(&self) -> http::StatusCode {
        http::StatusCode::from_u16(self.status_code).unwrap()
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        //HttpResponse::<BoxBody>::new(http::StatusCode::from_u16(500).unwrap())
        let res = HttpResponse::<String>::with_body(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            self.err.to_string()
            );

        res.map_into_boxed_body()
    }
}


pub async fn root_handler() -> impl Responder {
    format!("Hello {}", 1)
}


#[derive(Deserialize, Clone)]
pub struct IdQuery {
    id: String,
}

pub async fn get_by_id_handler(query: web::Json<IdQuery>, db: web::Data<JsonDatabase>)
    -> Result<web::Json<DatabaseEntry>, CustomError> {
    let x = db.get_by_id(&query.id)
        .ok_or(CustomError{status_code: 500,
            err: anyhow!("no entry found with id")});
    Ok(web::Json(x?))
}

#[derive(Deserialize, Clone)]
pub struct TagsQuery {
    tags: Vec<String>,
}

pub async fn get_by_tags_handler(query: web::Json<TagsQuery>, db: web::Data<JsonDatabase>)
    -> Result<web::Json<Vec<DatabaseEntry>>, CustomError> {
    let x = db.get_by_tags(&query.tags);
    Ok(web::Json(x))
}

#[derive(Deserialize, Clone)]
pub struct MonthQuery {
    year: i32,
    month: u32,
}

pub async fn get_by_month_handler(query: web::Json<MonthQuery>, db: web::Data<JsonDatabase>)
    -> Result<web::Json<Vec<DatabaseEntry>>, CustomError> {

    Ok(web::Json(db.get_by_month(query.year, query.month)))
}


pub async fn get_by_advanced_handler(query: web::Json<AdvancedQuery>, db: web::Data<JsonDatabase>)
    -> Result<web::Json<Vec<DatabaseEntry>>, CustomError> {
    Ok(web::Json(db.get_by_advanced(query.into_inner())))
}

pub async fn add_entry_handler(query: web::Json<BudgetEntry>, db: web::Data<JsonDatabase>) 
    -> Result<web::Json<()>, DatabaseError> {
    db.add_entry(query.into_inner())?;
    Ok(web::Json(()))
}

#[derive(Deserialize)]
pub struct DeleteQuery {
    id: String
}

pub async fn delete_entry_handler(query: web::Json<DeleteQuery>, db: web::Data<JsonDatabase>)
    -> Result<web::Json<()>, DatabaseError> {
    db.delete_entry(&query.id)?;
    Ok(web::Json(()))
}

pub async fn update_entry_handler(query: web::Json<DatabaseEntry>, db: web::Data<JsonDatabase>)
    -> Result<web::Json<()>, DatabaseError> {
    Ok(web::Json(db.update_entry(query.into_inner())?))
    //Ok(web::Json((db.update_entry(query.into_inner()))?))
}
