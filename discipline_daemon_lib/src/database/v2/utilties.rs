#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Name(pub &'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Index(pub usize);

pub trait ScalarWrite {
  fn write(&self, destination: &mut impl ScalarWriteDestination);
}

pub trait ScalarWriteDestination {
  fn write_u8(&mut self, value: u8) {}
  fn write_u16(&mut self, value: u16) {}
  fn write_u32(&mut self, value: u32) {}
  fn write_u64(&mut self, value: u64) {}

  fn write_i8(&mut self, value: i8) {}
  fn write_i16(&mut self, value: i16) {}
  fn write_i32(&mut self, value: i32) {}
  fn write_i64(&mut self, value: i64) {}

  fn write_string(&mut self, value: &str) {}
}

pub trait OrderedWriteNull {
  fn ordered_write_null(destination: &mut impl OrderedWriteNullDestination);
}

pub trait OrderedWriteNullDestination {
  fn write_null(&mut self) {}
}

pub trait OrderedWrite {
  fn ordered_write(&self, destination: &mut impl OrderedWriteDestination);
}

pub trait OrderedWriteDestination {
  fn write_u8(&mut self, value: u8) {}
  fn write_u16(&mut self, value: u16) {}
  fn write_u32(&mut self, value: u32) {}
  fn write_u64(&mut self, value: u64) {}

  fn write_i8(&mut self, value: i8) {}
  fn write_i16(&mut self, value: i16) {}
  fn write_i32(&mut self, value: i32) {}
  fn write_i64(&mut self, value: i64) {}

  fn write_string(&mut self, value: &str) {}

  fn write_scalar<Scalar>(&mut self, value: &Scalar)
  where
    Scalar: ScalarWrite {}

  fn as_ordered_write_null_destination(&mut self) -> &mut impl OrderedWriteNullDestination;
}

pub trait NamedWriteNull: NamedWrite {
  fn named_write_null(names: &Self::Names, destination: &mut impl NamedWriteNullDestination);
}

pub trait NamedWriteNullDestination {
  fn write_null(&mut self, name: Name) {}
}

pub trait NamedWrite {
  type Names;

  fn named_write(&self, names: &Self::Names, destination: &mut impl NamedWriteDestination);
}

pub trait NamedWriteDestination {
  fn write_u8(&mut self, name: Name, value: u8) {}
  fn write_u16(&mut self, name: Name, value: u16) {}
  fn write_u32(&mut self, name: Name, value: u32) {}
  fn write_u64(&mut self, name: Name, value: u64) {}

  fn write_i8(&mut self, name: Name, value: i8) {}
  fn write_i16(&mut self, name: Name, value: i16) {}
  fn write_i32(&mut self, name: Name, value: i32) {}
  fn write_i64(&mut self, name: Name, value: i64) {}

  fn write_string(&mut self, name: Name, value: &str) {}

  fn write_scalar<Scalar>(&mut self, name: Name, value: &Scalar)
  where 
    Scalar: ScalarWrite {}

  fn write_compound<Compound>(&mut self, names: &Compound::Names, value: &Compound)
  where 
    Compound: NamedWrite {}

  fn write_optional() {}

  fn as_namef_write_null_destination(&mut self) -> &mut impl NamedWriteNullDestination;
}

pub trait ScalarIndexedRead {
  fn internal_indexed_read(source: &mut impl IndexedReadSource, index: Index) -> Result<Self, ()>;

  fn indexed_read(source: &mut impl IndexedReadSource) -> Result<Self, ()> {
    todo!()
  }
}

pub trait CompoundIndexedRead {
  type Indexes;

  fn internal_indexed_read(source: &mut impl IndexedReadSource, indexes: &Self::Indexes) -> Result<Self, ()>;

  fn indexed_read(source: &mut impl IndexedReadSource) -> Result<Self, ()> {
    todo!()
  }
}

pub trait IndexedReadSource {
  fn read_u8(&mut self, index: Index) -> Result<u8, ()> {
    todo!()
  }

  fn read_u16(&mut self, index: Index) -> Result<u16, ()> {
    todo!()
  }

  fn read_u32(&mut self, index: Index) -> Result<u32, ()> {
    todo!()
  }

  fn read_u64(&mut self, index: Index) -> Result<u64, ()> {
    todo!()
  }

  fn read_i8(&mut self, index: Index) -> Result<i8, ()> {
    todo!()
  }

  fn read_i16(&mut self, index: Index) -> Result<i16, ()> {
    todo!()
  }

  fn read_i32(&mut self, index: Index) -> Result<i32, ()> {
    todo!()
  }

  fn read_i64(&mut self, index: Index) -> Result<i64, ()> {
    todo!()
  }

  fn read_string(&mut self, index: Index) -> Result<String, ()> {
    todo!()
  }

  fn read_scalar<Scalar>(&mut self, index: Index) -> Result<Scalar, ()> {
    todo!()
  }

  fn read_compound<Compound>(&mut self, indexes: &Compound::Indexes) -> Result<Compound, ()> 
  where 
    Compound: CompoundIndexedRead
  {
    todo!()
  }
}




// trait OrderedWriterDestination {}
