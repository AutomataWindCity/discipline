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
        val ruleId = TimeRangeRulesTable.insertRuleOrThrow(database, ruleId, rule, location.locationId)
        MainUserProfileScreenRegulationTimeRangeRulesLinkingTable.insertOrThrow(ruleId)
        return ruleId
      }
      is TimeRangeRuleLocation.MainUserProfileApplicationRegulation -> {
        val ruleId = TimeRangeRulesTable.insertRuleOrThrow(database, rule)
        MainUserProfileApplicationRegulationTimeRangeRulesLinkingTable.insertOrThrow(ruleId)
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