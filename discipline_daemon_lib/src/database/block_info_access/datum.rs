use super::*;

pub struct DatumSchema {
  pub vault_id: ColumnName,
  pub datum_id: ColumnName,
  pub datum_text: ColumnName,
}

impl DatumSchema {
  pub fn new(
    vault_id: ColumnName,
    datum_id: ColumnName,
    datum_text: ColumnName,
  ) -> Self {
    Self {
      vault_id,
      datum_id,
      datum_text,
    }
  }
}

pub struct DatumWriter<'a> {
  pub vault_id: &'a UuidV4,
  pub datum_id: &'a UuidV4,
  pub datum_text: &'a Datum,
}

impl<'a> CompoundValueWriter for DatumWriter<'a> {
  type Schema = DatumSchema;

  fn write(&self, schema: &Self::Schema, writer: &mut impl CompoundValueWriteDestination) {
    writer.write_scalar_value(schema.vault_id, self.vault_id);
    writer.write_scalar_value(schema.datum_id, self.datum_id);
    writer.write_scalar_value(schema.datum_text, self.datum_text);
  }
}