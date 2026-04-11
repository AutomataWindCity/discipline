package com.example.app.database

import com.example.app.*

object AlwaysRuleDbAdapter {
  fun createOrThrow(
    database: DatabaseConnection,
    location: AlwaysRuleLocation,
    ruleId: AlwaysRuleId,
    rule: AlwaysRule,
  ) {
    when (location) {
      is AlwaysRuleLocation.MainUserProfileScreenRegulation -> {
        AlwaysRulesTable.insertRuleOrThrow(database, ruleId, rule, location.locationId)
        // MainUserProfileScreenRegulationAlwaysRules
      }
      is AlwaysRuleLocation.MainUserProfileApplicationRegulation -> {
        AlwaysRulesTable.insertRuleOrThrow(database, ruleId, rule, location.locationId)
      }
    }
  }

  fun deleteOrThrow(
    database: DatabaseConnection,
    location: AlwaysRuleLocation,
    ruleId: AlwaysRuleId,
  ) {
    when (location) {
      is AlwaysRuleLocation.MainUserProfileScreenRegulation -> {
        AlwaysRulesTable.deleteRuleOrThrow(database, ruleId)
      }
      is AlwaysRuleLocation.MainUserProfileApplicationRegulation -> {
        AlwaysRulesTable.deleteRuleOrThrow(database, ruleId)
      }
    }
  }
}