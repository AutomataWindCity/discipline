package com.example.app.database

import com.example.app.*

object TimeAllowanceRuleDbAdapter {
  fun createOrThrow(
    database: DatabaseConnection,
    location: TimeAllowanceRuleLocation,
    rule: TimeAllowanceRule,
  ): TimeAllowanceRuleId {
    when (location) {
      is TimeAllowanceRuleLocation.MainUserProfileScreenRegulationDailyTimeAllowance -> {
        val ruleId = TimeAllowanceRulesTable.insertRuleOrThrow(database, rule)
        MainUserProfileScreenRegulationDailyTimeAllowanceRulesLinkingTable.insertOrThrow(database, ruleId)
        return ruleId
      }
      is TimeAllowanceRuleLocation.MainUserProfileApplicationRegulationDailyTimeAllowance -> {
        val ruleId = TimeAllowanceRulesTable.insertRuleOrThrow(database, rule)
        MainUserProfileApplicationRegulationDailyTimeAllowanceRulesLinkingTable.insertOrThrow(database, location.regulationId, ruleId)
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
        MainUserProfileScreenRegulationDailyTimeAllowanceRulesLinkingTable.insertOrThrow(database, ruleId)
      }
      is TimeAllowanceRuleLocation.MainUserProfileApplicationRegulationDailyTimeAllowance -> {
        TimeAllowanceRulesTable.deleteRuleOrThrow(database, ruleId)
        MainUserProfileApplicationRegulationDailyTimeAllowanceRulesLinkingTable.insertOrThrow(database, location.regulationId, ruleId)
      }
    }
  }
}