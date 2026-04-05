use crate::IsTextualError;
use crate::x::{AlwaysRule, CountdownAfterPleaConditional, CountdownAfterPleaConditionalState, CountdownConditional, Duration, RuleEnabler, UuidV4};
use crate::x::database::user_profile_screen_regulation_always_rules_table;
use super::*;


pub enum RuleEnablerActivate {
  Countdown,
  CountdownAfterPlea,
}

pub struct Create {
  user_profile_id: UuidV4,
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
    if daemon.state.rules_stats.rules_number >= daemon.state.rules_stats.maximum_rules_number {
      return CreateReturn::TooManyRules;
    }
    
    let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(&self.user_profile_id) else {
      return CreateReturn::NoSuchUserProfile;
    };

    if user_profile.rules_stats.rules_number >= user_profile.rules_stats.maximum_rules_number {
      return CreateReturn::TooManyRules;
    }
    
    let client_created_rule_id = self.rule_id.is_some();

    let rule_id = self.rule_id.unwrap_or_else(UuidV4::generate);
    let rule = AlwaysRule::create(self.rule_enabler.create());

    if let Err(error) = user_profile_screen_regulation_always_rules_table::insert_rule(
      &daemon.database, 
      &self.user_profile_id, 
      &rule_id, 
      &rule, 
      textual_error,
    ) {
      return match error {
        user_profile_screen_regulation_always_rules_table::InsertError::DuplicateRuleId if client_created_rule_id => {
          CreateReturn::DuplicateRuleId
        }
        user_profile_screen_regulation_always_rules_table::InsertError::DuplicateRuleId => {
          CreateReturn::InternalError
        }
        user_profile_screen_regulation_always_rules_table::InsertError::Other => {
          CreateReturn::InternalError
        }
      };
    }

    user_profile.screen_access_regulation.always_rules.rules.insert(rule_id, rule);
    user_profile.rules_stats.update_after_always_rule_created();
    daemon.state.rules_stats.update_after_always_rule_created();

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
    let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(&self.user_profile_id) else {
      return DeleteReturn::NoSuchUserProfile;
    };

    let Some(rule) = user_profile.screen_access_regulation.always_rules.rules.get(&self.rule_id) else {
      return DeleteReturn::NoSuchRule;
    };

    let now = daemon.state.monotonic_clock.now();
    if rule.is_enabled(now) {
      return DeleteReturn::PermissionDenied;
    }

    if let Err(error) = user_profile_screen_regulation_always_rules_table::delete_rule(
      &daemon.database, 
      &self.rule_id, 
      textual_error,
    ) {
      return match error {
        user_profile_screen_regulation_always_rules_table::DeleteRule::NoSuchRule => {
          DeleteReturn::NoSuchRule
        }
        user_profile_screen_regulation_always_rules_table::DeleteRule::Other => {
          DeleteReturn::InternalError
        }
      }
    }

    user_profile.screen_access_regulation.always_rules.rules.remove(&self.rule_id);
    user_profile.rules_stats.update_after_always_rule_deleted();
    daemon.state.rules_stats.update_after_always_rule_deleted();
    DeleteReturn::Success
  }
}

pub struct EnablerCountdownActivate {
  user_profile_id: UuidV4,
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
    let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id_mut(&self.user_profile_id) else {
      return EnablerCountdownActivateReturn::NoSuchUserProfile;
    };

    let Some(rule) = user_profile.screen_access_regulation.always_rules.rules.get_mut(&self.rule_id) else {
      return EnablerCountdownActivateReturn::NoSuchRule;
    };

    let RuleEnabler::Countdown(enabler) = &mut rule.enabler else {
      return EnablerCountdownActivateReturn::WrongEnablerType;
    };

    let now = daemon.state.monotonic_clock.now();

    if let Err(error) = user_profile_screen_regulation_always_rules_table::enabler_countdown_activate(
      &daemon.database, 
      &self.rule_id, 
      textual_error,
    ) {
      return match error {
        user_profile_screen_regulation_always_rules_table::EnablerCountdownActivate::NoSuchRule => {
          EnablerCountdownActivateReturn::NoSuchRule
        }
        user_profile_screen_regulation_always_rules_table::EnablerCountdownActivate::Other => {
          EnablerCountdownActivateReturn::InternalError
        }
      }
    }

    enabler.activate(now);
    EnablerCountdownActivateReturn::Success
  }
}

pub struct EnablerCountdownAfterPleaActivate {
  user_profile_id: UuidV4,
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
    let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(&self.user_profile_id) else {
      return EnablerCountdownAfterPleaActivateReturn::NoSuchUserProfile;
    };

    let Some(rule) = user_profile.screen_access_regulation.always_rules.rules.get(&self.rule_id) else {
      return EnablerCountdownAfterPleaActivateReturn::NoSuchRule;
    };

    let RuleEnabler::CountdownAfterPlea(enabler) = &mut rule.enabler else {
      return EnablerCountdownAfterPleaActivateReturn::WrongEnablerType;
    };

    let now = daemon.state.monotonic_clock.now();
    let enabler_state = enabler.get_state(now);
    if enabler_state == CountdownAfterPleaConditionalState::Active {
      return EnablerCountdownAfterPleaActivateReturn::AlreadyActive; 
    }

    if let Err(error) = user_profile_screen_regulation_always_rules_table::enaber_countdown_after_plea_activate(
      &daemon.database, 
      &self.rule_id, 
      textual_error,
    ) {
      return match error {
        user_profile_screen_regulation_always_rules_table::EnablerCountdownAfterPleaActivate::NoSuchRule => {
          EnablerCountdownAfterPleaActivateReturn::NoSuchRule
        }
        user_profile_screen_regulation_always_rules_table::EnablerCountdownAfterPleaActivate::Other => {
          EnablerCountdownAfterPleaActivateReturn::InternalError
        }
      }
    }

    enabler.activate();
    EnablerCountdownAfterPleaActivateReturn::Success
  }
}

pub struct ProtectorCountdownAfterPleaDeactivate {
  user_profile_id: UuidV4,
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
    let Some(user_profile) = daemon.state.user_profiles.get_profile_given_id(&self.user_profile_id) else {
      return ProtectorCountdownAfterPleaDeactivateReturn::NoSuchUserProfile;
    };

    let Some(rule) = user_profile.screen_access_regulation.always_rules.rules.get_mut(&self.rule_id) else {
      return ProtectorCountdownAfterPleaDeactivateReturn::NoSuchRule;
    };

    let RuleEnabler::CountdownAfterPlea(enabler) = &mut rule.enabler else {
      return ProtectorCountdownAfterPleaDeactivateReturn::WrongEnablerType;
    };

    let now = daemon.state.monotonic_clock.now();
    if enabler.is_deactivated(now) {
      return ProtectorCountdownAfterPleaDeactivateReturn::AlreadyDeactivated;
    }

    if let Err(error) = user_profile_screen_regulation_always_rules_table::enabler_countdown_after_plea_deactivate(
      &daemon.database, 
      &self.rule_id, 
      textual_error,
    ) {
      return match error {
        user_profile_screen_regulation_always_rules_table::EnablerCountdownAfterPleaDeactivate::NoSuchRule => {
          ProtectorCountdownAfterPleaDeactivateReturn::NoSuchRule
        }
        user_profile_screen_regulation_always_rules_table::EnablerCountdownAfterPleaDeactivate::Other => {
          ProtectorCountdownAfterPleaDeactivateReturn::InternalError
        }
      }
    }

    rule.enabler.enable(now);
    ProtectorCountdownAfterPleaDeactivateReturn::Success
  }
}
