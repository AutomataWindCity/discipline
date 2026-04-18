package com.example.app

sealed class AtMainUserProfileScreenRegulationAddAlwaysRuleReturn {
  class TooManyRules() : AtMainUserProfileScreenRegulationAddAlwaysRuleReturn() {}
  class TooManyRulesForUserProfile() : AtMainUserProfileScreenRegulationAddAlwaysRuleReturn() {}
  class Success() : AtMainUserProfileScreenRegulationAddAlwaysRuleReturn() {}
}

fun State.atMainUserProfileScreenRegulationAddAlwaysRule(
  id: AlwaysRuleId,
  rule: AlwaysRule,
): AtMainUserProfileScreenRegulationAddAlwaysRuleReturn {
  if (rulesStats.isFull()) {
    return AtMainUserProfileScreenRegulationAddAlwaysRuleReturn.TooManyRules()
  }
  
  mainUserProfile.screenRegulation.alwaysRules.add(id, rule)
  rulesStats.updateAfterAlwaysRuleCreated()
  return AtMainUserProfileScreenRegulationAddAlwaysRuleReturn.Success()
}

sealed class AtMainUserProfileScreenRegulationRemoveAlwaysRule {
  class NoSuchRule() : AtMainUserProfileScreenRegulationRemoveAlwaysRule() {}
  class PermissionDenied() : AtMainUserProfileScreenRegulationRemoveAlwaysRule() {}
  class Success() : AtMainUserProfileScreenRegulationRemoveAlwaysRule() {}
}

fun State.atMainUserProfileScreenRegulationRemoveAlwaysRule(
  id: AlwaysRuleId,
): AtMainUserProfileScreenRegulationRemoveAlwaysRule {
  val rule = mainUserProfile.screenRegulation.alwaysRules.get(id)
    ?: return AtMainUserProfileScreenRegulationRemoveAlwaysRule.NoSuchRule()

  val now = monotonicClock.getNow()
  if (rule.isEnabled(now)) {
    return AtMainUserProfileScreenRegulationRemoveAlwaysRule.PermissionDenied()
  }

  mainUserProfile.screenRegulation.alwaysRules.remove(id) 
  rulesStats.updateAfterAlwaysRuleDeleted()
  return AtMainUserProfileScreenRegulationRemoveAlwaysRule.Success()
}

sealed class AtMainUserProfileApplicationRegulationAddAlwaysRuleReturn {
  class NoSuchApplicationRegulation() : AtMainUserProfileApplicationRegulationAddAlwaysRuleReturn() {}
  class TooManyRulesForUserProfile() : AtMainUserProfileApplicationRegulationAddAlwaysRuleReturn() {}
  class TooManyRules() : AtMainUserProfileApplicationRegulationAddAlwaysRuleReturn() {}
  class Success() : AtMainUserProfileApplicationRegulationAddAlwaysRuleReturn() {}
}

fun State.atMainUserProfileApplicationRegulationAddAlwaysRule(
  applicationName: ApplicationName,
  ruleId: AlwaysRuleId,
  rule: AlwaysRule,
): AtMainUserProfileApplicationRegulationAddAlwaysRuleReturn {
  if (rulesStats.isFull()) {
    return AtMainUserProfileApplicationRegulationAddAlwaysRuleReturn.TooManyRules()
  }
  
  val regulation = mainUserProfile.applicationRegulations.get(applicationName)
      ?: return AtMainUserProfileApplicationRegulationAddAlwaysRuleReturn.NoSuchApplicationRegulation()

    regulation.alwaysRules.add(ruleId, rule)
    rulesStats.updateAfterAlwaysRuleCreated()
    return AtMainUserProfileApplicationRegulationAddAlwaysRuleReturn.Success()
}

sealed class AtMainUserProfileApplicationRegulationRemoveAlwaysRuleReturn {
  class NoSuchApplicationRegulation() : AtMainUserProfileApplicationRegulationRemoveAlwaysRuleReturn() {}
  class NoSuchRule() : AtMainUserProfileApplicationRegulationRemoveAlwaysRuleReturn() {}
  class PermissionDenied() : AtMainUserProfileApplicationRegulationRemoveAlwaysRuleReturn() {}
  class Success() : AtMainUserProfileApplicationRegulationRemoveAlwaysRuleReturn() {}
}

fun State.atMainUserProfileApplicationRegulationRemoveAlwaysRule(
  applicationName: ApplicationName,
  id: AlwaysRuleId,
): AtMainUserProfileApplicationRegulationRemoveAlwaysRuleReturn {
  val regulation = mainUserProfile.applicationRegulations.get(applicationName)
    ?: return AtMainUserProfileApplicationRegulationRemoveAlwaysRuleReturn.NoSuchApplicationRegulation()

  val rule = regulation.alwaysRules.get(id)
    ?: return AtMainUserProfileApplicationRegulationRemoveAlwaysRuleReturn.NoSuchRule()

  val now = monotonicClock.getNow()
  if (rule.isEnabled(now)) {
    return AtMainUserProfileApplicationRegulationRemoveAlwaysRuleReturn.PermissionDenied()
  }

  mainUserProfile.screenRegulation.alwaysRules.remove(id) 
  rulesStats.updateAfterAlwaysRuleDeleted()
  return AtMainUserProfileApplicationRegulationRemoveAlwaysRuleReturn.Success()
}
package com.example.app

sealed class AddTimeRangeRuleAtMainUserProfileScreenRegulationReturn {
  class TooManyRules() : AddTimeRangeRuleAtMainUserProfileScreenRegulationReturn() {}
  class TooManyRulesForUserProfile() : AddTimeRangeRuleAtMainUserProfileScreenRegulationReturn() {}
  class Success() : AddTimeRangeRuleAtMainUserProfileScreenRegulationReturn() {}
}

fun State.atMainUserProfileScreenRegulationAddTimeRangeRule(
  id: TimeRangeRuleId,
  rule: TimeRangeRule,
): AddTimeRangeRuleAtMainUserProfileScreenRegulationReturn {
  if (rulesStats.isFull()) {
    return AddTimeRangeRuleAtMainUserProfileScreenRegulationReturn.TooManyRules()
  }
  
  mainUserProfile.screenRegulation.timeRangeRules.add(id, rule)
  rulesStats.updateAfterTimeRangeRuleCreated()
  return AddTimeRangeRuleAtMainUserProfileScreenRegulationReturn.Success()
}

sealed class AtMainUserProfileScreenRegulationRemoveTimeRangeRule {
  class NoSuchRule() : AtMainUserProfileScreenRegulationRemoveTimeRangeRule() {}
  class PermissionDenied() : AtMainUserProfileScreenRegulationRemoveTimeRangeRule() {}
  class Success() : AtMainUserProfileScreenRegulationRemoveTimeRangeRule() {}
}

fun State.atMainUserProfileScreenRegulationRemoveTimeRangeRule(
  id: TimeRangeRuleId,
): AtMainUserProfileScreenRegulationRemoveTimeRangeRule {
  val rule = mainUserProfile.screenRegulation.timeRangeRules.get(id)
    ?: return AtMainUserProfileScreenRegulationRemoveTimeRangeRule.NoSuchRule()

  val now = monotonicClock.getNow()
  if (rule.isEnabled(now)) {
    return AtMainUserProfileScreenRegulationRemoveTimeRangeRule.PermissionDenied()
  }

  mainUserProfile.screenRegulation.timeRangeRules.remove(id) 
  rulesStats.updateAfterTimeRangeRuleDeleted()
  return AtMainUserProfileScreenRegulationRemoveTimeRangeRule.Success()
}

sealed class AddTimeRangeRuleAtMainUserProfileApplicationRegulationReturn {
  class NoSuchApplicationRegulation() : AddTimeRangeRuleAtMainUserProfileApplicationRegulationReturn() {}
  class TooManyRulesForUserProfile() : AddTimeRangeRuleAtMainUserProfileApplicationRegulationReturn() {}
  class TooManyRules() : AddTimeRangeRuleAtMainUserProfileApplicationRegulationReturn() {}
  class Success() : AddTimeRangeRuleAtMainUserProfileApplicationRegulationReturn() {}
}

fun State.atMainUserProfileApplicationRegulationAddTimeRangeRule(
  applicationName: ApplicationName,
  ruleId: TimeRangeRuleId,
  rule: TimeRangeRule,
): AddTimeRangeRuleAtMainUserProfileApplicationRegulationReturn {
  if (rulesStats.isFull()) {
    return AddTimeRangeRuleAtMainUserProfileApplicationRegulationReturn.TooManyRules()
  }
  
  val regulation = mainUserProfile.applicationRegulations.get(applicationName)
    ?: return AddTimeRangeRuleAtMainUserProfileApplicationRegulationReturn.NoSuchApplicationRegulation()

  regulation.timeRangeRules.add(ruleId, rule)
  rulesStats.updateAfterTimeRangeRuleCreated()
  return AddTimeRangeRuleAtMainUserProfileApplicationRegulationReturn.Success()
}

sealed class AtMainUserProfileApplicationRegulationRemoveTimeRangeRuleReturn {
  class NoSuchApplicationRegulation() : AtMainUserProfileApplicationRegulationRemoveTimeRangeRuleReturn() {}
  class NoSuchRule() : AtMainUserProfileApplicationRegulationRemoveTimeRangeRuleReturn() {}
  class PermissionDenied() : AtMainUserProfileApplicationRegulationRemoveTimeRangeRuleReturn() {}
  class Success() : AtMainUserProfileApplicationRegulationRemoveTimeRangeRuleReturn() {}
}

fun State.atMainUserProfileApplicationRegulationRemoveTimeRangeRule(
  applicationName: ApplicationName,
  id: TimeRangeRuleId,
): AtMainUserProfileApplicationRegulationRemoveTimeRangeRuleReturn {
  val regulation = mainUserProfile.applicationRegulations.get(applicationName)
    ?: return AtMainUserProfileApplicationRegulationRemoveTimeRangeRuleReturn.NoSuchApplicationRegulation()

  val rule = regulation.timeRangeRules.get(id)
    ?: return AtMainUserProfileApplicationRegulationRemoveTimeRangeRuleReturn.NoSuchRule()

  val now = monotonicClock.getNow()
  if (rule.isEnabled(now)) {
    return AtMainUserProfileApplicationRegulationRemoveTimeRangeRuleReturn.PermissionDenied()
  }

  mainUserProfile.screenRegulation.timeRangeRules.remove(id) 
  rulesStats.updateAfterTimeRangeRuleDeleted()
  return AtMainUserProfileApplicationRegulationRemoveTimeRangeRuleReturn.Success()
}

sealed class AtMainUserProfileScreenRegulationAddDailyTimeAllowanceRuleReturn {
  class TooManyRules() : AtMainUserProfileScreenRegulationAddDailyTimeAllowanceRuleReturn() {}
  class TooManyRulesForUserProfile() : AtMainUserProfileScreenRegulationAddDailyTimeAllowanceRuleReturn() {}
  class Success() : AtMainUserProfileScreenRegulationAddDailyTimeAllowanceRuleReturn() {}
}

fun State.atMainUserProfileScreenRegulationAddDailyTimeAllowanceRule(
  id: TimeAllowanceRuleId,
  rule: TimeAllowanceRule,
): AtMainUserProfileApplicationRegulationAddDailyTimeAllowanceRuleReturn {
  if (rulesStats.isFull()) {
    return AtMainUserProfileApplicationRegulationAddDailyTimeAllowanceRuleReturn.TooManyRules()
  }
  
  mainUserProfile.screenRegulation.dailyTimeAllowanceRules.add(id, rule)
  rulesStats.updateAfterTimeAllowanceRuleCreated()
  return AtMainUserProfileApplicationRegulationAddDailyTimeAllowanceRuleReturn.Success()
}

sealed class AtMainUserProfileScreenRegulationRemoveDailyTimeAllowanceRule {
  class NoSuchRule() : AtMainUserProfileScreenRegulationRemoveDailyTimeAllowanceRule() {}
  class PermissionDenied() : AtMainUserProfileScreenRegulationRemoveDailyTimeAllowanceRule() {}
  class Success() : AtMainUserProfileScreenRegulationRemoveDailyTimeAllowanceRule() {}
}

fun State.atMainUserProfileScreenRegulationRemoveTimeAllowanceRule(
  id: TimeAllowanceRuleId,
): AtMainUserProfileScreenRegulationRemoveDailyTimeAllowanceRule {
  val rule = mainUserProfile.screenRegulation.dailyTimeAllowanceRules.get(id)
    ?: return AtMainUserProfileScreenRegulationRemoveDailyTimeAllowanceRule.NoSuchRule()

  val now = monotonicClock.getNow()
  if (rule.isEnabled(now)) {
    return AtMainUserProfileScreenRegulationRemoveDailyTimeAllowanceRule.PermissionDenied()
  }

  mainUserProfile.screenRegulation.dailyTimeAllowanceRules.remove(id) 
  rulesStats.updateAfterTimeAllowanceRuleDeleted()
  return AtMainUserProfileScreenRegulationRemoveDailyTimeAllowanceRule.Success()
}

sealed class AtMainUserProfileApplicationRegulationAddDailyTimeAllowanceRuleReturn {
  class NoSuchApplicationRegulation() : AtMainUserProfileApplicationRegulationAddDailyTimeAllowanceRuleReturn() {}
  class TooManyRulesForUserProfile() : AtMainUserProfileApplicationRegulationAddDailyTimeAllowanceRuleReturn() {}
  class TooManyRules() : AtMainUserProfileApplicationRegulationAddDailyTimeAllowanceRuleReturn() {}
  class Success() : AtMainUserProfileApplicationRegulationAddDailyTimeAllowanceRuleReturn() {}
}

fun State.atMainUserProfileApplicationRegulationAddDailyTimeAllowanceRule(
  applicationName: ApplicationName,
  ruleId: TimeAllowanceRuleId,
  rule: TimeAllowanceRule,
): AtMainUserProfileApplicationRegulationAddDailyTimeAllowanceRuleReturn {
  if (rulesStats.isFull()) {
    return AtMainUserProfileApplicationRegulationAddDailyTimeAllowanceRuleReturn.TooManyRules()
  }
  
  val regulation = mainUserProfile.applicationRegulations.get(applicationName)
      ?: return AtMainUserProfileApplicationRegulationAddDailyTimeAllowanceRuleReturn.NoSuchApplicationRegulation()

    regulation.alwaysRules.add(ruleId, rule)
    rulesStats.updateAfterTimeAllowanceRuleCreated()
    return AtMainUserProfileApplicationRegulationAddDailyTimeAllowanceRuleReturn.Success()
}

sealed class AtMainUserProfileApplicationRegulationRemoveTimeAllowanceRuleReturn {
  class NoSuchApplicationRegulation() : AtMainUserProfileApplicationRegulationRemoveTimeAllowanceRuleReturn() {}
  class NoSuchRule() : AtMainUserProfileApplicationRegulationRemoveTimeAllowanceRuleReturn() {}
  class PermissionDenied() : AtMainUserProfileApplicationRegulationRemoveTimeAllowanceRuleReturn() {}
  class Success() : AtMainUserProfileApplicationRegulationRemoveTimeAllowanceRuleReturn() {}
}

fun State.atMainUserProfileApplicationRegulationRemoveTimeAllowanceRule(
  applicationName: ApplicationName,
  id: TimeAllowanceRuleId,
): AtMainUserProfileApplicationRegulationRemoveTimeAllowanceRuleReturn {
  val regulation = mainUserProfile.applicationRegulations.get(applicationName)
    ?: return AtMainUserProfileApplicationRegulationRemoveTimeAllowanceRuleReturn.NoSuchApplicationRegulation()

  val rule = regulation.alwaysRules.get(id)
    ?: return AtMainUserProfileApplicationRegulationRemoveTimeAllowanceRuleReturn.NoSuchRule()

  val now = monotonicClock.getNow()
  if (rule.isEnabled(now)) {
    return AtMainUserProfileApplicationRegulationRemoveTimeAllowanceRuleReturn.PermissionDenied()
  }

  mainUserProfile.screenRegulation.alwaysRules.remove(id) 
  rulesStats.updateAfterTimeAllowanceRuleDeleted()
  return AtMainUserProfileApplicationRegulationRemoveTimeAllowanceRuleReturn.Success()
}