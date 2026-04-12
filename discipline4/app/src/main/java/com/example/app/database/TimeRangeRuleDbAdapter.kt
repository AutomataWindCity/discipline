package com.example.app.database

import com.example.app.*

object TimeRangeRuleDbAdapter {
  fun createOrThrow(
    database: DatabaseConnection,
    location: TimeRangeRuleLocation,
    rule: TimeRangeRule,
  ): TimeRangeRuleId {
    when (location) {
      is TimeRangeRuleLocation.MainUserProfileScreenRegulation -> {
        val ruleId = TimeRangeRulesTable.insertRuleOrThrow(database, rule)
        MainUserProfileScreenRegulationTimeRangeRulesLinkingTable.insertOrThrow(database, ruleId)
        return ruleId
      }
      is TimeRangeRuleLocation.MainUserProfileApplicationRegulation -> {
        val ruleId = TimeRangeRulesTable.insertRuleOrThrow(database, rule)
        MainUserProfileApplicationRegulationTimeRangeRulesLinkingTable.insertOrThrow(database, location.regulationId, ruleId)
        return ruleId
      }
    }
  }

  fun deleteOrThrow(
    database: DatabaseConnection,
    location: TimeRangeRuleLocation,
    ruleId: TimeRangeRuleId,
  ) {
    when (location) {
      is TimeRangeRuleLocation.MainUserProfileScreenRegulation -> {
        TimeRangeRulesTable.deleteRuleOrThrow(database, ruleId)
      }
      is TimeRangeRuleLocation.MainUserProfileApplicationRegulation -> {
        TimeRangeRulesTable.deleteRuleOrThrow(database, ruleId)
      }
    }
  }
}