package com.example.app.database

import com.example.app.*

object TimeAllowanceRuleDbAdapter {
  fun createOrThrow(
    database: DatabaseConnection,
    location: TimeAllowanceRuleLocation,
    ruleId: TimeAllowanceRuleId,
    rule: TimeAllowanceRule,
  ) {
    when (location) {
      is TimeAllowanceRuleLocation.MainUserProfileScreenRegulationDailyTimeAllowance -> {
        TimeAllowanceRulesTable.insertRuleOrThrow(database, ruleId, rule, location.locationId)
      }
      is TimeAllowanceRuleLocation.MainUserProfileApplicationRegulationDailyTimeAllowance -> {
        TimeAllowanceRulesTable.insertRuleOrThrow(database, ruleId, rule, location.locationId)
      }
    }
  }

  fun deleteOrThrow(
    database: DatabaseConnection,
    location: TimeAllowanceRuleLocation,
    ruleId: TimeAllowanceRuleId,
  ) {
    when (location) {
      is TimeAllowanceRuleLocation.MainUserProfileScreenRegulationDailyTimeAllowance -> {
        TimeAllowanceRulesTable.deleteRuleOrThrow(database, ruleId)
      }
      is TimeAllowanceRuleLocation.MainUserProfileApplicationRegulationDailyTimeAllowance -> {
        TimeAllowanceRulesTable.deleteRuleOrThrow(database, ruleId)
      }
    }
  }
}