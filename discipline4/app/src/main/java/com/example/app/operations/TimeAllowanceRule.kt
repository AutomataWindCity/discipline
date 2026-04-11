package com.example.app.procedures.timeallowancerule

import com.example.app.*

sealed class CreateReturn {
  class TooManyRules() : CreateReturn() {}
  class DuplicateRuleId() : CreateReturn() {}
  class Database(val error: Throwable) : CreateReturn() {}
  class Success(val id: UuidV4, val rule: TimeAllowanceRule) : CreateReturn() {}
}

fun create(
  database: DatabaseConnection,
  adapter: TimeAllowanceRuleDbAdapter,
  location: TimeRangeRuleLocation,
  rules: TimeAllowanceRules,
  stats: RulesStats,
  optionalRuleId: UuidV4?,
  totalAllowance: Duration,
  enablerCreator: RuleEnabler.Creator,
): CreateReturn {
  if (stats.isFull()) {
    return CreateReturn.TooManyRules()
  }

  val ruleId = optionalRuleId ?: UuidV4.generateOrThrow()
  if (rules.has(id)) {
    return CreateReturn.DuplicateRuleId()
  }

  val rule = TimeAllowanceRule(
    enabler = enablerCreator.create(),
    allowance = totalAllowance,
  )

  try {
    adapter.createOrThrow(database, location, ruleId, rule)
  } catch (exception: Throwable) {
    return CreateReturn.Database(exception)
  }

  rules.add(id, rule)
  stats.updateAfterTimeAllowanceRuleCreated()
  return CreateReturn.Success(id, rule)
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
  location: Location,
  rules: TimeAllowanceRules,
  stats: RulesStats,
  ruleId: UuidV4,
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