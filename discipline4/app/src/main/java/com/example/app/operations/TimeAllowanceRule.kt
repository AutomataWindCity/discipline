package com.example.app.procedures.timeallowancerule

import com.example.app.*
import com.example.app.database.*

sealed class CreateReturn {
  class TooManyRules() : CreateReturn() {}
  class DuplicateRuleId() : CreateReturn() {}
  class Database(val error: Throwable) : CreateReturn() {}
  class Success(val id: TimeAllowanceRuleId, val rule: TimeAllowanceRule) : CreateReturn() {}
}

fun create(
  database: DatabaseConnection,
  adapter: TimeAllowanceRuleDbAdapter,
  location: TimeAllowanceRuleLocation,
  rules: TimeAllowanceRules,
  stats: RulesStats,
  totalAllowance: Duration,
  enablerCreator: RuleEnabler.Creator,
): CreateReturn {
  if (stats.isFull()) {
    return CreateReturn.TooManyRules()
  }

  val rule = TimeAllowanceRule(
    enabler = enablerCreator.create(),
    allowance = totalAllowance,
  )

  val ruleId = try {
    adapter.createOrThrow(database, location, rule)
  } catch (exception: Throwable) {
    return CreateReturn.Database(exception)
  }

  rules.add(ruleId, rule)
  stats.updateAfterTimeAllowanceRuleCreated()
  return CreateReturn.Success(ruleId, rule)
}

sealed class DeleteReturn {
  class NoSuchRule() : DeleteReturn() {}
  class PermissionDenied() : DeleteReturn() {}
  class Database(val error: Throwable) : DeleteReturn() {}
  class Success(val rule: TimeAllowanceRule) : DeleteReturn() {}
}

fun delete(
  database: DatabaseConnection,
  adapter: TimeAllowanceRuleDbAdapter, 
  location: TimeAllowanceRuleLocation,
  rules: TimeAllowanceRules,
  stats: RulesStats,
  ruleId: TimeAllowanceRuleId,
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
  stats.updateAfterTimeAllowanceRuleDeleted()
  return DeleteReturn.Success(rule)
}