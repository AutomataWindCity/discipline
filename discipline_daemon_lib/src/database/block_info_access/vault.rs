use super::*;

pub struct VaultSchema {
  pub user_id: ColumnName,
  pub vault_id: ColumnName,
  pub vault_name: ColumnName,
  pub vault_Enabler: EnablerSchema,
}

impl VaultSchema {
  pub fn new(
    user_id: ColumnName,
    vault_id: ColumnName,
    vault_name: ColumnName,
    vault_Enabler_enum_type: ColumnName,
    vault_Enabler_enum_data_1: ColumnName,
    vault_Enabler_enum_data_2: ColumnName,
    vault_Enabler_enum_data_3: ColumnName,
  ) -> Self {
    Self {
      user_id,
      vault_id,
      vault_name,
      vault_Enabler: EnablerSchema::new(
        vault_Enabler_enum_type, 
        vault_Enabler_enum_data_1, 
        vault_Enabler_enum_data_2, 
        vault_Enabler_enum_data_3,
      ),
    }
  }
}

pub struct VaultWriter<'a> {
  pub user_id: &'a UuidV4,
  pub vault_id: &'a UuidV4,
  pub vault_name: &'a VaultName,
  pub vault_Enabler: &'a VaultEnabler,
}

impl<'a> CompoundValueWriter for VaultWriter<'a> {
  type Schema = VaultSchema;

  fn write(&self, schema: &Self::Schema, destination: &mut impl CompoundValueWriteDestination) {
    destination.write_scalar_value(schema.user_id, self.user_id);
    destination.write_scalar_value(schema.vault_id, self.vault_id);
    destination.write_scalar_value(schema.vault_name, self.vault_name);
    destination.write_compound_value(&schema.vault_Enabler, self.vault_Enabler);
  }
}