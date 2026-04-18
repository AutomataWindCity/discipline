package com.example.app

object TimeRangeRuleProcedure {
  sealed class CreateReturn {
    class TooManyRules() : CreateReturn() {}
    class Database(val error: Throwable) : CreateReturn() {}
    class Success(val id: UuidV4, val rule: TimeRangeRule) : CreateReturn() {}
  }

  fun create(
    state: State,
    database: Database,
    ruleGroupId: TimeRangeRuleGroupId,
    ruleCondition: TimeRange,
    ruleEnablerCreator: RuleEnabler.Creator
  ): CreateReturn {
    val ruleGroupInfo = state.getTimeRangeRuleGroupInfo(ruleGroupId)

    val statsPermission = state.rulesStats.mayCreateAlwaysRuleInRuleGroup(ruleGroupInfo, ruleGroupId)
    if statsPermission is RulesStatsPermission.No {
      return CreateReturn.TooManyRules(statsPermission.reason)
    }
    

    val ruleId = optionalRuleId ?: UuidV4.generateOrThrow()
    if (rules.has(ruleId)) {
      return CreateReturn.DuplicateRuleId()
    }

    val rule = TimeRangeRule.create(
      ruleCondition,
      ruleEnablerCreator.create(),
    )

    try {
      adapter.createOrThrow(database, location, ruleId, rule)
    } catch (exception: Throwable) {
      return CreateReturn.Database(exception)
    }

    rules.add(id, rule)
    stats.updateAfterTimeRangeRuleCreated()
    return CreateReturn.Success(id, rule)
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
    location: Location,
    rules: TimeRagneRules,
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
    stats.updateAfterTimeRangeRuleDeleted()
    return DeleteReturn.Success(rule)
  }
}