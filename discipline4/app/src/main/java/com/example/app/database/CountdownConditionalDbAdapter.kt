package com.example.app.database

import com.example.app.*

object CountdownConditionalDbAdapter {
  fun reactivateOrThrow(
    database: DatabaseConnection,
    location: CountdownConditionalLocation,
    reactivateState: CountdownConditional.ReactivateState,
  ) {
    when (location) {
      is CountdownConditionalLocation.MainUserProfileScreenRegulationAlwaysRuleEnabler -> {
        AlwaysRulesTable.enablerCountdownConditionalReactivate(database, location.ruleId, reactivateState)
      }
      is CountdownConditionalLocation.MainUserProfileScreenRegulationTimeRangeRuleEnabler -> {
        TimeRangeRulesTable.enablerCountdownConditionalReactivate(database, location.ruleId, reactivateState)
      }
      is CountdownConditionalLocation.MainUserProfileScreenRegulationDailyTimeAllowanceRuleEnabler -> {
        TimeAllowanceRulesTable.enablerCountdownConditionalReactivate(database, location.ruleId, reactivateState)
      }
      is CountdownConditionalLocation.MainUserProfileApplicationRegulationAlwaysRuleEnabler -> {
        AlwaysRulesTable.enablerCountdownConditionalReactivate(database, location.ruleId, reactivateState)
      }
      is CountdownConditionalLocation.MainUserProfileApplicationRegulationTimeRangeRuleEnabler -> {
       TimeRangeRulesTable.enablerCountdownConditionalReactivate(database, location.ruleId, reactivateState)
      }
      is CountdownConditionalLocation.MainUserProfileApplicationRegulationDailyTimeAllowanceRuleEnabler -> {
        TimeAllowanceRulesTable.enablerCountdownConditionalReactivate(database, location.ruleId, reactivateState)
      }
      is CountdownConditionalLocation.MainUserProfileVaultProtector -> {

      }
    }
  }
}
