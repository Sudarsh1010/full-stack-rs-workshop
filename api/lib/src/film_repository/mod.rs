use uuid::Uuid;

use crate::models::{CreateFilm, Film};

pub type FilmError = String;
pub type FilmResult<T> = Result<T, FilmError>;

#[async_trait]
pub trait FilmRepository: Send + Sync + 'static {
    async fn get_films(&self) -> FilmResult<Vec<Film>>;
    async fn create_film(&self, body: &CreateFilm) -> FilmResult<Film>;
    async fn get_film(&self, id: &Uuid) -> FilmResult<Film>;
    async fn update_film(&self, film: &Film) -> FilmResult<Film>;
    async fn delete_film(&self, id: &Uuid) -> FilmResult<Uuid>;
}
