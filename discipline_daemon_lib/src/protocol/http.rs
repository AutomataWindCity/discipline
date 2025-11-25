use tokio::net::TcpStream;
use llhttp_rs::{Parser};

pub struct ParserState {

}

pub async fn x(
  buffer: &[u8],
  stream: &mut TcpStream,
) {
  let mut settings = 
  let mut parser = Parser::default();
  parser.init(settings, lltype);
}