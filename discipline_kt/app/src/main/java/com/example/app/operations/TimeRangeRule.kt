package com.example.app

import com.example.app.database.*

object TimeRangeRuleProcedures {
  sealed class CreateReturn {
    class PermissionError(val value: GetCreateRulePermissionError) : CreateReturn() {}
    class GetGroupInfo(val value: GetTimeRangeRuleGroupInfoError) : CreateReturn() {}
    class InternalError(val error: Throwable) : CreateReturn() {}
    class Success(val id: TimeRangeRuleId, val rule: TimeRangeRule) : CreateReturn() {}
  }

  fun create(
    state: State,
    database: Database,
    ruleGroupId: TimeRangeRuleGroupId,
    ruleEnablerCreator: RuleEnabler.Creator,
    ruleCondition: TimeRange,
  ): CreateReturn {
    val ruleGroupLocation = state.getTimeRangeRuleGroupLocation(ruleGroupId).let {
      when (it) {
        is Tried.Failure -> {
          return CreateReturn.GetGroupInfo(it.error)
        }
        is Tried.Success -> {
          it.value
        }
      }
    }

    val rule = TimeRangeRule.create(
      enabler = ruleEnablerCreator.create(),
      timeRange = ruleCondition,
    )

    val ruleId = try {
      database.createTimeRangeRule(ruleGroupLocation, ruleGroupId, rule)
    } catch (exception: Throwable) {
      return CreateReturn.InternalError(exception)
    }

    state.createTimeRangeRuleOrNoop(ruleGroupLocation, ruleId, rule)
    return CreateReturn.Success(ruleId, rule)
  }

  sealed class DeleteReturn {
    class RuleEnabled() : DeleteReturn() {}
    class NoSuchRuleGroup(val value: GetTimeRangeRuleGroupInfoError) : DeleteReturn() {}
    class NoSuchRule(val value: GetTimeRangeRuleError) : DeleteReturn() {}
    class InternalError(val error: Throwable) : DeleteReturn() {}
    class Success(val rule: TimeRangeRule) : DeleteReturn() {}
  }

  fun delete(
    state: State,
    database: Database,
    ruleGroupId: TimeRangeRuleGroupId,
    ruleId: TimeRangeRuleId,
  ): DeleteReturn {
    val ruleGroupLocation = state.getTimeRangeRuleGroupLocation(ruleGroupId).let {
      when (it) {
        is Tried.Success -> {
          it.value
        }
        is Tried.Failure -> {
          return DeleteReturn.NoSuchRuleGroup(it.error)
        }
      }
    }

    val rule = state.getTimeRangeRule(ruleGroupLocation, ruleId).let {
      when (it) {
        is Tried.Success -> {
          it.value
        }
        is Tried.Failure -> {
          return DeleteReturn.NoSuchRule(it.error)
        }
      }
    }

    val now = state.getMonotonicNow()
    if (rule.isEnabled(now)) {
      return DeleteReturn.RuleEnabled()
    }

    try {
      database.deleteTimeRangeRule(ruleGroupLocation, ruleGroupId, ruleId)
    } catch (exception: Throwable) {
      return DeleteReturn.InternalError(exception)
    }
    
    state.deleteTimeRangeRuleOrNoop(ruleGroupLocation, ruleId)
    return DeleteReturn.Success(rule)
  }
}