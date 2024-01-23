pub mod api;
pub use ticker::TickerSymbol;

mod sockets;
pub use sockets::handle_tcp_client;
