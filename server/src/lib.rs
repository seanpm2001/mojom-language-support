extern crate lsp_types;
extern crate serde;
extern crate serde_json;

mod definition;
mod diagnostic;
mod import;
mod initialization;
mod messagesender;
mod mojomast;
mod protocol;
mod server;

pub use server::start;
