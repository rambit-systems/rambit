mod dashboard;
mod homepage;
mod login;
mod logout;
mod protected;
mod signup;
mod unauthorized;

pub use self::{
  dashboard::*, homepage::*, login::*, logout::*, protected::*, signup::*,
  unauthorized::*,
};
