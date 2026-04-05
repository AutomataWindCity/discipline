use std::ops::{Index, IndexMut, RangeFrom, RangeTo, RangeFull, Range};

pub struct MessageLength(usize);

impl MessageLength {
  pub const BINARY_SIZE: usize = size_of::<u32>();
}

// Invariant: BufferSize.0 is greater than MessageSize::BINARY_SIZE.
pub struct BufferLength(usize);

impl BufferLength {
  pub const fn create_or_panic(value: usize) -> Self {
    let value = value + MessageLength::BINARY_SIZE;
    if value > usize::MAX {
      panic!("Length too large");
    }

    Self(value)
  }

  pub fn get(&self) -> usize {
    self.0
  }

  pub fn get_maximum_message_size(&self) -> MessageLength {
    MessageLength(self.0 - MessageLength::BINARY_SIZE)
  }
}


pub struct StreamBuffer {
  data: Vec<u8>,
}

impl StreamBuffer {
  pub fn buffer_length(&self) -> usize {
    todo!()
  }

  pub fn maximum_message_length(&self) -> usize {
    todo!()
  }
}

impl Index<Range<usize>> for StreamBuffer {
  type Output = [u8];
  
  fn index(&self, range: Range<usize>) -> &[u8] {
    &self.data[range]
  }
}

impl Index<RangeFrom<usize>> for StreamBuffer {
  type Output = [u8];
  
  fn index(&self, range: RangeFrom<usize>) -> &[u8] {
    &self.data[range]
  }
}

impl Index<RangeTo<usize>> for StreamBuffer {
  type Output = [u8];
    
  fn index(&self, range: RangeTo<usize>) -> &[u8] {
    &self.data[range]
  }
}

impl Index<RangeFull> for StreamBuffer {
  type Output = [u8];
  
  fn index(&self, _: RangeFull) -> &[u8] {
    &self.data[..]
  }
}

impl IndexMut<Range<usize>> for StreamBuffer {
  fn index_mut(&mut self, range: Range<usize>) -> &mut [u8] {
    &mut self.data[range]
  }
}

impl IndexMut<RangeFrom<usize>> for StreamBuffer {
  fn index_mut(&mut self, range: RangeFrom<usize>) -> &mut [u8] {
    &mut self.data[range]
  }
}

impl IndexMut<RangeTo<usize>> for StreamBuffer {
  fn index_mut(&mut self, range: RangeTo<usize>) -> &mut [u8] {
    &mut self.data[range]
  }
}

impl IndexMut<RangeFull> for StreamBuffer {
  fn index_mut(&mut self, _: RangeFull) -> &mut [u8] {
    &mut self.data[..]
  }
}