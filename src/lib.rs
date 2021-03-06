use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub enum AuthError {
	EmailExists,
	UsernameExists,
	BadUsername,
	BadPassword,
	InvalidToken,
	Other,
}

impl std::error::Error for AuthError {}
impl fmt::Display for AuthError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Debug)]
pub enum ApiError {
	BadToken,
	Unauthorized,
	Unauthenticated,
	Other,
}

impl std::error::Error for ApiError {}
impl fmt::Display for ApiError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Debug)]
pub enum ServerError {
	///http-actix errors
	Other,
}

impl std::error::Error for ServerError {}
impl fmt::Display for ServerError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Debug)]
pub enum BackendError {
	///models errros
	Other,
}

impl std::error::Error for BackendError {}
impl fmt::Display for BackendError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Debug)]
pub enum GitError {
	Other,
}

impl std::error::Error for GitError {}
impl fmt::Display for GitError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
