/// Generic type for erros in tests where we don't care about the underlying error kind
pub type TestResult<T, E = Box<dyn std::error::Error>> = Result<T, E>;
