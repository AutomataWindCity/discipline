package com.example.app

import com.example.app.*
import com.example.app.database.*
import android.database.Cursor
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

public class SharedStateApi(
  val state: State,
  val database: DatabaseConnection,
  val mutex: Mutex,
) {
  suspend fun createApplicationRegulation(location: ApplicationRegulationLocation, applicationName: ApplicationName): ApplicationRegulationId {
  }
  
  suspend fun deleteApplicationRegulation(location: ApplicationRegulationLocation, regulationId: ApplicationRegulationId) {
    
  }
  suspend fun createAlwaysRule(location: AlwaysRuleLocation, rule: AlwaysRule): AlwaysRuleId {
    com.example.app.procedures.alwaysrule.create(
      database,
      AlwaysRuleDbAdapter,
      location,
    )
  }

  suspend fun deleteAlwaysRule(location: AlwaysRuleLocation, ruleId: AlwaysRuleId) {
    
  }
  suspend fun createTimeRangeRule(location: TimeRangeRuleLocation, rule: TimeRangeRule): TimeRangeRuleId {
    
  }
  suspend fun deleteTimeRangeRule(location: TimeRangeRuleLocation, ruleId: TimeRangeRuleId) {
    
  }
  suspend fun createTimeAllowanceRule(location: TimeAllowanceRuleLocation, rule: TimeAllowanceRule): TimeAllowanceRuleId {
    
  }
  suspend fun deleteTimeAllowanceRule(location: TimeAllowanceRuleLocation, ruleId: TimeAllowanceRuleId) {
    
  }
  suspend fun reactivateCountdownConditional(location: CountdownConditionalLocation, reactivateState: CountdownConditional.ReactivateState) {
    
  }
  suspend fun reactivateCountdownAfterPleaConditional(location: CountdownAfterPleaConditionalLocation) {
    
  }
  suspend fun reDeactivateCountdownAfterPleaConditional(location: CountdownAfterPleaConditionalLocation, reDeactivateState: CountdownAfterPleaConditional.ReDeactivateState) {
    
  }
}
