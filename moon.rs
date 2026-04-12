pub struct CachedState {
  monotonic_clock: MonotonicClock,
  user_profiles: HashMap<UserProfileId, UserProfile>,
  rules_stats: RulesStats,
  always_rules_groups_info: HashMap<AlwaysRuleGroupId, AlwaysRuleGroupInfo>,
  time_range_rules_groups_info: HashMap<TimeRangeGroupId, TimeRangeRuleGroupInfo>,
  time_allowance_rules_groups_info: HashMap<TimeAllowanceGroupId, TimeAllowanceRuleGroupInfo>,
}

impl CachedState {
  pub fn add_always_rule_or_noop(
    &mut self, 
    rule_group_info: AlwaysRuleGroupInfo,
    rule_id: AlwaysRuleId,
    rule: AlwaysRule,
  ) {
    match rule_group_info.location {
      AlwaysRuleGroupLocation::UserProfileScreenRegulation { user_profile_id } => {
        self
          .user_profiles
          .find_mut(user_profile_id)
          .map(|profile| {
            profile.screen_regulation.always_rules.add_or_noop(rule_id, rule)
          })
      }
    }
  }
}

struct AlwaysRuleGroupInfo {
  location: AlwaysRuleGroupLocation,
}

pub enum AlwaysRuleGroupLocation {
  UserProfileScreenRegulation {
    user_profile_id: UserProfileId,
  },
  UserProfileInternetRegulation {
    user_profile_id: UserProfileId,
  },
  UserProfileApplicationAccessRegulation {
    user_profile_id: UserProfileId,
    application_regulation_id: ApplicationRegulationId,
  },
  UserProfileApplicationInstallRegulation {
    user_profile_id: UserProfileId,
    application_regulation_id: ApplicationRegulationId,
  },
}

pub struct RulesStats {
  rules_number,
  maximum_rules_number,
  rules_number_per_rule_group,
  maximum_rules_number_per_rule_group,
  rules_number_per_user_profile,
  maximum_rules_number_per_user_profile,
}

pub struct Api {
  pub fn create_always_rule(
    state,
    rule_group_id,
  ) {
    let rule_group_info = state.get_always_rule_group_info();
    if state.rules_stats.may_create_always_rule_in_rule_group(rule_group_info, rule_group_id) {
      return;
    }

    let always_rule = create();

    database.create_always_rule(rule_group_id, rule_group_info, always_rule);
    state.add_always_rule(rule_group_id, rule_group_info, always_rule);
  }

  pub fn delete_always_rule(
    &mut self, 
    rule_group_id,
    rule_id,
  ) {
    let rule_group_info = state.get_always_rule_group_info(rule_group_id)

    let rule = state
      .get_always_rule_enabler(rule_group_info, rule_group_id)
      .or(|| { database.get_always_rule_enabler(rule_group_info, rule_group_id) })

    let now = state.get_monotonic_now()
    if rule.is_enabled(now) {
      return;
    }

    database.delete_always_rule(rule_group_info, rule_group_id, rule_id)
    state.delete_always_rule_or_noop(rule_group_info, rule_group_id, rule_id)
  }
}

pub struct State {
  user_profiles_by_user_name: HashMap,
  user_profiles_by_profile_id: HashMap,
  monotonic_clock: MonotonicClock,
  always_rule_groups: HashMap<AlwaysRuleGroupId, AlwaysRuleGroup>,

}

impl State {
  pub fn is_screen_restricted_for_user(&self, user_profile_id: UserProfileId) {
    let Some(user_profile) = self.user_profiles_by_profile_id.get(user_profile_id) else {
      return;
    };

    let Some(always_rules) = self.always_rule_groups.get(user_profile.always_rules_id) else {
      return;
    };

    if (always_rules.is_active()) {
      
    }
  }

  pub fn add_always_rule(
    group_id: AlwaysRuleGroupId,

  ) {

  }
}