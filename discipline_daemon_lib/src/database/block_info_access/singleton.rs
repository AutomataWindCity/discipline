use super::*;

pub struct SingletonSchema {
  vault_number: Key,
  vault_maximum_number: Key,
  data_number: Key,
  data_maximum_number: Key,
}

impl SingletonSchema {
  pub fn new(
    vault_number: Key,
    vault_maximum_number: Key,
    data_number: Key,
    data_maximum_number: Key,
  ) -> Self {
    Self {
      vault_maximum_number,
      vault_number,
      data_number,
      data_maximum_number,
    }
  }
}

impl WriteCompoundValue for VaultsSingleton {
  type Schema = SingletonSchema;

  fn write(value: &Self, schema: &Self::Schema, destination: &mut impl CompoundValueWriteDestination) {
    destination.write_scalar_value(schema.vault_number, &value.get_vault_number());
    destination.write_scalar_value(schema.vault_maximum_number, &value.get_maximum_vault_number());
    destination.write_scalar_value(schema.data_number, &value.get_data_number());
    destination.write_scalar_value(schema.data_maximum_number, &value.get_maximum_data_number());
  }
}

impl ReadCompoundValue for VaultsSingleton {
  type Schema = SingletonSchema;

  fn deserialize(source: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(VaultsSingleton::construct(
      source.read_scalar_value(schema.vault_number)?, 
      source.read_scalar_value(schema.vault_maximum_number)?, 
      source.read_scalar_value(schema.data_number)?, 
      source.read_scalar_value(schema.data_maximum_number)?,
    ))
  }
}