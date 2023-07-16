use axum::extract::State;
use axum::{extract, response};
use futures::StreamExt;

use crate::database::Db;
use mongodb::bson::{self, Document};

use crate::model::user::*;
