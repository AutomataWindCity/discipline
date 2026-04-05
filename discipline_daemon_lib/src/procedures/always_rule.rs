use crate::IsTextualError;
use crate::launcher::{State, UserProfiles};
use crate::x::{AlwaysRule, Duration, RuleEnabler, RulesStats, UuidV4};
use crate::x::launcher::{UserProfile, Daemon};
use crate::x::database::*;

pub enum DbError {
  NoSuchRule,
  DuplicateRuleId,
  Other,
}

pub struct DbBatchOps {}

impl DbBatchOps {
  pub fn create() -> Self {
    Self {}
  }

  pub fn commit(&self, database: &Database) -> Result<(), DbError> {
    todo!()
  }

  pub fn write_add_rule_at_user_profile_screen_access_regulation(
    &mut self, 
    user_profile_id: &UuidV4, 
    rule_id: &UuidV4, 
    rule: &AlwaysRule,
  ) {}

  pub fn write_add_rule_at_user_profile_device_access_regulation(
    &mut self, 
    user_profile_id: &UuidV4, 
    rule_id: &UuidV4, 
    rule: &AlwaysRule,
  ) {}

  pub fn write_add_rule_at_user_profile_internet_access_regulation(
    &mut self, 
    user_profile_id: &UuidV4, 
    rule_id: &UuidV4, 
    rule: &AlwaysRule,
  ) {}
}


  pub fn add_rule_at_user_profile_screen_access_regulation(
    database: &Database,
    client_created_rule_id: bool,
    user_profile_id: &UuidV4, 
    rule_id: &UuidV4, 
    rule: &AlwaysRule,
  ) -> Result<(), CreateReturn> {
    todo!()
  }

  pub fn add_rule_at_user_profile_device_access_regulation(
    database: &Database,
    client_created_rule_id: bool,
    user_profile_id: &UuidV4, 
    rule_id: &UuidV4, 
    rule: &AlwaysRule,
  ) -> Result<(), CreateReturn> {
    todo!()
  }

  pub fn add_rule_at_user_profile_internet_access_regulation(
    database: &Database,
    client_created_rule_id: bool,
    user_profile_id: &UuidV4, 
    rule_id: &UuidV4, 
    rule: &AlwaysRule,
  ) -> Result<(), CreateReturn> {
    todo!()
  }

pub enum RuleEnablerCreator {
  Countdown(Duration),
  CountdownAfterPlea(Duration),
}

impl RuleEnablerCreator {
  pub fn create(self) -> RuleEnabler {
    todo!()
  }
}

pub enum AlwaysRuleLocation {
  UserProfileScreenRegulation { user_profile_id: UuidV4 },
  UserProfileDeviceRegulation { user_profile_id: UuidV4 },
  UserProfileInternetRegulation { user_profile_id: UuidV4 },
}

impl AlwaysRuleLocation {
  pub fn get_context<'a>(&'a self, daemon: &'a Daemon) -> Option<AlwaysRuleContext<'a>> {
    Some(match self {
      AlwaysRuleLocation::UserProfileDeviceRegulation { user_profile_id } => {
        AlwaysRuleContext::UserProfileDeviceRegulation { 
          user_profile_id, 
          user_profile: daemon.state.user_profiles.get_profile_given_id(user_profile_id)?,
        }
      }
      AlwaysRuleLocation::UserProfileInternetRegulation { user_profile_id } => {
        AlwaysRuleContext::UserProfileInternetRegulation { 
          user_profile_id, 
          user_profile: daemon.state.user_profiles.get_profile_given_id(user_profile_id)?,
        }
      }
      AlwaysRuleLocation::UserProfileScreenRegulation { user_profile_id } => {
        AlwaysRuleContext::UserProfileScreenRegulation { 
          user_profile_id, 
          user_profile: daemon.state.user_profiles.get_profile_given_id(user_profile_id)?,
        }
      }
    })
  }

  pub fn get_context_mut<'a>(&'a self, daemon: &'a mut Daemon) -> Option<AlwaysRuleContextMut<'a>> {
    Some(match self {
      AlwaysRuleLocation::UserProfileDeviceRegulation { user_profile_id } => {
        AlwaysRuleContextMut::UserProfileDeviceRegulation { 
          user_profile_id, 
          user_profile: daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id)?,
        }
      }
      AlwaysRuleLocation::UserProfileInternetRegulation { user_profile_id } => {
        AlwaysRuleContextMut::UserProfileInternetRegulation { 
          user_profile_id, 
          user_profile: daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id)?,
        }
      }
      AlwaysRuleLocation::UserProfileScreenRegulation { user_profile_id } => {
        AlwaysRuleContextMut::UserProfileScreenRegulation { 
          user_profile_id, 
          user_profile: daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id)?,
        }
      }
    })
  }
}

pub enum AlwaysRuleContext<'a> {
  UserProfileScreenRegulation { user_profile_id: &'a UuidV4, user_profile: &'a UserProfile },
  UserProfileDeviceRegulation { user_profile_id: &'a UuidV4, user_profile: &'a UserProfile },
  UserProfileInternetRegulation { user_profile_id: &'a UuidV4, user_profile: &'a UserProfile }, 
}

pub enum AlwaysRuleContextMut<'a> {
  UserProfileScreenRegulation { user_profile_id: &'a UuidV4, user_profile: &'a mut UserProfile },
  UserProfileDeviceRegulation { user_profile_id: &'a UuidV4, user_profile: &'a mut UserProfile },
  UserProfileInternetRegulation { user_profile_id: &'a UuidV4, user_profile: &'a mut UserProfile }, 
}

pub struct Create {
  rule_location: AlwaysRuleLocation,
  rule_id: Option<UuidV4>,
  rule_enabler: RuleEnablerCreator,
}

pub enum CreateReturn {
  TooManyRules,
  NoSuchUserProfile,
  DuplicateRuleId,
  InternalError,
  Success,
}

fn create(
  database: &Database,
  global_stats: &mut RulesStats,
  rule_location: &AlwaysRuleLocation,
  rule_id: Option<UuidV4>,
  rule_enabler: RuleEnablerCreator,
  textual_error: &mut impl IsTextualError,
) -> Option<CreateReturn> {
  if global_stats.rules_number >= global_stats.maximum_rules_number {
    return Some(CreateReturn::TooManyRules);
  }
  
  let client_created_rule_id = rule_id.is_some();
  let rule_id = rule_id.unwrap_or_else(UuidV4::generate);
  let rule = AlwaysRule::create(rule_enabler.create());

  if let Err(error) = always_rule_table::insert_rule(
    &database, 
    rule_location, 
    &rule_id, 
    &rule, 
    textual_error,
  ) {
    return Some(match error {
      always_rule_table::InsertError::DuplicateRuleId if client_created_rule_id => {
        CreateReturn::DuplicateRuleId
      }
      always_rule_table::InsertError::DuplicateRuleId => {
        CreateReturn::InternalError
      }
      always_rule_table::InsertError::Other => {
        CreateReturn::InternalError
      }
    });
  }

  global_stats.update_after_always_rule_created();
  None
}

fn create_for_user_profile_screen_regulation(
  daemon: &mut Daemon,
  user_profile_id: &UuidV4,
  rule_id: Option<UuidV4>,
  rule_enabler: RuleEnablerCreator,
  textual_error: &mut impl IsTextualError,
) {
  let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id) else {
    return;
  };
  
  let Some(rule_stats_updater) = user_profile.rules_stats.create_add_always_rule_updater() else {
    return;
  };

  
}

pub enum DeviceRegulationOperation {

}

pub enum Operation {
    
}

fn revert_create(
  daemon: &mut Daemon,
  rule_location: &AlwaysRuleLocation,
) {}

fn create_at_screen(
  daemon: &mut Daemon,
  rule_location: &AlwaysRuleLocation,
  rule_id: Option<UuidV4>,
  rule_enabler: RuleEnablerCreator,
  textual_error: &mut impl IsTextualError,
) -> CreateReturn {
  if let Err(()) = daemon.state.rules_stats.add_always_rule() {
    return CreateReturn::TooManyRules;
  }

  let client_created_rule_id = rule_id.is_some();
  let rule_id = rule_id.unwrap_or_else(UuidV4::generate);
  let rule = AlwaysRule::create(rule_enabler.create());

  match rule_location {
    AlwaysRuleLocation::UserProfileDeviceRegulation { user_profile_id } => {
      let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id) else {
        return CreateReturn::NoSuchUserProfile;
      };

      if let Err(()) = user_profile.rules_stats.add_always_rule() {
        return CreateReturn::TooManyRules;
      }

      if let Err(error) = add_rule_at_user_profile_device_access_regulation(
        &daemon.database, 
        client_created_rule_id,
        user_profile_id, 
        &rule_id, 
        &rule,
      ) {
        revert_create(daemon, rule_location);
        return error;
      }

      user_profile.device_access_regulation.always_rules.rules.insert(rule_id, rule);
    }
    AlwaysRuleLocation::UserProfileInternetRegulation { user_profile_id } => {
      let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id) else {
        return CreateReturn::NoSuchUserProfile;
      };

      
      if let Err(()) = user_profile.rules_stats.add_always_rule() {
        return CreateReturn::TooManyRules;
      }

      if let Err(error) = add_rule_at_user_profile_internet_access_regulation(
        &daemon.database, 
        client_created_rule_id,
        user_profile_id, 
        &rule_id, 
        &rule,
      ) {
        revert_create(daemon, rule_location);
        return error;
      }

      user_profile.internet_access_regulation.always_rules.rules.insert(rule_id, rule);
    }
    AlwaysRuleLocation::UserProfileScreenRegulation { user_profile_id } => {
      let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id) else {
        return CreateReturn::NoSuchUserProfile;
      };

      if let Err(()) = user_profile.rules_stats.add_always_rule() {
        return CreateReturn::TooManyRules;
      }

      if let Err(error) = add_rule_at_user_profile_screen_access_regulation(
        &daemon.database, 
        client_created_rule_id,
        user_profile_id, 
        &rule_id, 
        &rule,
      ) {
        revert_create(daemon, rule_location);
        return error;
      }

      user_profile.screen_access_regulation.always_rules.rules.insert(rule_id, rule);
    }
  }

  // if let Err(error) = statement.commit(&daemon.database) {
  //   return match error {
  //     DbError::DuplicateRuleId if client_created_rule_id => {
  //       CreateReturn::DuplicateRuleId
  //     }
  //     _ => {
  //       CreateReturn::InternalError
  //     }
  //   };
  // }

  CreateReturn::Success  
}

fn revert_from_create(
  daemon: &mut Daemon,
  rule_location: &AlwaysRuleLocation,
  rule_id: UuidV4,
) {

}

fn create_at_device(

) {}
fn create_at_internet() {}

// impl Create {
//   pub fn execute(
//     self, 
//     daemon: &mut Daemon, 
//     textual_error: &mut impl IsTextualError,
//   ) -> CreateReturn {
//     let Some(rule_context) = self.rule_location.get_context(daemon) else {
//       return CreateReturn::TooManyRules
//     };

//     match rule_context {
//       AlwaysRuleContext::UserProfileDeviceRegulation { user_profile, .. } => {
//         if user_profile.rules_stats.
//       }
//       AlwaysRuleContext::UserProfileInternetRegulation { user_profile, .. } => {

//       }
//       AlwaysRuleContext::UserProfileScreenRegulation { user_profile, .. } => {

//       }
//     }


//     if daemon.state.rules_stats.rules_number >= daemon.state.rules_stats.maximum_rules_number {
//       return CreateReturn::TooManyRules;
//     }
    
//     let client_created_rule_id = self.rule_id.is_some();
//     let rule_id = self.rule_id.unwrap_or_else(UuidV4::generate);
//     let rule = AlwaysRule::create(self.rule_enabler.create());

//     if let Err(error) = always_rule_table::insert_rule(
//       &daemon.database, 
//       &rule_context, 
//       &rule_id, 
//       &rule, 
//       textual_error,
//     ) {
//       return match error {
//         always_rule_table::InsertError::DuplicateRuleId if client_created_rule_id => {
//           CreateReturn::DuplicateRuleId
//         }
//         always_rule_table::InsertError::DuplicateRuleId => {
//           CreateReturn::InternalError
//         }
//         always_rule_table::InsertError::Other => {
//           CreateReturn::InternalError
//         }
//       };
//     }

//     daemon.state.rules_stats.update_after_always_rule_created();

//     match rule_context {
//       AlwaysRuleContext::UserProfileDeviceRegulation { user_profile, .. } => {
//         user_profile.rules_stats.update_after_always_rule_deleted();
//       }
//       AlwaysRuleContext::UserProfileInternetRegulation { user_profile, .. } => {
//         user_profile.rules_stats.update_after_always_rule_deleted();
//       }
//       AlwaysRuleContext::UserProfileScreenRegulation { user_profile, .. } => {
//         user_profile.rules_stats.update_after_always_rule_deleted();
//       }
//     }

//     CreateReturn::Success
//   }
// }

// pub struct Delete {
//   rule_location: AlwaysRuleLocator,
//   rule_id: UuidV4,
// }

// pub enum DeleteReturn {
//   NoSuchUserProfile,
//   NoSuchRule,
//   PermissionDenied,
//   InternalError,
//   Success,
// }

// impl Delete {
//   pub fn execute(
//     self, 
//     daemon: &mut Daemon, 
//     textual_error: &mut impl IsTextualError,
//   ) -> DeleteReturn {
//     let (rule, rule_context) = match &self.rule_location {
//       AlwaysRuleLocator::UserProfileDeviceRegulation { user_profile_id } => {
//         let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id) else {
//           return DeleteReturn::NoSuchUserProfile;
//         };

//         let Some(rule) = user_profile.device_access_regulation.always_rules.rules.get_mut(&self.rule_id) else {
//           return DeleteReturn::NoSuchRule;
//         };

//         (rule, AlwaysRuleContext::UserProfileDeviceRegulation { user_profile_id, user_profile })
//       }
//       AlwaysRuleLocator::UserProfileInternetRegulation { user_profile_id } => {
//         let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id) else {
//           return DeleteReturn::NoSuchUserProfile;
//         };

//         let Some(rule) = user_profile.internet_access_regulation.always_rules.rules.get_mut(&self.rule_id) else {
//           return DeleteReturn::NoSuchRule;
//         };
        
//         (rule, AlwaysRuleContext::UserProfileInternetRegulation { user_profile_id, user_profile })
//       }
//       AlwaysRuleLocator::UserProfileScreenRegulation { user_profile_id } => {
//         let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id) else {
//           return DeleteReturn::NoSuchUserProfile;
//         };

//         let Some(rule) = user_profile.screen_access_regulation.always_rules.rules.get_mut(&self.rule_id) else {
//           return DeleteReturn::NoSuchRule;
//         };
        
//         (rule, AlwaysRuleContext::UserProfileScreenRegulation { user_profile_id, user_profile })
//       }
//     };

//     let now = daemon.state.monotonic_clock.now();
//     if rule.is_enabled(now) {
//       return DeleteReturn::PermissionDenied;
//     }

//     if let Err(error) = always_rule_table::delete_rule(
//       &daemon.database, 
//       &rule_context,
//       &self.rule_id, 
//       textual_error,
//     ) {
//       return match error {
//         always_rule_table::DeleteRule::NoSuchRule => {
//           DeleteReturn::NoSuchRule
//         }
//         always_rule_table::DeleteRule::Other => {
//           DeleteReturn::InternalError
//         }
//       }
//     }

//     daemon.state.rules_stats.update_after_always_rule_deleted();

//     match rule_context {
//       AlwaysRuleContext::UserProfileDeviceRegulation { user_profile, .. } => {
//         user_profile.rules_stats.update_after_always_rule_deleted();
//         user_profile.device_access_regulation.always_rules.rules.remove(&self.rule_id);
//       }
//       AlwaysRuleContext::UserProfileInternetRegulation { user_profile, .. } => {
//         user_profile.rules_stats.update_after_always_rule_deleted();
//         user_profile.internet_access_regulation.always_rules.rules.remove(&self.rule_id);
//       }
//       AlwaysRuleContext::UserProfileScreenRegulation { user_profile, .. } => {
//         user_profile.rules_stats.update_after_always_rule_deleted();
//         user_profile.screen_access_regulation.always_rules.rules.remove(&self.rule_id);
//       }
//     }

//     DeleteReturn::Success
//   }
// }

// pub struct EnablerCountdownActivate {
//   rule_location: AlwaysRuleLocator,
//   rule_id: UuidV4,
// }

// pub enum EnablerCountdownActivateReturn {
//   NoSuchUserProfile,
//   NoSuchRule,
//   WrongEnablerType,
//   InternalError,
//   Success,
// }

// impl EnablerCountdownActivate {
//   pub fn execute(
//     self,
//     daemon: &mut Daemon,
//     textual_error: &mut impl IsTextualError,
//   ) -> EnablerCountdownActivateReturn {
//     let (rule, rule_context) = match &self.rule_location {
//       AlwaysRuleLocator::UserProfileDeviceRegulation { user_profile_id } => {
//         let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id) else {
//           return EnablerCountdownActivateReturn::NoSuchUserProfile;
//         };

//         let Some(rule) = user_profile.device_access_regulation.always_rules.rules.get(&self.rule_id) else {
//           return EnablerCountdownActivateReturn::NoSuchRule;
//         };

//         (rule, AlwaysRuleContext::UserProfileDeviceRegulation { user_profile_id, user_profile })
//       }
//       AlwaysRuleLocator::UserProfileInternetRegulation { user_profile_id } => {
//         let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id) else {
//           return EnablerCountdownActivateReturn::NoSuchUserProfile;
//         };

//         let Some(rule) = user_profile.internet_access_regulation.always_rules.rules.get(&self.rule_id) else {
//           return EnablerCountdownActivateReturn::NoSuchRule;
//         };
        
//         (rule, AlwaysRuleContext::UserProfileInternetRegulation { user_profile_id, user_profile })
//       }
//       AlwaysRuleLocator::UserProfileScreenRegulation { user_profile_id } => {
//         let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id) else {
//           return EnablerCountdownActivateReturn::NoSuchUserProfile;
//         };

//         let Some(rule) = user_profile.screen_access_regulation.always_rules.rules.get(&self.rule_id) else {
//           return EnablerCountdownActivateReturn::NoSuchRule;
//         };
        
//         (rule, AlwaysRuleContext::UserProfileScreenRegulation { user_profile_id, user_profile })
//       }
//     };

//     let RuleEnabler::Countdown(conditional) = &mut rule.enabler else {
//       return EnablerCountdownActivateReturn::WrongEnablerType;
//     };

//     let now = daemon.state.monotonic_clock.now();
//     let activate_state = conditional.create_activate_state(now);

//     if let Err(error) = always_rule_table::enabler_countdown_activate(
//       &daemon.database, 
//       &rule_context,
//       &self.rule_id, 
//       &activate_state,
//       textual_error,
//     ) {
//       return match error {
//         always_rule_table::EnablerCountdownActivate::NoSuchRule => {
//           EnablerCountdownActivateReturn::NoSuchRule
//         }
//         always_rule_table::EnablerCountdownActivate::Other => {
//           EnablerCountdownActivateReturn::InternalError
//         }
//       }
//     }

//     conditional.activate_from_activate_state(activate_state);
//     EnablerCountdownActivateReturn::Success
//   }
// }

// pub struct EnablerCountdownAfterPleaActivate {
//   location: AlwaysRuleLocator,
//   rule_id: UuidV4,
// }

// pub enum EnablerCountdownAfterPleaActivateReturn {
//   NoSuchUserProfile,
//   NoSuchRule,
//   WrongEnablerType,
//   AlreadyActive,
//   InternalError,
//   Success,
// }

// impl EnablerCountdownAfterPleaActivate {
//   pub fn execute(
//     self,
//     daemon: &mut Daemon,
//     textual_error: &mut impl IsTextualError,
//   ) -> EnablerCountdownAfterPleaActivateReturn {
//     let (rule, rule_context) = match &self.rule_location {
//       AlwaysRuleLocator::UserProfileDeviceRegulation { user_profile_id } => {
//         let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id) else {
//           return EnablerCountdownAfterPleaActivateReturn::NoSuchUserProfile;
//         };

//         let Some(rule) = user_profile.device_access_regulation.always_rules.rules.get(&self.rule_id) else {
//           return EnablerCountdownAfterPleaActivateReturn::NoSuchRule;
//         };

//         (rule, AlwaysRuleContext::UserProfileDeviceRegulation { user_profile_id, user_profile })
//       }
//       AlwaysRuleLocator::UserProfileInternetRegulation { user_profile_id } => {
//         let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id) else {
//           return EnablerCountdownAfterPleaActivateReturn::NoSuchUserProfile;
//         };

//         let Some(rule) = user_profile.internet_access_regulation.always_rules.rules.get(&self.rule_id) else {
//           return EnablerCountdownAfterPleaActivateReturn::NoSuchRule;
//         };
        
//         (rule, AlwaysRuleContext::UserProfileInternetRegulation { user_profile_id, user_profile })
//       }
//       AlwaysRuleLocator::UserProfileScreenRegulation { user_profile_id } => {
//         let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id) else {
//           return EnablerCountdownAfterPleaActivateReturn::NoSuchUserProfile;
//         };

//         let Some(rule) = user_profile.screen_access_regulation.always_rules.rules.get(&self.rule_id) else {
//           return EnablerCountdownAfterPleaActivateReturn::NoSuchRule;
//         };
        
//         (rule, AlwaysRuleContext::UserProfileScreenRegulation { user_profile_id, user_profile })
//       }
//     };

//     let RuleEnabler::CountdownAfterPlea(enabler) = &mut rule.enabler else {
//       return EnablerCountdownAfterPleaActivateReturn::WrongEnablerType;
//     };

//     let now = daemon.state.monotonic_clock.now();
//     if enabler.is_active(now) {
//       return EnablerCountdownAfterPleaActivateReturn::AlreadyActive; 
//     }

//     if let Err(error) = always_rule_table::enaber_countdown_after_plea_activate(
//       &daemon.database, 
//       &self.rule_id, 
//       textual_error,
//     ) {
//       return match error {
//         always_rule_table::EnablerCountdownAfterPleaActivate::NoSuchRule => {
//           EnablerCountdownAfterPleaActivateReturn::NoSuchRule
//         }
//         always_rule_table::EnablerCountdownAfterPleaActivate::Other => {
//           EnablerCountdownAfterPleaActivateReturn::InternalError
//         }
//       }
//     }

//     enabler.activate();
//     EnablerCountdownAfterPleaActivateReturn::Success
//   }
// }

// pub struct ProtectorCountdownAfterPleaDeactivate {
//   rule_location: AlwaysRuleLocator,
//   rule_id: UuidV4,
// }

// pub enum ProtectorCountdownAfterPleaDeactivateReturn {
//   NoSuchUserProfile,
//   NoSuchRule,
//   WrongEnablerType,
//   AlreadyDeactivated,
//   InternalError,
//   Success,
// }

// impl ProtectorCountdownAfterPleaDeactivate {
//   pub fn execute(
//     self,
//     daemon: &mut Daemon,
//     textual_error: &mut impl IsTextualError,
//   ) -> ProtectorCountdownAfterPleaDeactivateReturn {
//     let (rule, rule_context) = match &self.rule_location {
//       AlwaysRuleLocator::UserProfileDeviceRegulation { user_profile_id } => {
//         let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id) else {
//           return ProtectorCountdownAfterPleaDeactivateReturn::NoSuchUserProfile;
//         };

//         let Some(rule) = user_profile.device_access_regulation.always_rules.rules.get(&self.rule_id) else {
//           return ProtectorCountdownAfterPleaDeactivateReturn::NoSuchRule;
//         };

//         (rule, AlwaysRuleContext::UserProfileDeviceRegulation { user_profile_id, user_profile })
//       }
//       AlwaysRuleLocator::UserProfileInternetRegulation { user_profile_id } => {
//         let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id) else {
//           return ProtectorCountdownAfterPleaDeactivateReturn::NoSuchUserProfile;
//         };

//         let Some(rule) = user_profile.internet_access_regulation.always_rules.rules.get(&self.rule_id) else {
//           return ProtectorCountdownAfterPleaDeactivateReturn::NoSuchRule;
//         };
        
//         (rule, AlwaysRuleContext::UserProfileInternetRegulation { user_profile_id, user_profile })
//       }
//       AlwaysRuleLocator::UserProfileScreenRegulation { user_profile_id } => {
//         let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(user_profile_id) else {
//           return ProtectorCountdownAfterPleaDeactivateReturn::NoSuchUserProfile;
//         };

//         let Some(rule) = user_profile.screen_access_regulation.always_rules.rules.get(&self.rule_id) else {
//           return ProtectorCountdownAfterPleaDeactivateReturn::NoSuchRule;
//         };
        
//         (rule, AlwaysRuleContext::UserProfileScreenRegulation { user_profile_id, user_profile })
//       }
//     };

//     let RuleEnabler::CountdownAfterPlea(conditional) = &mut rule.enabler else {
//       return ProtectorCountdownAfterPleaDeactivateReturn::WrongEnablerType;
//     };

//     let now = daemon.state.monotonic_clock.now();
//     if conditional.is_deactivated(now) {
//       return ProtectorCountdownAfterPleaDeactivateReturn::AlreadyDeactivated;
//     }

//     let deactivating_state = conditional.create_deactivating_state(now);

//     if let Err(error) = always_rule_table::enabler_countdown_after_plea_deactivate(
//       &daemon.database, 
//       &self.rule_id, 
//       &deactivating_state,
//       textual_error,
//     ) {
//       return match error {
//         always_rule_table::EnablerCountdownAfterPleaDeactivate::NoSuchRule => {
//           ProtectorCountdownAfterPleaDeactivateReturn::NoSuchRule
//         }
//         always_rule_table::EnablerCountdownAfterPleaDeactivate::Other => {
//           ProtectorCountdownAfterPleaDeactivateReturn::InternalError
//         }
//       }
//     }

//     conditional.deactivate(now);
//     ProtectorCountdownAfterPleaDeactivateReturn::Success
//   }
// }
