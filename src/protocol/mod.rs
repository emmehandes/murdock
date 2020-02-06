/** DNS protocol module
collections of structures describing the protocol
as well as utilities **/
mod buffer;
mod header;
mod resultcode;
mod querytype;
mod question;
mod record;
mod packet;

pub use self::buffer::Buffer;
pub use self::header::Header;
pub use self::resultcode::ResultCode;
pub use self::querytype::QueryType;
pub use self::question::Question;
pub use self::record::Record;
pub use self::packet::Packet;
