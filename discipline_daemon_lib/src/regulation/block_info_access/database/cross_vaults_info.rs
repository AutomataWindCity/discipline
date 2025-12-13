use super::*;

pub struct CrossVaultGroupInfoSchema {
  vault_number: Key,
  maximum_vault_number: Key,
  data_number: Key,
  maximum_data_number: Key,
}

impl CrossVaultGroupInfoSchema {
  pub fn new(
    vault_number: Key,
    maximum_vault_number: Key,
    data_number: Key,
    maximum_data_number: Key,
  ) -> Self {
    Self {
      data_number,
      maximum_data_number,
      maximum_vault_number,
      vault_number,
    }
  }
}

impl WriteCompoundValue for CrossVaultGroupInfo {
  type Schema = CrossVaultGroupInfoSchema;

  fn write(value: &Self, schema: &Self::Schema, destination: &mut impl CompoundValueWriteDestination) {
    destination.write_scalar_value(schema.vault_number, &value.get_vault_number());
    destination.write_scalar_value(schema.maximum_vault_number, &value.get_maximum_vault_number());
    destination.write_scalar_value(schema.data_number, &value.get_data_number());
    destination.write_scalar_value(schema.maximum_data_number, &value.get_maximum_data_number());
  }
}

impl ReadCompoundValue for CrossVaultGroupInfo {
  type Schema = CrossVaultGroupInfoSchema;

  fn deserialize(source: &mut impl CompoundValueReadSource, schema: &Self::Schema) -> Result<Self, TextualError> {
    Ok(CrossVaultGroupInfo::construct(
      source.read_scalar_value(schema.vault_number)?, 
      source.read_scalar_value(schema.maximum_vault_number)?, 
      source.read_scalar_value(schema.data_number)?, 
      source.read_scalar_value(schema.maximum_data_number)?,
    ))
  }
}