use crate::x::{TextualError, WeekdaySet};
use crate::x::database::{SerializableScalarValue, DeserializableScalarValue, ScalarValueReader, ScalarValueWrtier};

impl SerializableScalarValue for WeekdaySet {
  fn serialize(value: &Self, writer: &mut ScalarValueWrtier) {
    writer.write_scalar_value(&value.bitmask());
  }
}

impl DeserializableScalarValue for WeekdaySet {
  fn deserialize(reader: &mut ScalarValueReader) -> Result<Self, TextualError> {
    let bitmask = reader.read_scalar_value()?;
    Ok(WeekdaySet::from_bitmask(bitmask))
  }
}