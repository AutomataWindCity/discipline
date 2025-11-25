use super::*;

pub struct VaultSchema {
  pub user_id: Key,
  pub vault_id: Key,
  pub vault_name: Key,
  pub vault_protector: ProtectorSchema,
}

impl VaultSchema {
  pub fn new(
    user_id: Key,
    vault_id: Key,
    vault_name: Key,
    vault_protector_enum_type: Key,
    vault_protector_enum_data_1: Key,
    vault_protector_enum_data_2: Key,
    vault_protector_enum_data_3: Key,
  ) -> Self {
    Self {
      user_id,
      vault_id,
      vault_name,
      vault_protector: ProtectorSchema::new(
        vault_protector_enum_type, 
        vault_protector_enum_data_1, 
        vault_protector_enum_data_2, 
        vault_protector_enum_data_3,
      ),
    }
  }
}

pub struct VaultWriter<'a> {
  pub user_id: &'a UuidV4,
  pub vault_id: &'a UuidV4,
  pub vault_name: &'a VaultName,
  pub vault_protector: &'a VaultProtector,
}

impl<'a> CompoundValueWriter for VaultWriter<'a> {
  type Schema = VaultSchema;

  fn write(&self, schema: &Self::Schema, destination: &mut impl CompoundValueWriteDestination) {
    destination.write_scalar_value(schema.user_id, self.user_id);
    destination.write_scalar_value(schema.vault_id, self.vault_id);
    destination.write_scalar_value(schema.vault_name, self.vault_name);
    destination.write_compound_value(&schema.vault_protector, self.vault_protector);
  }
}