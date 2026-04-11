package com.example.app

import com.example.app.database.RuleLocationsTable
import com.example.app.database.LocationId

sealed class CountdownConditionalLocation {
  class MainUserProfileScreenRegulationAlwaysRuleEnabler(val ruleId: AlwaysRuleId, val locationId: LocationId) : CountdownConditionalLocation() {}
  class MainUserProfileScreenRegulationTimeRangeRuleEnabler(val ruleId: TimeRangeRuleId, val locationId: LocationId) : CountdownConditionalLocation() {}
  class MainUserProfileScreenRegulationDailyTimeAllowanceRuleEnabler(val ruleId: TimeAllowanceRuleId, val locationId: LocationId) : CountdownConditionalLocation() {}

  class MainUserProfileApplicationRegulationAlwaysRuleEnabler(val ruleId: AlwaysRuleId, val locationId: LocationId) : CountdownConditionalLocation() {}
  class MainUserProfileApplicationRegulationTimeRangeRuleEnabler(val ruleId: TimeRangeRuleId, val locationId: LocationId) : CountdownConditionalLocation() {}
  class MainUserProfileApplicationRegulationDailyTimeAllowanceRuleEnabler(val ruleId: TimeAllowanceRuleId, val locationId: LocationId) : CountdownConditionalLocation() {}

  class MainUserProfileVaultProtector(val vaultId: UuidV4, val locationId: LocationId) : CountdownConditionalLocation() {}
}

sealed class CountdownConditionalLocateError {
  class NoSuchApplicationRegulation() : CountdownConditionalLocateError() {}
  class NoSuchRule() : CountdownConditionalLocateError() {}
  class WrongRuleEnablerType() : CountdownConditionalLocateError() {}
  class WrongVaultProtectorType() : CountdownConditionalLocateError() {}
}

sealed class CountdownAfterPleaConditionalLocation {
  class MainUserProfileScreenRegulationAlwaysRuleEnabler(val ruleId: AlwaysRuleId) : CountdownAfterPleaConditionalLocation() {}
  class MainUserProfileScreenRegulationTimeRangeRuleEnabler(val ruleId: TimeRangeRuleId) : CountdownAfterPleaConditionalLocation() {}
  class MainUserProfileScreenRegulationDailyTimeAllowanceRuleEnabler(val ruleId: TimeAllowanceRuleId) : CountdownAfterPleaConditionalLocation() {}

  class MainUserProfileApplicationRegulationAlwaysRuleEnabler(val ruleId: AlwaysRuleId) : CountdownAfterPleaConditionalLocation() {}
  class MainUserProfileApplicationRegulationTimeRangeRuleEnabler(val ruleId: TimeRangeRuleId) : CountdownAfterPleaConditionalLocation() {}
  class MainUserProfileApplicationRegulationDailyTimeAllowanceRuleEnabler(val ruleId: TimeAllowanceRuleId) : CountdownAfterPleaConditionalLocation() {}

  class MainUserProfileVaultProtector(val vaultId: UuidV4) : CountdownAfterPleaConditionalLocation() {}
}

sealed class CountdownAfterPleaConditionalLocateError {
  class NoSuchApplicationRegulation() : CountdownAfterPleaConditionalLocateError() {}
  class NoSuchRule() : CountdownAfterPleaConditionalLocateError() {}
  class WrongRuleEnablerType() : CountdownAfterPleaConditionalLocateError() {}
  class WrongVaultProtectorType() : CountdownAfterPleaConditionalLocateError() {}
}

sealed class AlwaysRuleLocation {
  class MainUserProfileScreenRegulation(val locationId: LocationId): AlwaysRuleLocation() {}
  class MainUserProfileApplicationRegulation(val locationId: LocationId, val applicationName: ApplicationName):  AlwaysRuleLocation() {}
}

sealed class AlwaysRuleLocateError {
  class NoSuchApplicationRegulation() : AlwaysRuleLocateError() {}
}

sealed class TimeRangeRuleLocateError {
  class NoSuchApplicationRegulation() : TimeRangeRuleLocateError() {}
}

sealed class TimeRangeRuleLocation {
  class MainUserProfileScreenRegulation(val locationId: LocationId): TimeRangeRuleLocation() {}
  class MainUserProfileApplicationRegulation(val locationId: LocationId, val applicationName: ApplicationName): TimeRangeRuleLocation() {}
}

sealed class TimeAllowanceRuleLocateError {
  class NoSuchApplicationRegulation() : TimeAllowanceRuleLocateError() {}
}

sealed class TimeAllowanceRuleLocation {
  class MainUserProfileScreenRegulationDailyTimeAllowance(val locationId: LocationId) : TimeAllowanceRuleLocation() {}
  class MainUserProfileApplicationRegulationDailyTimeAllowance(val locationId: LocationId) : TimeAllowanceRuleLocation() {}
}

sealed class ApplicationRegulationLocateError() {
  
}

sealed class ApplicationRegulationLocation() {
  class MainUserProfile() : ApplicationRegulationLocation() {}
}

sealed class VaultLocateError() {

}

sealed class VaultLocation() {
  class MainUserProfile() : VaultLocation() {}
}