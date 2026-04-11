package com.example.app.database

import com.example.app.*

object CountdownAfterPleaConditionalDbAdapter {
  fun reactivateOrThrow(
    database: DatabaseConnection,
    location: CountdownAfterPleaConditionalLocation,
  ) {
    when (location) {
      is CountdownAfterPleaConditionalLocation.MainUserProfileScreenRegulationAlwaysRuleEnabler -> {
        AlwaysRulesTable.enablerCountdownAfterPleaConditionalReactivate(database, location.ruleId)
      }
      is CountdownAfterPleaConditionalLocation.MainUserProfileScreenRegulationTimeRangeRuleEnabler -> {
        TimeRangeRulesTable.enablerCountdownAfterPleaConditionalReactivate(database, location.ruleId)
      }
      is CountdownAfterPleaConditionalLocation.MainUserProfileScreenRegulationDailyTimeAllowanceRuleEnabler -> {
        TimeAllowanceRulesTable.enablerCountdownAfterPleaConditionalReactivate(database, location.ruleId)
      }
      is CountdownAfterPleaConditionalLocation.MainUserProfileApplicationRegulationAlwaysRuleEnabler -> {
        AlwaysRulesTable.enablerCountdownAfterPleaConditionalReactivate(database, location.ruleId)
      }
      is CountdownAfterPleaConditionalLocation.MainUserProfileApplicationRegulationTimeRangeRuleEnabler -> {
        TimeRangeRulesTable.enablerCountdownAfterPleaConditionalReactivate(database, location.ruleId)
      }
      is CountdownAfterPleaConditionalLocation.MainUserProfileApplicationRegulationDailyTimeAllowanceRuleEnabler -> {
        TimeAllowanceRulesTable.enablerCountdownAfterPleaConditionalReactivate(database, location.ruleId)
      }
      is CountdownAfterPleaConditionalLocation.MainUserProfileVaultProtector -> {

      }
    }
  }

  fun reDeactivateOrThrow(
    database: DatabaseConnection,
    location: CountdownAfterPleaConditionalLocation,
    reDeactivateState: CountdownAfterPleaConditional.ReDeactivateState
  ) {
    when (location) {
      is CountdownAfterPleaConditionalLocation.MainUserProfileScreenRegulationAlwaysRuleEnabler -> {
        AlwaysRulesTable.enablerCountdownAfterPleaConditionalReDeactivate(database, location.ruleId, reDeactivateState)
      }
      is CountdownAfterPleaConditionalLocation.MainUserProfileScreenRegulationTimeRangeRuleEnabler -> {
        TimeRangeRulesTable.enablerCountdownAfterPleaConditionalReDeactivate(database, location.ruleId, reDeactivateState)
      }
      is CountdownAfterPleaConditionalLocation.MainUserProfileScreenRegulationDailyTimeAllowanceRuleEnabler -> {
        TimeAllowanceRulesTable.enablerCountdownAfterPleaConditionalReDeactivate(database, location.ruleId, reDeactivateState)
      }
      is CountdownAfterPleaConditionalLocation.MainUserProfileApplicationRegulationAlwaysRuleEnabler -> {
        AlwaysRulesTable.enablerCountdownAfterPleaConditionalReDeactivate(database, location.ruleId, reDeactivateState)
      }
      is CountdownAfterPleaConditionalLocation.MainUserProfileApplicationRegulationTimeRangeRuleEnabler -> {
        TimeRangeRulesTable.enablerCountdownAfterPleaConditionalReDeactivate(database, location.ruleId, reDeactivateState)
      }
      is CountdownAfterPleaConditionalLocation.MainUserProfileApplicationRegulationDailyTimeAllowanceRuleEnabler -> {
        TimeAllowanceRulesTable.enablerCountdownAfterPleaConditionalReDeactivate(database, location.ruleId, reDeactivateState)
      }
      is CountdownAfterPleaConditionalLocation.MainUserProfileVaultProtector -> {

      }
    }
  }
}