package com.example.app.database

import com.example.app.*

object TimeAllowanceRuleDbAdapter {
  fun createOrThrow(
    database: DatabaseConnection,
    rule: TimeAllowanceRule,
  ): TimeAllowanceRuleId {
    when (location) {
      is TimeAllowanceRuleLocation.MainUserProfileScreenRegulationDailyTimeAllowance -> {
        val ruleId = TimeAllowanceRulesTable.insertRuleOrThrow(database, ruleId, rule, location.locationId)
        MainUserProfileScreenRegulationDailyTimeAllowanceLinkingTable.insertOrThrow(ruleId)
        return ruleId
      }
      is TimeAllowanceRuleLocation.MainUserProfileApplicationRegulationDailyTimeAllowance -> {
        val ruleId = TimeAllowanceRulesTable.insertRuleOrThrow(database, ruleId, rule, location.locationId)
        MainUserProfileApplicationRegulationDailyTimeAllowanceRulesLinkingTable.insertOrThrow(ruleId)
        return ruleId
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