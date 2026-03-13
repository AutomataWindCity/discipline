use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
use crate::x::TextualError;
use super::{CreateVaultDatumFromStringError, CreateVaultNameFromStringError, VaultDatum, VaultName};

impl Serialize for VaultName {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.string)
  }
}

impl<'a> Deserialize<'a> for VaultName {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'a>,
  {
    let string = String::deserialize(deserializer).map_err(|error| {
      Error::custom(
        TextualError::new("Deserializing String")
          .with_message("Failed to deserialize a value of type Srting")
          .with_attachement_display("Error", error)
          .with_context("Deserializing VaultName, which is serialized as String"),
      )
    })?;

    Self::new(string).map_err(|error| {
      Error::custom(match error {
        CreateVaultNameFromStringError::LengthViolation { .. } => {
          TextualError::new("Creating VaultName from String")
            .with_message("String length is invalid")
            .with_attachement_display("Minimum length", VaultName::MINIMUM_LENGTH)
            .with_attachement_display("Maximum length", VaultName::MAXIMUM_LENGTH)
            .with_context("Deserializing VaultName")
        }
      })
    })
  }
}

impl Serialize for VaultDatum {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.string)
  }
}

impl<'a> Deserialize<'a> for VaultDatum {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'a>,
  {
    let string = String::deserialize(deserializer).map_err(|error| {
      Error::custom(
        TextualError::new("Deserializing String")
          .with_message("Failed to deserialize a value of type Srting")
          .with_attachement_display("Error", error)
          .with_context("Deserializing VaultDatum, which is serialized as String"),
      )
    })?;

    Self::new(string).map_err(|error| {
      Error::custom(match error {
        CreateVaultDatumFromStringError::LengthViolation { .. } => {
          TextualError::new("Creating VaultDatum from String")
            .with_message("String length is invalid")
            .with_attachement_display("Minimum length", VaultDatum::MINIMUM_LENGTH)
            .with_attachement_display("Maximum length", VaultDatum::MAXIMUM_LENGTH)
            .with_context("Deserializing VaultDatum")
        }
      })
    })
  }
}
