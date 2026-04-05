use crate::IsTextualError;
use crate::x::{AlwaysRule, Duration, RuleEnabler, UuidV4};
use crate::x::launcher::{UserProfile, Daemon};
use crate::x::database::*;

pub enum RuleEnablerCreator {
  Countdown(Duration),
  CountdownAfterPlea(Duration),
}

impl RuleEnablerCreator {
  pub fn create(self) -> RuleEnabler {
    todo!()
  }
}

pub enum AlwaysRuleLocator {
  UserProfileScreenRegulation { user_profile_id: UuidV4 },
  UserProfileDeviceRegulation { user_profile_id: UuidV4 },
  UserProfileInternetRegulation { user_profile_id: UuidV4 },
}

pub enum AlwaysRuleContext<'a> {
  UserProfileScreenRegulation { user_profile_id: &'a UuidV4, user_profile: &'a UserProfile },
  UserProfileDeviceRegulation { user_profile_id: &'a UuidV4, user_profile: &'a UserProfile },
  UserProfileInternetRegulation { user_profile_id: &'a UuidV4, user_profile: &'a UserProfile }, 
}

pub struct Create {
  rule_locator: AlwaysRuleLocator,
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

impl Create {
  pub fn execute(
    self, 
    daemon: &mut Daemon, 
    textual_error: &mut impl IsTextualError,
  ) -> CreateReturn {
    let rule_context = match &self.rule_locator {
      AlwaysRuleLocator::UserProfileDeviceRegulation { user_profile_id } => {
        let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(user_profile_id) else {
          return CreateReturn::NoSuchUserProfile;
        };

        if user_profile.rules_stats.rules_number >= user_profile.rules_stats.maximum_rules_number {
          return CreateReturn::TooManyRules;
        }

        AlwaysRuleContext::UserProfileDeviceRegulation { user_profile_id, user_profile }
      }
      AlwaysRuleLocator::UserProfileInternetRegulation { user_profile_id } => {
        let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(user_profile_id) else {
          return CreateReturn::NoSuchUserProfile;
        };

        if user_profile.rules_stats.rules_number >= user_profile.rules_stats.maximum_rules_number {
          return CreateReturn::TooManyRules;
        }
        
        AlwaysRuleContext::UserProfileDeviceRegulation { user_profile_id, user_profile }
      }
      AlwaysRuleLocator::UserProfileScreenRegulation { user_profile_id } => {
        let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(user_profile_id) else {
          return CreateReturn::NoSuchUserProfile;
        };

        if user_profile.rules_stats.rules_number >= user_profile.rules_stats.maximum_rules_number {
          return CreateReturn::TooManyRules;
        }
        
        AlwaysRuleContext::UserProfileDeviceRegulation { user_profile_id, user_profile }
      }
    };

    if daemon.state.rules_stats.rules_number >= daemon.state.rules_stats.maximum_rules_number {
      return CreateReturn::TooManyRules;
    }
    
    let client_created_rule_id = self.rule_id.is_some();
    let rule_id = self.rule_id.unwrap_or_else(UuidV4::generate);
    let rule = AlwaysRule::create(self.rule_enabler.create());

    if let Err(error) = always_rule_table::insert_rule(
      &daemon.database, 
      &rule_context, 
      &rule_id, 
      &rule, 
      textual_error,
    ) {
      return match error {
        always_rule_table::InsertError::DuplicateRuleId if client_created_rule_id => {
          CreateReturn::DuplicateRuleId
        }
        always_rule_table::InsertError::DuplicateRuleId => {
          CreateReturn::InternalError
        }
        always_rule_table::InsertError::Other => {
          CreateReturn::InternalError
        }
      };
    }

    daemon.state.rules_stats.update_after_always_rule_created();

    match rule_context {
      AlwaysRuleContext::UserProfileDeviceRegulation { user_profile_id, user_profile } => {
        user_profile.rules_stats.update_after_always_rule_deleted();
      }
      AlwaysRuleContext::UserProfileInternetRegulation { user_profile_id, user_profile } => {
        user_profile.rules_stats.update_after_always_rule_deleted();
      }
      AlwaysRuleContext::UserProfileScreenRegulation { user_profile_id, user_profile } => {
        user_profile.rules_stats.update_after_always_rule_deleted();
      }
    }

    CreateReturn::Success
  }
}

pub struct Delete {
  user_profile_id: UuidV4,
  rule_id: UuidV4,
}

pub enum DeleteReturn {
  NoSuchUserProfile,
  NoSuchRule,
  PermissionDenied,
  InternalError,
  Success,
}

impl Delete {
  pub fn execute(
    self, 
    daemon: &mut Daemon, 
    textual_error: &mut impl IsTextualError,
  ) -> DeleteReturn {
    let (rule, rule_context) = match &self.rule_locator {
      AlwaysRuleLocator::UserProfileDeviceRegulation { user_profile_id } => {
        let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(user_profile_id) else {
          return DeleteReturn::NoSuchUserProfile;
        };

        let Some(rule) = user_profile.device_access_regulation.always_rules.rules.get(&self.rule_id) else {
          return DeleteReturn::NoSuchRule;
        };

        (rule, AlwaysRuleContext::UserProfileDeviceRegulation { user_profile_id, user_profile })
      }
      AlwaysRuleLocator::UserProfileInternetRegulation { user_profile_id } => {
        let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(user_profile_id) else {
          return DeleteReturn::NoSuchUserProfile;
        };

        let Some(rule) = user_profile.internet_access_regulation.always_rules.rules.get(&self.rule_id) else {
          return DeleteReturn::NoSuchRule;
        };
        
        (rule, AlwaysRuleContext::UserProfileInternetRegulation { user_profile_id, user_profile })
      }
      AlwaysRuleLocator::UserProfileScreenRegulation { user_profile_id } => {
        let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(user_profile_id) else {
          return DeleteReturn::NoSuchUserProfile;
        };

        let Some(rule) = user_profile.screen_access_regulation.always_rules.rules.get(&self.rule_id) else {
          return DeleteReturn::NoSuchRule;
        };
        
        (rule, AlwaysRuleContext::UserProfileScreenRegulation { user_profile_id, user_profile })
      }
    };

    let now = daemon.state.monotonic_clock.now();
    if rule.is_enabled(now) {
      return DeleteReturn::PermissionDenied;
    }

    if let Err(error) = always_rule_table::delete_rule(
      &daemon.database, 
      &rule_context,
      &self.rule_id, 
      textual_error,
    ) {
      return match error {
        always_rule_table::DeleteRule::NoSuchRule => {
          DeleteReturn::NoSuchRule
        }
        always_rule_table::DeleteRule::Other => {
          DeleteReturn::InternalError
        }
      }
    }

    daemon.state.rules_stats.update_after_always_rule_deleted();

    match rule_context {
      AlwaysRuleContext::UserProfileDeviceRegulation { user_profile, .. } => {
        user_profile.rules_stats.update_after_always_rule_deleted();
        user_profile.device_access_regulation.always_rules.rules.remove(&self.rule_id);
      }
      AlwaysRuleContext::UserProfileInternetRegulation { user_profile, .. } => {
        user_profile.rules_stats.update_after_always_rule_deleted();
        user_profile.internet_access_regulation.always_rules.rules.remove(&self.rule_id);
      }
      AlwaysRuleContext::UserProfileScreenRegulation { user_profile, .. } => {
        user_profile.rules_stats.update_after_always_rule_deleted();
        user_profile.screen_access_regulation.always_rules.rules.remove(&self.rule_id);
      }
    }

    DeleteReturn::Success
  }
}

pub struct EnablerCountdownActivate {
  rule_locator: AlwaysRuleLocator,
  rule_id: UuidV4,
}

pub enum EnablerCountdownActivateReturn {
  NoSuchUserProfile,
  NoSuchRule,
  WrongEnablerType,
  InternalError,
  Success,
}

impl EnablerCountdownActivate {
  pub fn execute(
    self,
    daemon: &mut Daemon,
    textual_error: &mut impl IsTextualError,
  ) -> EnablerCountdownActivateReturn {
    let (rule, rule_context) = match &self.rule_locator {
      AlwaysRuleLocator::UserProfileDeviceRegulation { user_profile_id } => {
        let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(user_profile_id) else {
          return EnablerCountdownActivateReturn::NoSuchUserProfile;
        };

        let Some(rule) = user_profile.device_access_regulation.always_rules.rules.get(&self.rule_id) else {
          return EnablerCountdownActivateReturn::NoSuchRule;
        };

        (rule, AlwaysRuleContext::UserProfileDeviceRegulation { user_profile_id, user_profile })
      }
      AlwaysRuleLocator::UserProfileInternetRegulation { user_profile_id } => {
        let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(user_profile_id) else {
          return EnablerCountdownActivateReturn::NoSuchUserProfile;
        };

        let Some(rule) = user_profile.internet_access_regulation.always_rules.rules.get(&self.rule_id) else {
          return EnablerCountdownActivateReturn::NoSuchRule;
        };
        
        (rule, AlwaysRuleContext::UserProfileInternetRegulation { user_profile_id, user_profile })
      }
      AlwaysRuleLocator::UserProfileScreenRegulation { user_profile_id } => {
        let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(user_profile_id) else {
          return EnablerCountdownActivateReturn::NoSuchUserProfile;
        };

        let Some(rule) = user_profile.screen_access_regulation.always_rules.rules.get(&self.rule_id) else {
          return EnablerCountdownActivateReturn::NoSuchRule;
        };
        
        (rule, AlwaysRuleContext::UserProfileScreenRegulation { user_profile_id, user_profile })
      }
    };

    let RuleEnabler::Countdown(conditional) = &mut rule.enabler else {
      return EnablerCountdownActivateReturn::WrongEnablerType;
    };

    let now = daemon.state.monotonic_clock.now();
    let activate_state = conditional.create_activate_state(now);

    if let Err(error) = always_rule_table::enabler_countdown_activate(
      &daemon.database, 
      &rule_context,
      &self.rule_id, 
      &activate_state,
      textual_error,
    ) {
      return match error {
        always_rule_table::EnablerCountdownActivate::NoSuchRule => {
          EnablerCountdownActivateReturn::NoSuchRule
        }
        always_rule_table::EnablerCountdownActivate::Other => {
          EnablerCountdownActivateReturn::InternalError
        }
      }
    }

    conditional.activate_from_activate_state(activate_state);
    EnablerCountdownActivateReturn::Success
  }
}

pub struct EnablerCountdownAfterPleaActivate {
  locator: AlwaysRuleLocator,
  rule_id: UuidV4,
}

pub enum EnablerCountdownAfterPleaActivateReturn {
  NoSuchUserProfile,
  NoSuchRule,
  WrongEnablerType,
  AlreadyActive,
  InternalError,
  Success,
}

impl EnablerCountdownAfterPleaActivate {
  pub fn execute(
    self,
    daemon: &mut Daemon,
    textual_error: &mut impl IsTextualError,
  ) -> EnablerCountdownAfterPleaActivateReturn {
    let (rule, rule_context) = match &self.rule_locator {
      AlwaysRuleLocator::UserProfileDeviceRegulation { user_profile_id } => {
        let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(user_profile_id) else {
          return EnablerCountdownAfterPleaActivateReturn::NoSuchUserProfile;
        };

        let Some(rule) = user_profile.device_access_regulation.always_rules.rules.get(&self.rule_id) else {
          return EnablerCountdownAfterPleaActivateReturn::NoSuchRule;
        };

        (rule, AlwaysRuleContext::UserProfileDeviceRegulation { user_profile_id, user_profile })
      }
      AlwaysRuleLocator::UserProfileInternetRegulation { user_profile_id } => {
        let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(user_profile_id) else {
          return EnablerCountdownAfterPleaActivateReturn::NoSuchUserProfile;
        };

        let Some(rule) = user_profile.internet_access_regulation.always_rules.rules.get(&self.rule_id) else {
          return EnablerCountdownAfterPleaActivateReturn::NoSuchRule;
        };
        
        (rule, AlwaysRuleContext::UserProfileInternetRegulation { user_profile_id, user_profile })
      }
      AlwaysRuleLocator::UserProfileScreenRegulation { user_profile_id } => {
        let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(user_profile_id) else {
          return EnablerCountdownAfterPleaActivateReturn::NoSuchUserProfile;
        };

        let Some(rule) = user_profile.screen_access_regulation.always_rules.rules.get(&self.rule_id) else {
          return EnablerCountdownAfterPleaActivateReturn::NoSuchRule;
        };
        
        (rule, AlwaysRuleContext::UserProfileScreenRegulation { user_profile_id, user_profile })
      }
    };

    let RuleEnabler::CountdownAfterPlea(enabler) = &mut rule.enabler else {
      return EnablerCountdownAfterPleaActivateReturn::WrongEnablerType;
    };

    let now = daemon.state.monotonic_clock.now();
    if enabler.is_active(now) {
      return EnablerCountdownAfterPleaActivateReturn::AlreadyActive; 
    }

    if let Err(error) = always_rule_table::enaber_countdown_after_plea_activate(
      &daemon.database, 
      &self.rule_id, 
      textual_error,
    ) {
      return match error {
        always_rule_table::EnablerCountdownAfterPleaActivate::NoSuchRule => {
          EnablerCountdownAfterPleaActivateReturn::NoSuchRule
        }
        always_rule_table::EnablerCountdownAfterPleaActivate::Other => {
          EnablerCountdownAfterPleaActivateReturn::InternalError
        }
      }
    }

    enabler.activate();
    EnablerCountdownAfterPleaActivateReturn::Success
  }
}

pub struct ProtectorCountdownAfterPleaDeactivate {
  rule_locator: AlwaysRuleLocator,
  rule_id: UuidV4,
}

pub enum ProtectorCountdownAfterPleaDeactivateReturn {
  NoSuchUserProfile,
  NoSuchRule,
  WrongEnablerType,
  AlreadyDeactivated,
  InternalError,
  Success,
}

impl ProtectorCountdownAfterPleaDeactivate {
  pub fn execute(
    self,
    daemon: &mut Daemon,
    textual_error: &mut impl IsTextualError,
  ) -> ProtectorCountdownAfterPleaDeactivateReturn {
    let (rule, rule_context) = match &self.rule_locator {
      AlwaysRuleLocator::UserProfileDeviceRegulation { user_profile_id } => {
        let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(user_profile_id) else {
          return ProtectorCountdownAfterPleaDeactivateReturn::NoSuchUserProfile;
        };

        let Some(rule) = user_profile.device_access_regulation.always_rules.rules.get(&self.rule_id) else {
          return ProtectorCountdownAfterPleaDeactivateReturn::NoSuchRule;
        };

        (rule, AlwaysRuleContext::UserProfileDeviceRegulation { user_profile_id, user_profile })
      }
      AlwaysRuleLocator::UserProfileInternetRegulation { user_profile_id } => {
        let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(user_profile_id) else {
          return ProtectorCountdownAfterPleaDeactivateReturn::NoSuchUserProfile;
        };

        let Some(rule) = user_profile.internet_access_regulation.always_rules.rules.get(&self.rule_id) else {
          return ProtectorCountdownAfterPleaDeactivateReturn::NoSuchRule;
        };
        
        (rule, AlwaysRuleContext::UserProfileInternetRegulation { user_profile_id, user_profile })
      }
      AlwaysRuleLocator::UserProfileScreenRegulation { user_profile_id } => {
        let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(user_profile_id) else {
          return ProtectorCountdownAfterPleaDeactivateReturn::NoSuchUserProfile;
        };

        let Some(rule) = user_profile.screen_access_regulation.always_rules.rules.get(&self.rule_id) else {
          return ProtectorCountdownAfterPleaDeactivateReturn::NoSuchRule;
        };
        
        (rule, AlwaysRuleContext::UserProfileScreenRegulation { user_profile_id, user_profile })
      }
    };

    let RuleEnabler::CountdownAfterPlea(conditional) = &mut rule.enabler else {
      return ProtectorCountdownAfterPleaDeactivateReturn::WrongEnablerType;
    };

    let now = daemon.state.monotonic_clock.now();
    if conditional.is_deactivated(now) {
      return ProtectorCountdownAfterPleaDeactivateReturn::AlreadyDeactivated;
    }

    let deactivating_state = conditional.create_deactivating_state(now);

    if let Err(error) = always_rule_table::enabler_countdown_after_plea_deactivate(
      &daemon.database, 
      &self.rule_id, 
      &deactivating_state,
      textual_error,
    ) {
      return match error {
        always_rule_table::EnablerCountdownAfterPleaDeactivate::NoSuchRule => {
          ProtectorCountdownAfterPleaDeactivateReturn::NoSuchRule
        }
        always_rule_table::EnablerCountdownAfterPleaDeactivate::Other => {
          ProtectorCountdownAfterPleaDeactivateReturn::InternalError
        }
      }
    }

    conditional.deactivate(now);
    ProtectorCountdownAfterPleaDeactivateReturn::Success
  }
}
