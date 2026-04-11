package com.example.app.database

import com.example.app.*

object TimeRangeRuleDbAdapter {
  fun createOrThrow(
    database: DatabaseConnection,
    location: TimeRangeRuleLocation,
    ruleId: TimeRangeRuleId,
    rule: TimeRangeRule,
  ) {
    when (location) {
      is TimeRangeRuleLocation.MainUserProfileScreenRegulation -> {
        TimeRangeRulesTable.insertRuleOrThrow(database, ruleId, rule, location.locationId)
      }
      is TimeRangeRuleLocation.MainUserProfileApplicationRegulation -> {
        TimeRangeRulesTable.insertRuleOrThrow(database, ruleId, rule, location.locationId)
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