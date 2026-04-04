use crate::x::{TextualError, WeekdaySet};
use crate::x::database::{ScalarWrite, ScalarRead, ScalarValueReadSource, ScalarValueWriteDestination};

impl ScalarWrite for WeekdaySet {
  fn write(value: &Self, writer: &mut ScalarValueWriteDestination) {
    writer.write_scalar_value(&value.bitmask());
  }
}

impl ScalarRead for WeekdaySet {
  fn read(reader: &mut ScalarValueReadSource) -> Result<Self, TextualError> {
    let bitmask = reader.read_scalar_value()?;
    Ok(WeekdaySet::from_bitmask(bitmask))
  }
}