package com.example.app

import com.example.app.database.*

object TimeAllowanceRuleProcedures {
  sealed class CreateReturn {
    class PermissionError(val value: GetCreateRulePermissionError) : CreateReturn() {}
    class GetGroupInfo(val value: GetTimeAllowanceRuleGroupInfoError) : CreateReturn() {}
    class InternalError(val error: Throwable) : CreateReturn() {}
    class Success(val id: TimeAllowanceRuleId, val rule: TimeAllowanceRule) : CreateReturn() {}
  }

  fun create(
    state: State,
    database: Database,
    ruleGroupId: TimeAllowanceRuleGroupId,
    ruleEnablerCreator: RuleEnabler.Creator,
    totalAllowance: Duration,
  ): CreateReturn {
    val ruleGroupLocation = state.getTimeAllowanceRuleGroupLocation(ruleGroupId).let {
      when (it) {
        is Tried.Failure -> {
          return CreateReturn.GetGroupInfo(it.error)
        }
        is Tried.Success -> {
          it.value
        }
      }
    }

    val rule = TimeAllowanceRule.create(
      enabler = ruleEnablerCreator.create(),
      allowance = totalAllowance,
    )

    val ruleId = try {
      database.createTimeAllowanceRule(ruleGroupLocation, ruleGroupId, rule)
    } catch (exception: Throwable) {
      return CreateReturn.InternalError(exception)
    }

    state.createTimeAllowanceRuleOrNoop(ruleGroupLocation, ruleId, rule)
    return CreateReturn.Success(ruleId, rule)
  }

  sealed class DeleteReturn {
    class RuleEnabled() : DeleteReturn() {}
    class NoSuchRuleGroup(val value: GetTimeAllowanceRuleGroupInfoError) : DeleteReturn() {}
    class NoSuchRule(val value: GetTimeAllowanceRuleError) : DeleteReturn() {}
    class InternalError(val error: Throwable) : DeleteReturn() {}
    class Success(val rule: TimeAllowanceRule) : DeleteReturn() {}
  }

  fun delete(
    state: State,
    database: Database,
    ruleGroupId: TimeAllowanceRuleGroupId,
    ruleId: TimeAllowanceRuleId,
  ): DeleteReturn {
    val ruleGroupLocation = state.getTimeAllowanceRuleGroupLocation(ruleGroupId).let {
      when (it) {
        is Tried.Success -> {
          it.value
        }
        is Tried.Failure -> {
          return DeleteReturn.NoSuchRuleGroup(it.error)
        }
      }
    }

    val rule = state.getTimeAllowanceRule(ruleGroupLocation, ruleId).let {
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
      database.deleteTimeAllowanceRule(ruleGroupLocation, ruleGroupId, ruleId)
    } catch (exception: Throwable) {
      return DeleteReturn.InternalError(exception)
    }
    
    state.deleteTimeAllowanceRuleOrNoop(ruleGroupLocation, ruleId)
    return DeleteReturn.Success(rule)
  }
}