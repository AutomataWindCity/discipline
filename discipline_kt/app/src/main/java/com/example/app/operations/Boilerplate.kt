package com.example.app

import com.example.app.database.*

sealed class CountdownConditionalLocation {
  class MainUserProfileScreenRegulationAlwaysRuleEnabler(val ruleId: AlwaysRuleId) : CountdownConditionalLocation() {}
  class MainUserProfileScreenRegulationTimeRangeRuleEnabler(val ruleId: TimeRangeRuleId) : CountdownConditionalLocation() {}
  class MainUserProfileScreenRegulationDailyTimeAllowanceRuleEnabler(val ruleId: TimeAllowanceRuleId) : CountdownConditionalLocation() {}

  class MainUserProfileApplicationRegulationAlwaysRuleEnabler(val ruleId: AlwaysRuleId, val applicationName: ApplicationName) : CountdownConditionalLocation() {}
  class MainUserProfileApplicationRegulationTimeRangeRuleEnabler(val ruleId: TimeRangeRuleId, val applicationName: ApplicationName) : CountdownConditionalLocation() {}
  class MainUserProfileApplicationRegulationDailyTimeAllowanceRuleEnabler(val ruleId: TimeAllowanceRuleId, val applicationName: ApplicationName) : CountdownConditionalLocation() {}

  class MainUserProfileVaultProtector(val vaultId: UuidV4) : CountdownConditionalLocation() {}
}

sealed class CountdownConditionalLocateError {
  class NoSuchApplicationRegulation() : CountdownConditionalLocateError() {}
  class NoSuchRule() : CountdownConditionalLocateError() {}
  class WrongRuleEnablerType() : CountdownConditionalLocateError() {}
  class WrongVaultProtectorType() : CountdownConditionalLocateError() {}
}

suspend fun reactivateCountdown() {}

sealed class CountdownAfterPleaConditionalLocation {
  class MainUserProfileScreenRegulationAlwaysRuleEnabler(val ruleId: AlwaysRuleId) : CountdownAfterPleaConditionalLocation() {}
  class MainUserProfileScreenRegulationTimeRangeRuleEnabler(val ruleId: TimeRangeRuleId) : CountdownAfterPleaConditionalLocation() {}
  class MainUserProfileScreenRegulationDailyTimeAllowanceRuleEnabler(val ruleId: TimeAllowanceRuleId) : CountdownAfterPleaConditionalLocation() {}

  class MainUserProfileApplicationRegulationAlwaysRuleEnabler(val ruleId: AlwaysRuleId, val applicationName: ApplicationName) : CountdownAfterPleaConditionalLocation() {}
  class MainUserProfileApplicationRegulationTimeRangeRuleEnabler(val ruleId: TimeRangeRuleId, val applicationName: ApplicationName) : CountdownAfterPleaConditionalLocation() {}
  class MainUserProfileApplicationRegulationDailyTimeAllowanceRuleEnabler(val ruleId: TimeAllowanceRuleId, val applicationName: ApplicationName) : CountdownAfterPleaConditionalLocation() {}

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
  class MainUserProfileApplicationRegulation(val regulationId: ApplicationRegulationId, val applicationName: ApplicationName):  AlwaysRuleLocation() {}
}

sealed class AlwaysRuleLocateError {
  class NoSuchApplicationRegulation() : AlwaysRuleLocateError() {}
}

sealed class TimeRangeRuleLocateError {
  class NoSuchApplicationRegulation() : TimeRangeRuleLocateError() {}
}

sealed class TimeRangeRuleLocation {
  class MainUserProfileScreenRegulation(): TimeRangeRuleLocation() {}
  class MainUserProfileApplicationRegulation(val regulationId: ApplicationRegulationId, val applicationName: ApplicationName): TimeRangeRuleLocation() {}
}

sealed class TimeAllowanceRuleLocateError {
  class NoSuchApplicationRegulation() : TimeAllowanceRuleLocateError() {}
}

sealed class TimeAllowanceRuleLocation {
  class MainUserProfileScreenRegulationDailyTimeAllowance(val regulationId: ApplicationRegulationId) : TimeAllowanceRuleLocation() {}
  class MainUserProfileApplicationRegulationDailyTimeAllowance(val regulationId: ApplicationRegulationId, val applicationName: ApplicationName) : TimeAllowanceRuleLocation() {}
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

// object Procedures {
//   suspend fun createApplicationRegulation(
//     database: DatabaseConnection,
//     state: State,
//     location: ApplicationRegulationLocation, 
//     applicationName: ApplicationName,
//   ): com.example.app.procedures.applicationregulation.CreateReturn {
//     return when (location) {
//       is ApplicationRegulationLocation.MainUserProfile -> {
//         com.example.app.procedures.applicationregulation.create(
//           database,
//           ApplicationRegulationDbAdapter,
//           location,
//           state.mainUserProfile.applicationRegulations,
//           state.applicationRegulationsStats,
//           applicationName,
//         )
//       }
//     }
//   }

//   suspend fun deleteApplicationRegulation(
//     database: DatabaseConnection,
//     state: State,
//     location: ApplicationRegulationLocation, 
//     regulationId: ApplicationRegulationId,
//   ) {
//     when (location) {
//       is ApplicationREgulationLocation.
//     }
//   }
// }