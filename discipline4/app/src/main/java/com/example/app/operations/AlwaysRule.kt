package com.example.app.procedures.alwaysrule

import com.example.app.*

sealed class CreateReturn {
  class TooManyRules() : CreateReturn() {}
  class DuplicateRuleId() : CreateReturn() {}
  class InternalError(val error: Throwable) : CreateReturn() {}
  class Success(val id: UuidV4, val rule: AlwaysRule) : CreateReturn() {}
}

fun create(
  database: DatabaseConnection,
  adapter: AlwaysRuleDbAdapter,
  location: AlwaysRuleLocation,
  rules: AlwaysRules,
  stats: RulesStats,
  optionalRuleId: UuidV4?,
  ruleEnablerCreator: RuleEnabler.Creator,
): CreateReturn {
  if (stats.isFull()) {
    return CreateReturn.TooManyRules()
  }

  val ruleId = optionalRuleId ?: UuidV4.generateOrThrow()
  if (rules.has(ruleId)) {
    return CreateReturn.DuplicateRuleId()
  }

  val rule = AlwaysRule.create(
    enabler = ruleEnablerCreator.create()
  )

  try {
    adapter.createOrThrow(database, location, ruleId, rule)
  } catch (exception: Throwable) {
    return CreateReturn.InternalError(exception)
  }

  rules.add(ruledId, rule)
  stats.updateAfterAlwaysRuleCreated()
  return CreateReturn.Success(ruleId, rule)
}

sealed class DeleteReturn {
  class NoSuchRule() : DeleteReturn() {}
  class PermissionDenied() : DeleteReturn() {}
  class InternalError(val error: Throwable) : DeleteReturn() {}
  class Success(val rule: AlwaysRule) : DeleteReturn() {}
}

fun delete(
  database: DatabaseConnection,
  adapter: AlwaysRuleDbAdapter,
  location: AlwaysRuleLocation,
  rules: AlwaysRules,
  stats: RulesStats,
  ruleId: UuidV4,
  clock: MonotonicClock,
): DeleteReturn {
  val rule = rules.get(ruleId)
    ?: return DeleteReturn.NoSuchRule()

  val now = clock.getNow()
  if (rule.isActive(now)) {
    return DeleteReturn.PermissionDenied()
  }

  try {
    adapter.deleteOrThrow(database, location, ruledId, rule)
  } catch (exception: Throwable) {
    return DeleteReturn.InternalError(exception)
  }

  rules.remove(ruleId)
  stats.updateAfterAlwaysRuleDeleted()
  return DeleteReturn.Success(rule)
}
