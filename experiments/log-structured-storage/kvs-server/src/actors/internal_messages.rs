use actix::prelude::*;
use actix::Message;
use tokio::net::TcpStream;

// tokio::net::TcpStream doesn't allow Clone, so we can't use it in an actix message.
// I think that we need to follow https://ryhl.io/blog/actors-with-tokio/ to implement
// actors directly through tokio. We can see this being done in https://github.com/Darksonn/telnet-chat

// #[derive(Debug, Clone, Message)]
// #[rtype(result = "()")]
// pub struct NewConnection {
//     conn: TcpStream,
// }
