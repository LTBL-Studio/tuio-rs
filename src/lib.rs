//! ## Available features
//! - `validate` : Have [`Client::refresh`] yield a `TuioError::IncompleteBundle` if any of the buffered TUIO bundles is not
//!   [valid](https://www.tuio.org/?tuio11#:~:text=A%20minimal%20TUIO%20bundle%20needs%20to%20contain%20at%20least%20the%20compulsory%20ALIVE%20and%20FSEQ%20messages%2E) ;
//!   i.e. it is missing one or both of the compulsory ALIVE and FSEQ messages.

mod object;
mod blob;
mod errors;
pub mod server;
pub mod client;
pub mod cursor;
pub mod osc_encode_decode;
pub mod osc_receiver;

pub use server::Server;
pub use client::Client;
pub use cursor::Cursor;
pub use object::Object;
pub use blob::Blob;