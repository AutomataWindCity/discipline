package com.example.app.database

import com.example.app.*

object AlwaysRuleDbAdapter {
  fun createOrThrow(
    database: DatabaseConnection,
    location: AlwaysRuleLocation,
    rule: AlwaysRule,
  ): AlwaysRuleId {
    when (location) {
      is AlwaysRuleLocation.MainUserProfileScreenRegulation -> {
        val id = AlwaysRulesTable.insertRuleOrThrow(database, rule)
        MainUserProfileScreenRegulationAlwaysRulesLinkingTable.insertOrThrow(database, id)
        return id
      }
      is AlwaysRuleLocation.MainUserProfileApplicationRegulation -> {
        val id = AlwaysRulesTable.insertRuleOrThrow(database, rule)
        MainUserProfileApplicationRegulationAlwaysRulesLinkingTable.insertOrThrow(database, location.regulationId, id)
        return id
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