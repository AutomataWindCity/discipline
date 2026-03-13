pub struct MessageSize(usize);

impl MessageSize {
  pub const BINARY_SIZE: usize = size_of::<u32>();
}

// Invariant: BufferSize.0 is greater than MessageSize::BINARY_SIZE.
pub struct BufferSize(usize);

impl BufferSize {
  pub fn get(&self) -> usize {
    self.0
  }

  pub fn get_maximum_message_size(&self) -> MessageSize {
    MessageSize(self.0 - MessageSize::BINARY_SIZE)
  }
}