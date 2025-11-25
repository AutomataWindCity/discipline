use super::*;

pub struct DatumSchema {
  pub vault_id: Key,
  pub datum_id: Key,
  pub datum_text: Key,
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