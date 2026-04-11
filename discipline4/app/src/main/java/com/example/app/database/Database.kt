package com.example.app.database

import com.example.app.*

class Database(
  val connection: DatabaseConnection,
) {
  fun createAlwaysRule(location: AlwaysRuleLocation, ruleId: AlwaysRuleId, rule: AlwaysRule) {
    AlwaysRuleDbAdapter.createOrThrow(connection, location, ruleId, rule)
  }
  fun deleteAlwaysRule(location: AlwaysRuleLocation, ruleId: AlwaysRuleId) {
    AlwaysRuleDbAdapter.deleteOrThrow(connection, location, ruleId)
  }

  fun createTimeRangeRule(location: TimeRangeRuleLocation, ruleId: TimeRangeRuleId, rule: TimeRangeRule) {
    TimeRangeRuleDbAdapter.createOrThrow(connection, location, ruleId, rule)
  }
  fun deleteTimeRangeRule(location: TimeRangeRuleLocation, ruleId: TimeRangeRuleId) {
    TimeRangeRuleDbAdapter.deleteOrThrow(connection, location, ruleId)
  }

  fun createTimeAllowanceRule(location: TimeAllowanceRuleLocation, ruleId: TimeAllowanceRuleId, rule: TimeAllowanceRule) {
    TimeAllowanceRuleDbAdapter.createOrThrow(connection, location, ruleId, rule)
  }
  fun deleteTimeAllowanceRule(location: TimeAllowanceRuleLocation, ruleId: TimeAllowanceRuleId) {
    TimeAllowanceRuleDbAdapter.deleteOrThrow(connection, location, ruleId)
  }

  fun reactivateCountdownConditional(location: CountdownConditionalLocation, reactivateState: CountdownConditional.ReactivateState) {
    CountdownConditionalDbAdapter.reactivateOrThrow(connection, location, reactivateState)
  }

  fun reactivateCountdownAfterPleaConditional(location: CountdownAfterPleaConditionalLocation) {
    CountdownAfterPleaConditionalDbAdapter.reactivateOrThrow(connection, location)
  }
  fun reDeactivateCountdownAfterPleaConditional(location: CountdownAfterPleaConditionalLocation, reDeactivateState: CountdownAfterPleaConditional.ReDeactivateState) {
    CountdownAfterPleaConditionalDbAdapter.reDeactivateOrThrow(connection, location, reDeactivateState)
  }
}