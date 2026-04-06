package com.example.app

sealed class CountdownConditionalLocation {
  class MainUserProfileScreenRegulationAlwaysRuleEnabler(val ruleId: UuidV4) : CountdownConditionalLocation() {}
  class MainUserProfileScreenRegulationTimeRangeRuleEnabler(val ruleId: UuidV4) : CountdownConditionalLocation() {}
  class MainUserProfileScreenRegulationDailyTimeAllowanceRuleEnabler(val ruleId: UuidV4) : CountdownConditionalLocation() {}

  class MainUserProfileApplicationRegulationAlwaysRuleEnabler(val ruleId: UuidV4) : CountdownConditionalLocation() {}
  class MainUserProfileApplicationRegulationTimeRangeRuleEnabler(val ruleId: UuidV4) : CountdownConditionalLocation() {}
  class MainUserProfileApplicationRegulationDailyTimeAllowanceRuleEnabler(val ruleId: UuidV4) : CountdownConditionalLocation() {}

  class MainUserProfileVaultProtector(val vaultId: UuidV4) : CountdownConditionalLocation() {}
}

sealed class CountdownConditionalLocateError {
  class NoSuchApplicationRegulation() : CountdownConditionalLocateError() {}
  class NoSuchRule() : CountdownConditionalLocateError() {}
  class WrongRuleEnablerType() : CountdownConditionalLocateError() {}
  class WrongVaultProtectorType() : CountdownConditionalLocateError() {}
}

sealed class CountdownAfterPleaConditionalLocation {
  class MainUserProfileScreenRegulationAlwaysRuleEnabler(val ruleId: UuidV4) : CountdownAfterPleaConditionalLocation() {}
  class MainUserProfileScreenRegulationTimeRangeRuleEnabler(val ruleId: UuidV4) : CountdownAfterPleaConditionalLocation() {}
  class MainUserProfileScreenRegulationDailyTimeAllowanceRuleEnabler(val ruleId: UuidV4) : CountdownAfterPleaConditionalLocation() {}

  class MainUserProfileApplicationRegulationAlwaysRuleEnabler(val ruleId: UuidV4) : CountdownAfterPleaConditionalLocation() {}
  class MainUserProfileApplicationRegulationTimeRangeRuleEnabler(val ruleId: UuidV4) : CountdownAfterPleaConditionalLocation() {}
  class MainUserProfileApplicationRegulationDailyTimeAllowanceRuleEnabler(val ruleId: UuidV4) : CountdownAfterPleaConditionalLocation() {}

  class MainUserProfileVaultProtector(val vaultId: UuidV4) : CountdownAfterPleaConditionalLocation() {}
}

sealed class CountdownAfterPleaConditionalLocateError {
  class NoSuchApplicationRegulation() : CountdownAfterPleaConditionalLocateError() {}
  class NoSuchRule() : CountdownAfterPleaConditionalLocateError() {}
  class WrongRuleEnablerType() : CountdownAfterPleaConditionalLocateError() {}
  class WrongVaultProtectorType() : CountdownAfterPleaConditionalLocateError() {}
}

sealed class AlwaysRuleLocation {
  class MainUserProfileScreenRegulation(): AlwaysRuleLocation() {}
  class MainUserProfileApplicationRegulation(val applicationName: ApplicationName):  AlwaysRuleLocation() {}
}

sealed class AlwaysRuleLocateError {
  class NoSuchApplicationRegulation() : AlwaysRuleLocateError() {}
}

sealed class TimeRangeRuleLocateError {
  class NoSuchApplicationRegulation() : TimeRangeRuleLocateError() {}
}

sealed class TimeRangeRuleLocation {
  class MainUserProfileScreenRegulation(): TimeRangeRuleLocation() {}
  class MainUserProfileApplicationRegulation(val applicationName: ApplicationName): TimeRangeRuleLocation() {}
}

sealed class TimeAllowanceRuleLocateError {
  class NoSuchApplicationRegulation() : TimeAllowanceRuleLocateError() {}
}

sealed class TimeAllowanceRuleLocation {
  class MainUserProfileScreenRegulationDailyTimeAllowance() : TimeAllowanceRuleLocation() {}
  class MainUserProfileApplicationRegulationDailyTimeAllowance() : TimeAllowanceRuleLocation() {}
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