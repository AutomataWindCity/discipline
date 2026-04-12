package com.example.app

import com.example.app.*
import com.example.app.database.*

object AlwaysRuleProceduresCore {
  sealed class CreateReturn {
    class TooManyRules() : CreateReturn() {}
    class DuplicateRuleId() : CreateReturn() {}
    class InternalError(val error: Throwable) : CreateReturn() {}
    class Success(val id: AlwaysRuleId, val rule: AlwaysRule) : CreateReturn() {}
  }

  fun create(
    database: DatabaseConnection,
    adapter: AlwaysRuleDbAdapter,
    location: AlwaysRuleLocation,
    rules: AlwaysRules,
    stats: RulesStats,
    ruleEnablerCreator: RuleEnabler.Creator,
  ): CreateReturn {
    if (stats.isFull()) {
      return CreateReturn.TooManyRules()
    }

    val rule = AlwaysRule.create(
      enabler = ruleEnablerCreator.create()
    )

    val ruleId = try {
      adapter.createOrThrow(database, location, rule)
    } catch (exception: Throwable) {
      return CreateReturn.InternalError(exception)
    }

    rules.add(ruleId, rule)
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
    ruleId: AlwaysRuleId,
    clock: MonotonicClock,
  ): DeleteReturn {
    val rule = rules.get(ruleId)
      ?: return DeleteReturn.NoSuchRule()

    val now = clock.getNow()
    if (rule.isActive(now)) {
      return DeleteReturn.PermissionDenied()
    }

    try {
      adapter.deleteOrThrow(database, location, ruleId)
    } catch (exception: Throwable) {
      return DeleteReturn.InternalError(exception)
    }

    rules.remove(ruleId)
    stats.updateAfterAlwaysRuleDeleted()
    return DeleteReturn.Success(rule)
  }
}

object AlwaysRuleProcedures {
  sealed class CreateReturn {
    class TooManyRules() : CreateReturn() {}
    class DuplicateRuleId() : CreateReturn() {}
    class NoSuchApplicationRegulation() : CreateReturn() {}
    class InternalError(val error: Throwable) : CreateReturn() {}
    class Success(val id: AlwaysRuleId, val rule: AlwaysRule) : CreateReturn() {}
  }

  fun create(
    database: Database,
    state: State,
    location: AlwaysRuleLocation,
    ruleEnablerCreator: RuleEnabler.Creator,
  ): CreateReturn {
    val rule = AlwaysRule.create(
      enabler = ruleEnablerCreator.create()
    )

    val ruleId = try {
      database.createAlwaysRule(location, rule)
    } catch (exception: Throwable) {
      return CreateReturn.InternalError(exception)
    }

    state.addAlwaysRule(location, ruleId, rule)
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
    ruleId: AlwaysRuleId,
    clock: MonotonicClock,
  ): DeleteReturn {
    val rule = rules.get(ruleId)
      ?: return DeleteReturn.NoSuchRule()

    val now = clock.getNow()
    if (rule.isActive(now)) {
      return DeleteReturn.PermissionDenied()
    }

    try {
      adapter.deleteOrThrow(database, location, ruleId)
    } catch (exception: Throwable) {
      return DeleteReturn.InternalError(exception)
    }

    rules.remove(ruleId)
    stats.updateAfterAlwaysRuleDeleted()
    return DeleteReturn.Success(rule)
  }
}