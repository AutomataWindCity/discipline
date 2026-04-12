package com.example.app.procedures.timerangerule

import com.example.app.*
import com.example.app.database.*

sealed class CreateReturn {
  class TooManyRules() : CreateReturn() {}
  class DuplicateRuleId() : CreateReturn() {}
  class Database(val error: Throwable) : CreateReturn() {}
  class Success(val id: TimeRangeRuleId, val rule: TimeRangeRule) : CreateReturn() {}
}

fun create(
  database: DatabaseConnection,
  adapter: TimeRangeRuleDbAdapter,
  location: TimeRangeRuleLocation,
  rules: TimeRangeRules,
  stats: RulesStats,
  ruleCondition: TimeRange,
  ruleEnablerCreator: RuleEnabler.Creator
): CreateReturn {
  if (stats.isFull()) {
    return CreateReturn.TooManyRules()
  }

  val rule = TimeRangeRule.create(
    ruleEnablerCreator.create(),
    ruleCondition,
  )

  val ruleId = try {
    adapter.createOrThrow(database, location, rule)
  } catch (exception: Throwable) {
    return CreateReturn.Database(exception)
  }

  rules.add(ruleId, rule)
  stats.updateAfterTimeRangeRuleCreated()
  return CreateReturn.Success(ruleId, rule)
}

sealed class DeleteReturn {
  class NoSuchRule() : DeleteReturn() {}
  class PermissionDenied() : DeleteReturn() {}
  class Database(val error: Throwable) : DeleteReturn() {}
  class Success(val rule: TimeRangeRule) : DeleteReturn() {}
}

fun delete(
  database: DatabaseConnection, 
  adapter: TimeRangeRuleDbAdapter,
  location: TimeRangeRuleLocation,
  rules: TimeRangeRules,
  stats: RulesStats,
  ruleId: TimeRangeRuleId,
  clock: MonotonicClock,
): DeleteReturn {
  val rule = rules.get(ruleId)
    ?: return DeleteReturn.NoSuchRule()

  val now = clock.getNow()
  if (rule.isEnabled(now)) {
    return DeleteReturn.PermissionDenied()
  }

  try {
    adapter.deleteOrThrow(database, location, ruleId)
  } catch (exception: Throwable) {
    return DeleteReturn.Database(exception)
  }

  rules.remove(ruleId)
  stats.updateAfterTimeRangeRuleDeleted()
  return DeleteReturn.Success(rule)
}