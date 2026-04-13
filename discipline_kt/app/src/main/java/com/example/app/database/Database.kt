package com.example.app.database

import com.example.app.*

class Database(
  val connection: DatabaseConnection,
) {
  private fun <R> withTransaction(callback: (connection: DatabaseConnection) -> R): R {
    connection.connection.beginTransaction()
    val returnValue = callback(connection)
    connection.connection.setTransactionSuccessful()
    return returnValue
  }

  fun createApplicationRegulation(
    location: ApplicationRegulationLocation, 
    applicationName: ApplicationName,
  ): ApplicationRegulationId {
    return withTransaction {
      ApplicationRegulationDbAdapter.createOrThrow(
        connection, 
        location, 
        applicationName,
      )
    }
  }

  fun deleteApplicationRegulation(
    location: ApplicationRegulationLocation, 
    applicationName: ApplicationName,
  ) {
    withTransaction() {
      ApplicationRegulationDbAdapter.deleteOrThrow(
        connection, 
        location, 
        applicationName,
      )
    }
  }

  fun createAlwaysRule(
    ruleGroupLocator: AlwaysRuleLocation, 
    ruleGroupId: AlwaysRuleGroupId, 
    rule: AlwaysRule,
  ): AlwaysRuleId {
    return withTransaction {
      AlwaysRuleDbAdapter.createOrThrow(
        connection, 
        ruleGroupLocator, 
        rule,
      )
    }
  }

  fun deleteAlwaysRule(
    ruleGroupLocation: AlwaysRuleLocation, 
    ruleGroupId: AlwaysRuleGroupId, 
    ruleId: AlwaysRuleId,
  ) {
    withTransaction {
      AlwaysRuleDbAdapter.deleteOrThrow(
        connection, 
        ruleGroupLocation, 
        ruleId,
      )
    }
  }

  fun createTimeRangeRule(
    ruleGroupLocation: TimeRangeRuleLocation, 
    ruleGroupId: TimeRangeRuleGroupId, 
    rule: TimeRangeRule,
  ): TimeRangeRuleId {
    return withTransaction {
      TimeRangeRuleDbAdapter.createOrThrow(
        connection, 
        ruleGroupLocation, 
        rule,
      )
    }
  }

  fun deleteTimeRangeRule(
    ruleGroupLocation: TimeRangeRuleLocation,
    ruleGroupId: TimeRangeRuleGroupId,
    ruleId: TimeRangeRuleId,
  ) {
    withTransaction {
      TimeRangeRuleDbAdapter.deleteOrThrow(
        connection,
        ruleGroupLocation,
        ruleId,
      )
    }
  }

  fun createTimeAllowanceRule(
    ruleGroupLocation: TimeAllowanceRuleLocation, 
    ruleGroupId: TimeAllowanceRuleGroupId, 
    rule: TimeAllowanceRule,
  ): TimeAllowanceRuleId {
    return withTransaction {
      TimeAllowanceRuleDbAdapter.createOrThrow(
        connection, 
        ruleGroupLocation, 
        rule,
      )
    }
  }

  fun deleteTimeAllowanceRule(
    ruleGroupLocation: TimeAllowanceRuleLocation,
    ruleGroupId: TimeAllowanceRuleGroupId,
    ruleId: TimeAllowanceRuleId,
  ) {
    withTransaction {
      TimeAllowanceRuleDbAdapter.deleteOrThrow(
        connection,
        ruleGroupLocation,
        ruleId,
      )
    }
  }

  fun reactivateCountdownConditional(
    location: CountdownConditionalLocation, 
    reactivateState: CountdownConditional.ReactivateState,
  ) {
    withTransaction {
      CountdownConditionalDbAdapter.reactivateOrThrow(
        connection, 
        location, 
        reactivateState,
      )
    }
  }

  fun reactivateCountdownAfterPleaConditional(
    location: CountdownAfterPleaConditionalLocation,
  ) {
    withTransaction {
      CountdownAfterPleaConditionalDbAdapter.reactivateOrThrow(
        connection, 
        location,
      )
    }
  }

  fun reDeactivateCountdownAfterPleaConditional(
    location: CountdownAfterPleaConditionalLocation, 
    reDeactivateState: CountdownAfterPleaConditional.ReDeactivateState,
  ) {
    withTransaction {
      CountdownAfterPleaConditionalDbAdapter.reDeactivateOrThrow(
        connection, 
        location, 
        reDeactivateState,
      )
    }
  }
}