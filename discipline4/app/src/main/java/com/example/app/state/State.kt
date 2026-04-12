package com.example.app

import com.example.app.RemoveAlwaysRuleReturn

public data class State(
  val monotonicClock: MonotonicClock,
  val mainUserProfile: UserProfile,
  val rulesStats: RulesStats,
  val applicationRegulationsStats: ApplicationRegulationsStats,
) {
  companion object {
    val MONOTONIC_CLOCK_SYNCHRONIZATION_INTERVAL = Duration.fromMinutes(10).getOrThrow()

    fun createDefault(): State {
      val monotonicClock = MonotonicClock.create(MONOTONIC_CLOCK_SYNCHRONIZATION_INTERVAL)
      val mainUserProfile = UserProfile.create(monotonicClock.getNow())

      return State(
        monotonicClock,
        mainUserProfile,
        rulesStats = RulesStats(0, 100),
        applicationRegulationsStats = ApplicationRegulationsStats(0, 30)
      )
    }
  }
}

fun State.mayCreateAlwaysRule(location: AlwaysRuleLocation): Boolean {
  TODO()
}

sealed class AddAlwaysRuleAtMainUserProfileScreenRegulationReturn {
  
}

fun State.addAlwaysRuleAtMainUserProfileScreenRegulation(
  id: AlwaysRuleId,
  rule: AlwaysRule,
) {
  
}

sealed class AddAlwaysRuleReturn {
  class NoSuchApplicationRegulation() : AddAlwaysRuleReturn() {}
  class TooManyRulesForUserProfile() : AddAlwaysRuleReturn() {}
  class TooManyRules() : AddAlwaysRuleReturn() {}
  class Success() : AddAlwaysRuleReturn() {}
}

fun State.addAlwaysRule(
  location: AlwaysRuleLocation, 
  ruleId: AlwaysRuleId,
  rule: AlwaysRule,
): AddAlwaysRuleReturn {
  if (rulesStats.isFull()) {
    return AddAlwaysRuleReturn.TooManyRules()
  }

  when (location) {
    is AlwaysRuleLocation.MainUserProfileScreenRegulation -> {
      mainUserProfile.screenRegulation.alwaysRules.add(ruleId, rule)
      rulesStats.updateAfterAlwaysRuleCreated()
      return AddAlwaysRuleReturn.Success()
    }
    is AlwaysRuleLocation.MainUserProfileApplicationRegulation -> {
      val regulation = mainUserProfile.applicationRegulations.get(location.applicationName)
        ?: return AddAlwaysRuleReturn.NoSuchApplicationRegulation()

      regulation.alwaysRules.add(ruleId, rule)
      rulesStats.updateAfterAlwaysRuleCreated()
      return AddAlwaysRuleReturn.Success()
    }
  }
}

sealed class RemoveAlwaysRuleReturn {
  class NoSuchApplicationRegulation() : RemoveAlwaysRuleReturn() {}
  class NoSuchRule() : RemoveAlwaysRuleReturn() {}
  class PermissionDenied() : RemoveAlwaysRuleReturn() {}
  class Success() : RemoveAlwaysRuleReturn() {}
}

fun State.removeAlwaysRule(
  location: AlwaysRuleLocation,
  id: AlwaysRuleId,
): RemoveAlwaysRuleReturn {
  when (location) {
    is AlwaysRuleLocation.MainUserProfileScreenRegulation -> {
      val rule = mainUserProfile.screenRegulation.alwaysRules.get(id)
        ?: return RemoveAlwaysRuleReturn.NoSuchRule()

      val now = monotonicClock.getNow()
      if (rule.isEnabled(now)) {
        return RemoveAlwaysRuleReturn.PermissionDenied()
      }

      mainUserProfile.screenRegulation.alwaysRules.remove(id) 
      rulesStats.updateAfterAlwaysRuleDeleted()
      return RemoveAlwaysRuleReturn.Success()
    }
    is AlwaysRuleLocation.MainUserProfileApplicationRegulation -> {
      val regulation = mainUserProfile.applicationRegulations.get(location.applicationName)
        ?: return RemoveAlwaysRuleReturn.NoSuchApplicationRegulation()

      val rule = regulation.alwaysRules.get(id)
        ?: return RemoveAlwaysRuleReturn.NoSuchRule()

      val now = monotonicClock.getNow()
      if (rule.isEnabled(now)) {
        return RemoveAlwaysRuleReturn.PermissionDenied()
      }

      mainUserProfile.screenRegulation.alwaysRules.remove(id) 
      rulesStats.updateAfterAlwaysRuleDeleted()
      return RemoveAlwaysRuleReturn.Success()
    }
  }
}