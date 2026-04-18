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

  fun createApplicationRegulation(location: ApplicationRegulationLocation, applicationName: ApplicationName): ApplicationRegulationId {
    return withTransaction {
      ApplicationRegulationDbAdapter.createOrThrow(connection, location, applicationName)
    }
  }

  fun deleteApplicationRegulation(location: ApplicationRegulationLocation, regulationId: ApplicationRegulationId) {
    withTransaction() {
      ApplicationRegulationDbAdapter.deleteOrThrow(connection, location, regulationId)
    }
  }

  fun createAlwaysRule(location: AlwaysRuleLocation, rule: AlwaysRule): AlwaysRuleId {
    return withTransaction {
      AlwaysRuleDbAdapter.createOrThrow(connection, location, rule)
    }
  }

  fun deleteAlwaysRule(location: AlwaysRuleLocation, ruleId: AlwaysRuleId) {
    withTransaction {
      AlwaysRuleDbAdapter.deleteOrThrow(connection, location, ruleId)
    }
  }

  fun createTimeRangeRule(location: TimeRangeRuleLocation, rule: TimeRangeRule): TimeRangeRuleId {
    return withTransaction {
      TimeRangeRuleDbAdapter.createOrThrow(connection, location, rule)
    }
  }

  fun deleteTimeRangeRule(location: TimeRangeRuleLocation, ruleId: TimeRangeRuleId) {
    withTransaction {
      TimeRangeRuleDbAdapter.deleteOrThrow(connection, location, ruleId)
    }
  }

  fun createTimeAllowanceRule(location: TimeAllowanceRuleLocation, rule: TimeAllowanceRule): TimeAllowanceRuleId {
    return withTransaction {
      TimeAllowanceRuleDbAdapter.createOrThrow(connection, location, rule)
    }
  }

  fun deleteTimeAllowanceRule(location: TimeAllowanceRuleLocation, ruleId: TimeAllowanceRuleId) {
    withTransaction {
      TimeAllowanceRuleDbAdapter.deleteOrThrow(connection, location, ruleId)
    }
  }

  fun reactivateCountdownConditional(location: CountdownConditionalLocation, reactivateState: CountdownConditional.ReactivateState) {
    withTransaction {
      CountdownConditionalDbAdapter.reactivateOrThrow(connection, location, reactivateState)
    }
  }

  fun reactivateCountdownAfterPleaConditional(location: CountdownAfterPleaConditionalLocation) {
    withTransaction {
      CountdownAfterPleaConditionalDbAdapter.reactivateOrThrow(connection, location)
    }
  }

  fun reDeactivateCountdownAfterPleaConditional(location: CountdownAfterPleaConditionalLocation, reDeactivateState: CountdownAfterPleaConditional.ReDeactivateState) {
    withTransaction {
      CountdownAfterPleaConditionalDbAdapter.reDeactivateOrThrow(connection, location, reDeactivateState)
    }
  }
}