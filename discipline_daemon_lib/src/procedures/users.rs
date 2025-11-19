use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::hash_map::Entry;
use crate::x::{MonotonicInstant, User, UserGroup, UserName, UuidV4, operating_system};
use crate::x::database::Transaction;

pub trait TransactionProcedures {
  fn add_user(&self, transaction: &mut impl Transaction, user_id: &UuidV4, user: &User);
  fn delete_user(&self, transaction: &mut impl Transaction, user_id: &UuidV4);
  fn update_user(&self, transaction: &mut impl Transaction, user_id: &UuidV4, user: &User);
  fn update_user_name(&self, transaction: &mut impl Transaction, user_id: &UuidV4, new_user_name: &UserName);
}

pub struct AddUser {
  user_id: Option<UuidV4>,
  user_name: UserName,
  operating_system_user_name: operating_system::UserName,
}

pub enum AddUserFailure {
  TooManyUsers,
  DuplicateUuid,
  NoSuchOperatingSystemUser,
  OperatingSystemError,
}

pub struct RevertAddUser {
  user_id: UuidV4,
}

impl AddUser {
  pub fn execute(
    self,
    transaction: &mut impl Transaction,
    transaction_procedures: &impl TransactionProcedures,
    user_group: &mut UserGroup,
  ) -> Result<RevertAddUser, AddUserFailure> {
    if user_group.users.len() >= user_group.maximum_user_number {
      return Err(AddUserFailure::TooManyUsers);
    }
    
    let user_id = self.user_id.unwrap_or_else(UuidV4::generate);
    if user_group.users.contains_key(&user_id) {
      return Err(AddUserFailure::DuplicateUuid);
    }

    let operating_system_info = match operating_system::get_per_user_info(&self.operating_system_user_name) {
      Ok(info) => {
        info
      }
      Err(operating_system::GetPerUserInfoError::NoSuchUser) => {
        return Err(AddUserFailure::NoSuchOperatingSystemUser);
      }
      Err(_) => {
        return Err(AddUserFailure::OperatingSystemError);
      }
    };

    let user = User::new(
      self.user_name, 
      operating_system_info,
    );

    transaction_procedures.add_user(transaction, &user_id, &user);

    user_group.users.insert(user_id.clone(), Arc::new(Mutex::new(user)));

    Ok(RevertAddUser { user_id })
  }
}

pub struct EnsureUserDeleted {
  user_id: UuidV4,
}

pub enum EnsureUserDeletedFailure {
  UserHasEnabledRegulationRules,
}

pub enum EnsureUserDeletedSuccess {
  NoSuchUser,
  Success,
}

pub struct RevertDeleteUser {
  user_id: UuidV4,
  user: Arc<Mutex<User>>,
}

impl EnsureUserDeleted {
  pub async fn execute(
    self,
    transaction: &mut impl Transaction,
    transaction_procedures: &impl TransactionProcedures,
    user_group: &mut UserGroup,
    now: MonotonicInstant,
  ) -> Result<(EnsureUserDeletedSuccess, Option<RevertDeleteUser>), EnsureUserDeletedFailure> {
    let Entry::Occupied(entry) = user_group.users.entry(self.user_id) else {
      return Ok((EnsureUserDeletedSuccess::NoSuchUser, None));
    };

    let user = entry.get().lock().await;
    if user.regulation.has_activate_rules(now) {
      return Err(EnsureUserDeletedFailure::UserHasEnabledRegulationRules);
    }

    drop(user);
    let (user_id, user) = entry.remove_entry();

    transaction_procedures.delete_user(transaction, &user_id);

    Ok((
      EnsureUserDeletedSuccess::Success,
      Some(RevertDeleteUser { user_id, user })
    ))
  }
}

pub struct UpdateName {
  new_name: UserName
}

pub enum UpdateUser {
  UpdateName(UpdateName),
  BlockDeviceAccess(super::user_block_device_access::Procedure)
}

pub enum UpdateUserReturn {
  UpdateName,
  BlockDeviceAccess(super::user_block_device_access::ProcedureReturn),
}

pub enum Procedure {
  AddUser(AddUser),
  EnsureUserDeleted(EnsureUserDeleted),
  UpdateUser(UpdateUser),
}

pub enum ProcedureSuccess {
  AddUser(RevertAddUser),
  EnsureUserDeleted(EnsureUserDeleted),
  UpdateUser(UpdateUser),
}

pub enum ProcedureRevert {
  AddUser(RevertAddUser),
  DeleteUser(RevertDeleteUser),
  UpdateUser(RevertUpdateUser),
}

pub enum ProcedureFailure {}

impl Procedure {
  pub fn execute(
    self,
    transaction: &mut impl Transaction,
    transaction_procedures: &impl TransactionProcedures,
    user_group: &mut UserGroup,
  ) -> Result<(), > {
    match self {
      Self::AddUser(procedure) => {
        procedure
          .execute(transaction, transaction_procedures, user_group)
          .map(|revert| )
      }
      Self::EnsureUserDeleted(procedure) => {

      }
      Self::UpdateUser(procedure) => {

      }
    }
  }
}