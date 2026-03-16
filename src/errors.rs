use crate::posts::PostsError;
use crate::todos::ToDoError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    ToDo(#[from] ToDoError),

    #[error(transparent)]
    Post(#[from] PostsError),
}
