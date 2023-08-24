macro_rules! declare_error {
  ($err:ident, $msg:literal $(,)?) => {
    #[derive(Clone, Copy, Debug, derive_more::Display)]
    #[display = $msg]
    pub struct $err;

    impl ::error_stack::Context for $err {}
  };
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct StringError(String);

impl<T: std::fmt::Debug> From<T> for StringError {
  fn from(value: T) -> Self {
    Self(format!("{value:#?}"))
  }
}
