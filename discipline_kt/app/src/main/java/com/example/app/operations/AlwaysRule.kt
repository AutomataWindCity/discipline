package com.example.app

import com.example.app.database.*

object AlwaysRuleProcedures {
  sealed class CreateReturn {
    class PermissionError(val value: GetCreateRulePermissionError) : CreateReturn() {}
    class GetGroupInfo(val value: GetAlwaysRuleGroupInfoError) : CreateReturn() {}
    class InternalError(val error: Throwable) : CreateReturn() {}
    class Success(val id: AlwaysRuleId, val rule: AlwaysRule) : CreateReturn() {}
  }

  fun create(
    state: State,
    database: Database,
    ruleGroupId: AlwaysRuleGroupId,
    ruleEnablerCreator: RuleEnabler.Creator,
  ): CreateReturn {
    val ruleGroupLocation = state.getAlwaysRuleGroupLocation(ruleGroupId).let {
      when (it) {
        is Tried.Failure -> {
          return CreateReturn.GetGroupInfo(it.error)
        }
        is Tried.Success -> {
          it.value
        }
      }
    }

    val rule = AlwaysRule.create(
      enabler = ruleEnablerCreator.create()
    )

    val ruleId = try {
      database.createAlwaysRule(ruleGroupLocation, ruleGroupId, rule)
    } catch (exception: Throwable) {
      return CreateReturn.InternalError(exception)
    }

    state.createAlwaysRuleOrNoop(ruleGroupLocation, ruleId, rule)
    return CreateReturn.Success(ruleId, rule)
  }

  sealed class DeleteReturn {
    class RuleEnabled() : DeleteReturn() {}
    class NoSuchRuleGroup(val value: GetAlwaysRuleGroupInfoError) : DeleteReturn() {}
    class NoSuchRule(val value: GetAlwaysRuleError) : DeleteReturn() {}
    class InternalError(val error: Throwable) : DeleteReturn() {}
    class Success(val rule: AlwaysRule) : DeleteReturn() {}
  }

  fun delete(
    state: State,
    database: Database,
    ruleGroupId: AlwaysRuleGroupId,
    ruleId: AlwaysRuleId,
  ): DeleteReturn {
    val ruleGroupLocation = state.getAlwaysRuleGroupLocation(ruleGroupId).let {
      when (it) {
        is Tried.Success -> {
          it.value
        }
        is Tried.Failure -> {
          return DeleteReturn.NoSuchRuleGroup(it.error)
        }
      }
    }

    val rule = state.getAlwaysRule(ruleGroupLocation, ruleId).let {
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
      database.deleteAlwaysRule(ruleGroupLocation, ruleGroupId, ruleId)
    } catch (exception: Throwable) {
      return DeleteReturn.InternalError(exception)
    }
    
    state.deleteAlwaysRuleOrNoop(ruleGroupLocation, ruleId)
    return DeleteReturn.Success(rule)
  }
}