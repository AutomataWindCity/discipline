use std::os::unix::net::UnixStream;
use std::os::unix::io::AsRawFd;
use std::io::{self, Read, Write};
use crate::x::Duration;
use libc::{POLLIN, POLLOUT, poll, pollfd};

pub fn read_with_timeout(
  stream: &mut UnixStream,
  buffer: &mut [u8],
  timeout: Duration
) -> io::Result<usize> {
  
  
  let mut file_descriptors = [pollfd {
    fd: stream.as_raw_fd(),
    events: POLLIN,
    revents: 0,
  }];

  // TODO:
  let timeout = timeout.milliseconds() as i32;
  
  unsafe {
    let status = poll(
      file_descriptors.as_mut_ptr(), 
      1, 
      timeout,
    );

    match status {
      0 => {
        Err(io::Error::new(io::ErrorKind::TimedOut, "read timeout"))
      }
      -1 => {
        Err(io::Error::last_os_error())
      }
      _ => {
        stream.read(buffer)
      }
    }
  }
}

pub fn write_with_timeout(
  stream: &mut UnixStream,
  buffer: &[u8],
  timeout: Duration,
) -> Result<usize, io::Error> {
  let mut file_descriptors = [
    pollfd {
      fd: stream.as_raw_fd(),
      events: POLLOUT,
      revents: 0,
    },
  ];

  // TODO
  let timeout = timeout.milliseconds() as i32;

  unsafe {
    let status = poll(
      file_descriptors.as_mut_ptr(),
      1,
      timeout,
    );

    match status {
      0 => {
        Err(io::Error::new(io::ErrorKind::TimedOut, "read timeout"))
      }
      -1 => {
        Err(io::Error::last_os_error())
      }
      _ => {
        stream.write(buffer)
      }
    }
  }
}