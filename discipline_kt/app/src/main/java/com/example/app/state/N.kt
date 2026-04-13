package com.example.app

@JvmInline
value class AlwaysRuleGroupId(
  val value: Long,
)

class AlwaysRuleGroupInfo(
  val locationInfo: AlwaysRuleLocation,
)

class AlwaysRuleGroupInfoMap(
  val map: MutableMap<AlwaysRuleGroupId, AlwaysRuleGroupInfo>,
)

sealed class GetAlwaysRuleGroupInfoError {
  class NoGroupWithGivenId() : GetAlwaysRuleGroupInfoError() {}
}

sealed class GetAlwaysRuleError {
  class NoSuchRule() : GetAlwaysRuleError() {}
  class NoSuchApplicationRegulation() : GetAlwaysRuleError() {}
}

@JvmInline
value class TimeRangeRuleGroupId(
  val value: Long,
)

class TimeRangeRuleGroupInfo(
  val locationInfo: TimeRangeRuleLocation,
) 

class TimeRangeRuleGroupInfoMap(
  val map: MutableMap<TimeRangeRuleGroupId, TimeRangeRuleGroupInfo>,
)

sealed class GetTimeRangeRuleGroupInfoError {
  class NoGroupWithGivenId() : GetTimeRangeRuleGroupInfoError() {}
}

sealed class GetTimeRangeRuleError {
  class NoSuchRule() : GetTimeRangeRuleError() {}
  class NoSuchApplicationRegulation() : GetTimeRangeRuleError() {}
}

@JvmInline
value class TimeAllowanceRuleGroupId(
  val value: Long,
)

class TimeAllowanceRuleGroupInfo(
  val locationInfo: TimeAllowanceRuleLocation,
)

class TimeAllowanceRuleGroupInfoMap(
  val map: MutableMap<TimeAllowanceRuleGroupId, TimeAllowanceRuleGroupInfo>,
)

sealed class GetTimeAllowanceRuleGroupInfoError {
  class NoGroupWithGivenId() : GetTimeAllowanceRuleGroupInfoError() {}
}

sealed class GetTimeAllowanceRuleError {
  class NoSuchRule() : GetTimeAllowanceRuleError() {}
  class NoSuchApplicationRegulation() : GetTimeAllowanceRuleError() {}
}

sealed class GetCreateRulePermissionError() {
  class TooManyRulesInUserProfile() : GetCreateRulePermissionError() {}
  class TooManyRules() : GetCreateRulePermissionError() {}
}

sealed class GetDeleteRulePermissionError() {
  class RuleEnabled() : GetDeleteRulePermissionError() {}
}

fun State.getMonotonicNow(): Instant {
  return monotonicClock.getNow()
}

fun State.getAlwaysRuleGroupInfo(ruleGroupId: AlwaysRuleGroupId): Tried<AlwaysRuleGroupInfo, GetAlwaysRuleGroupInfoError> {
  val it = alwaysRuleGroupInfoMap.map.get(ruleGroupId)
  if (it !== null) {
    return Tried.success(it)
  } else {
    return Tried.failure(GetAlwaysRuleGroupInfoError.NoGroupWithGivenId())
  }
}

fun State.getAlwaysRuleGroupLocation(ruleGroupId: AlwaysRuleGroupId): Tried<AlwaysRuleLocation, GetAlwaysRuleGroupInfoError> {
  val it = alwaysRuleGroupInfoMap.map.get(ruleGroupId)
  if (it !== null) {
    return Tried.success(it.locationInfo)
  } else {
    return Tried.failure(GetAlwaysRuleGroupInfoError.NoGroupWithGivenId())
  }
}

fun State.getCreateAlwaysRuleInRuleGroupPermission(_ruleGroupInfo: AlwaysRuleGroupInfo): Tried<Unit, GetCreateRulePermissionError> {
  return if (rulesStats.isFull()) {
    Tried.failure(GetCreateRulePermissionError.TooManyRules())
  } else {
    Tried.success(Unit)
  }
}

fun State.getDeleteAlwaysRuleInRuleGroupPermission(
  _ruleGroupInfo: AlwaysRuleGroupInfo,
  rule: AlwaysRule,
): Tried<Unit, GetCreateRulePermissionError> {
  return if (rulesStats.isFull()) {
    Tried.failure(GetCreateRulePermissionError.TooManyRules())
  } else {
    Tried.success(Unit)
  }
}

fun State.getAlwaysRule(ruleGroupLocation: AlwaysRuleLocation, ruleId: AlwaysRuleId): Tried<AlwaysRule, GetAlwaysRuleError> {
  when (ruleGroupLocation) {
    is AlwaysRuleLocation.MainUserProfileScreenRegulation -> {
      val rule = mainUserProfile.screenRegulation.alwaysRules.get(ruleId)
      if (rule !== null) {
        return Tried.success(rule)
      } else {
        return Tried.failure(GetAlwaysRuleError.NoSuchRule())
      }
    }

    is AlwaysRuleLocation.MainUserProfileApplicationRegulation -> {
      val regulation = mainUserProfile.applicationRegulations.get(ruleGroupLocation.applicationName)
      if (regulation === null) {
        return Tried.failure(GetAlwaysRuleError.NoSuchApplicationRegulation())
      }

      val rule = regulation.alwaysRules.get(ruleId)
      if (rule !== null) {
        return Tried.success(rule)
      } else {
        return Tried.failure(GetAlwaysRuleError.NoSuchRule())
      }
    }
  }
}

fun State.createAlwaysRuleOrNoop(
  ruleGroupLocator: AlwaysRuleLocation,
  ruleId: AlwaysRuleId,
  rule: AlwaysRule,
) {
  when (ruleGroupLocator) {
    is AlwaysRuleLocation.MainUserProfileScreenRegulation -> {
      mainUserProfile.screenRegulation.alwaysRules.add(ruleId, rule)
      rulesStats.updateAfterAlwaysRuleCreated()
    }
    is AlwaysRuleLocation.MainUserProfileApplicationRegulation -> {
      val regulation = mainUserProfile.applicationRegulations.get(ruleGroupLocator.applicationName)
        ?: return

      regulation.alwaysRules.add(ruleId, rule)
      rulesStats.updateAfterAlwaysRuleCreated()
    }
  }
}

fun State.deleteAlwaysRuleOrNoop(ruleGroupLocation: AlwaysRuleLocation, ruleId: AlwaysRuleId) {
  rulesStats.updateAfterAlwaysRuleDeleted()

  when (ruleGroupLocation) {
    is AlwaysRuleLocation.MainUserProfileScreenRegulation -> {
      mainUserProfile.screenRegulation.alwaysRules.remove(ruleId)
    }
    is AlwaysRuleLocation.MainUserProfileApplicationRegulation -> {
      mainUserProfile
        .applicationRegulations
        .get(ruleGroupLocation.applicationName)
        ?.let { it.alwaysRules.remove(ruleId) }
    }
  }
}

/////////////////////////////////

fun State.getTimeRangeRuleGroupInfo(ruleGroupId: TimeRangeRuleGroupId): Tried<TimeRangeRuleGroupInfo, GetTimeRangeRuleGroupInfoError> {
  val it = timeRangeRuleGroupInfoMap.map.get(ruleGroupId)
  if (it !== null) {
    return Tried.success(it)
  } else {
    return Tried.failure(GetTimeRangeRuleGroupInfoError.NoGroupWithGivenId())
  }
}

fun State.getTimeRangeRuleGroupLocation(ruleGroupId: TimeRangeRuleGroupId): Tried<TimeRangeRuleLocation, GetTimeRangeRuleGroupInfoError> {
  val it = timeRangeRuleGroupInfoMap.map.get(ruleGroupId)
  if (it !== null) {
    return Tried.success(it.locationInfo)
  } else {
    return Tried.failure(GetTimeRangeRuleGroupInfoError.NoGroupWithGivenId())
  }
}

fun State.getCreateTimeRangeRuleInRuleGroupPermission(_ruleGroupInfo: TimeRangeRuleGroupInfo): Tried<Unit, GetCreateRulePermissionError> {
  return if (rulesStats.isFull()) {
    Tried.failure(GetCreateRulePermissionError.TooManyRules())
  } else {
    Tried.success(Unit)
  }
}

fun State.getDeleteTimeRangeRuleInRuleGroupPermission(
  _ruleGroupInfo: TimeRangeRuleGroupInfo,
  rule: TimeRangeRule,
): Tried<Unit, GetCreateRulePermissionError> {
  return if (rulesStats.isFull()) {
    Tried.failure(GetCreateRulePermissionError.TooManyRules())
  } else {
    Tried.success(Unit)
  }
}

fun State.getTimeRangeRule(ruleGroupLocation: TimeRangeRuleLocation, ruleId: TimeRangeRuleId): Tried<TimeRangeRule, GetTimeRangeRuleError> {
  when (ruleGroupLocation) {
    is TimeRangeRuleLocation.MainUserProfileScreenRegulation -> {
      val rule = mainUserProfile.screenRegulation.timeRangeRules.get(ruleId)
      if (rule !== null) {
        return Tried.success(rule)
      } else {
        return Tried.failure(GetTimeRangeRuleError.NoSuchRule())
      }
    }

    is TimeRangeRuleLocation.MainUserProfileApplicationRegulation -> {
      val regulation = mainUserProfile.applicationRegulations.get(ruleGroupLocation.applicationName)
      if (regulation === null) {
        return Tried.failure(GetTimeRangeRuleError.NoSuchApplicationRegulation())
      }

      val rule = regulation.timeRangeRules.get(ruleId)
      if (rule !== null) {
        return Tried.success(rule)
      } else {
        return Tried.failure(GetTimeRangeRuleError.NoSuchRule())
      }
    }
  }
}

fun State.createTimeRangeRuleOrNoop(
  ruleGroupLocator: TimeRangeRuleLocation,
  ruleId: TimeRangeRuleId,
  rule: TimeRangeRule,
) {
  when (ruleGroupLocator) {
    is TimeRangeRuleLocation.MainUserProfileScreenRegulation -> {
      mainUserProfile.screenRegulation.timeRangeRules.add(ruleId, rule)
      rulesStats.updateAfterTimeRangeRuleCreated()
    }
    is TimeRangeRuleLocation.MainUserProfileApplicationRegulation -> {
      val regulation = mainUserProfile.applicationRegulations.get(ruleGroupLocator.applicationName)
        ?: return

      regulation.timeRangeRules.add(ruleId, rule)
      rulesStats.updateAfterTimeRangeRuleCreated()
    }
  }
}

fun State.deleteTimeRangeRuleOrNoop(ruleGroupLocation: TimeRangeRuleLocation, ruleId: TimeRangeRuleId) {
  rulesStats.updateAfterTimeRangeRuleDeleted()
  
  when (ruleGroupLocation) {
    is TimeRangeRuleLocation.MainUserProfileScreenRegulation -> {
      mainUserProfile.screenRegulation.timeRangeRules.remove(ruleId)
    }
    is TimeRangeRuleLocation.MainUserProfileApplicationRegulation -> {
      mainUserProfile
        .applicationRegulations
        .get(ruleGroupLocation.applicationName)
        ?.let { it.timeRangeRules.remove(ruleId) }
    }
  }
}

///////////////////////////////////////

fun State.getTimeAllowanceRuleGroupInfo(ruleGroupId: TimeAllowanceRuleGroupId): Tried<TimeAllowanceRuleGroupInfo, GetTimeAllowanceRuleGroupInfoError> {
  val it = timeAllowanceRuleGroupInfoMap.map.get(ruleGroupId)
  if (it !== null) {
    return Tried.success(it)
  } else {
    return Tried.failure(GetTimeAllowanceRuleGroupInfoError.NoGroupWithGivenId())
  }
}

fun State.getTimeAllowanceRuleGroupLocation(ruleGroupId: TimeAllowanceRuleGroupId): Tried<TimeAllowanceRuleLocation, GetTimeAllowanceRuleGroupInfoError> {
  val it = timeAllowanceRuleGroupInfoMap.map.get(ruleGroupId)
  if (it !== null) {
    return Tried.success(it.locationInfo)
  } else {
    return Tried.failure(GetTimeAllowanceRuleGroupInfoError.NoGroupWithGivenId())
  }
}

fun State.getCreateTimeAllowanceRuleInRuleGroupPermission(_ruleGroupInfo: TimeAllowanceRuleGroupInfo): Tried<Unit, GetCreateRulePermissionError> {
  return if (rulesStats.isFull()) {
    Tried.failure(GetCreateRulePermissionError.TooManyRules())
  } else {
    Tried.success(Unit)
  }
}

fun State.getDeleteTimeAllowanceRuleInRuleGroupPermission(
  _ruleGroupInfo: TimeAllowanceRuleGroupInfo,
  rule: TimeAllowanceRule,
): Tried<Unit, GetCreateRulePermissionError> {
  return if (rulesStats.isFull()) {
    Tried.failure(GetCreateRulePermissionError.TooManyRules())
  } else {
    Tried.success(Unit)
  }
}

fun State.getTimeAllowanceRule(ruleGroupLocation: TimeAllowanceRuleLocation, ruleId: TimeAllowanceRuleId): Tried<TimeAllowanceRule, GetTimeAllowanceRuleError> {
  when (ruleGroupLocation) {
    is TimeAllowanceRuleLocation.MainUserProfileScreenRegulationDailyTimeAllowance -> {
      val rule = mainUserProfile.screenRegulation.dailyTimeAllowanceRules.get(ruleId)
      if (rule !== null) {
        return Tried.success(rule)
      } else {
        return Tried.failure(GetTimeAllowanceRuleError.NoSuchRule())
      }
    }

    is TimeAllowanceRuleLocation.MainUserProfileApplicationRegulationDailyTimeAllowance -> {
      val regulation = mainUserProfile.applicationRegulations.get(ruleGroupLocation.applicationName)
      if (regulation === null) {
        return Tried.failure(GetTimeAllowanceRuleError.NoSuchApplicationRegulation())
      }

      val rule = regulation.dailyTimeAllowanceRules.get(ruleId)
      if (rule !== null) {
        return Tried.success(rule)
      } else {
        return Tried.failure(GetTimeAllowanceRuleError.NoSuchRule())
      }
    }
  }
}

fun State.createTimeAllowanceRuleOrNoop(
  ruleGroupLocator: TimeAllowanceRuleLocation,
  ruleId: TimeAllowanceRuleId,
  rule: TimeAllowanceRule,
) {
  when (ruleGroupLocator) {
    is TimeAllowanceRuleLocation.MainUserProfileScreenRegulationDailyTimeAllowance -> {
      mainUserProfile.screenRegulation.dailyTimeAllowanceRules.add(ruleId, rule)
      rulesStats.updateAfterTimeAllowanceRuleCreated()
    }
    is TimeAllowanceRuleLocation.MainUserProfileApplicationRegulationDailyTimeAllowance -> {
      val regulation = mainUserProfile.applicationRegulations.get(ruleGroupLocator.applicationName)
        ?: return

      regulation.dailyTimeAllowanceRules.add(ruleId, rule)
      rulesStats.updateAfterTimeAllowanceRuleCreated()
    }
  }
}

fun State.deleteTimeAllowanceRuleOrNoop(ruleGroupLocation: TimeAllowanceRuleLocation, ruleId: TimeAllowanceRuleId) {
  rulesStats.updateAfterTimeAllowanceRuleDeleted()
  
  when (ruleGroupLocation) {
    is TimeAllowanceRuleLocation.MainUserProfileScreenRegulationDailyTimeAllowance -> {
      mainUserProfile.screenRegulation.dailyTimeAllowanceRules.remove(ruleId)
    }
    is TimeAllowanceRuleLocation.MainUserProfileApplicationRegulationDailyTimeAllowance -> {
      mainUserProfile
        .applicationRegulations
        .get(ruleGroupLocation.applicationName)
        ?.let { it.dailyTimeAllowanceRules.remove(ruleId) }
    }
  }
}

//////////////////

sealed class GetCountdownConditionalError {
  class NotFound() : GetCountdownConditionalError() {}
}

fun State.getCountdownConditional(
  locator: CountdownConditionalLocation,
): Tried<CountdownConditional, GetCountdownConditionalError> {
  TODO()
}

////////////

sealed class GetCountdownAfterPleaConditionalError {
  class NotFound() : GetCountdownAfterPleaConditionalError() {}
}

fun State.getCountdownAfterPleaConditional(
  locator: CountdownAfterPleaConditionalLocation,
): Tried<CountdownAfterPleaConditional, GetCountdownAfterPleaConditionalError> {
  TODO()
}