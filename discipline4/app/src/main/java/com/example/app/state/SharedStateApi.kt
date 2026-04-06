package com.example.app

import com.example.app.UuidV4
import com.example.app.State
import com.example.app.Tried
import com.example.app.success
import com.example.app.failure
import com.example.app.TimeRangeRuleOperation
import com.example.app.AlwaysRuleOperation
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

class Database {
  fun addAlwaysRuleOrThrow(id: UuidV4, rule: AlwaysRule, locator: AlwaysRuleOperation.Location) {}
  fun deleteAlwaysRuleOrThrow(id: UuidV4, locator: AlwaysRuleOperation.Location) {}
  fun alwaysRuleEnablerCountdownReactivateOrThrow(
    ruleId: UuidV4,
    ruleLocation: AlwaysRuleOperation.Location,
    reactivateState: CountdownConditional.ReactivateState
  ) {}
  fun alwaysRuleEnablerCountdownAfterPleaReactivateOrThrow(
    ruleId: UuidV4,
    ruleLocation: AlwaysRuleOperation.Location,
  ) {}
  fun alwaysRuleEnablerCountdownAfterPleaReDeactivateOrThrow(
    ruleId: UuidV4,
    ruleLocation: AlwaysRuleOperation.Location,
    reDeactivateState: CountdownAfterPleaConditional.ReDeactivateState
  ) {}

  fun addTimeRangeRuleOrThrow(id: UuidV4, rule: TimeRangeRule, locator: TimeRangeRuleOperation.Location) {}
  fun deleteTimeRangeRuleOrThrow(id: UuidV4, locator: TimeRangeRuleOperation.Location) {}
  
  fun addTimeAllowanceRuleOrThrow(id: UuidV4, rule: TimeAllowanceRule, locator: TimeAllowanceRuleOperation.Location) {}
  fun deleteTimeAllowanceRuleOrThrow(id: UuidV4, locator: TimeAllowanceRuleOperation.Location) {}

  fun addApplicationRegulationOrThrow(app: ApplicationName, rule: ApplicationRegulation, locator: ApplicationRegulationOperation.Location) {}
  fun deleteApplicationRegulationOrThrow(app: ApplicationName, locator: ApplicationRegulationOperation.Location) {}

  fun ruleEnablerCountdownReactivateOrThrow(
    location: RuleEnablerCountdownOperation.Location,
    reactivateState: CountdownConditional.ReactivateState
  ) {}

  fun ruleEnablerCountdownAfterPleaReactivateOrThrow(
    location: RuleEnablerCountdownAfterPleaOperation.Location,
  ) {}
  
  fun ruleEnablerCountdownAfterPleaReDeactivateOrThrow(
    location: RuleEnablerCountdownAfterPleaOperation.Location,
    reDeactivateState: CountdownAfterPleaConditional.ReDeactivateState
  ) {}
}



// sealed class TimeRangeRuleLocation {
//   class MainUserScreen : TimeRangeRuleLocation() {}
//   class MainUserApplicationRegulation(val app: ApplicationName) : TimeRangeRuleLocation() {}
// }

// sealed class TimeRangeRuleContext {
//   class MainUserScreen : TimeRangeRuleContext() {}
//   class MainUserApplicationRegulation(val app: ApplicationName, val appRule: ApplicationRegulation) : TimeRangeRuleContext() {}
// }

// sealed class TimeAllowanceRuleLocation {
//   class MainUserScreenDaily : TimeAllowanceRuleLocation() {}
//   class MainUserApplicationRegulationDaily(val app: ApplicationName) : TimeAllowanceRuleLocation() {}
// }

// sealed class TimeAllowanceRuleContext {
//   class MainUserScreenDaily : TimeAllowanceRuleContext() {}
//   class MainUserApplicationRegulationDaily(val app: ApplicationName, val appRule: ApplicationRegulation) : TimeAllowanceRuleContext() {}
// }

// sealed class AlwaysRuleError {
//   class DatabaseError(val textualError: TextualError) : AlwaysRuleError() {}
//   class DuplicateRuleId() : AlwaysRuleError() {}
//   class PermissionDenied() : AlwaysRuleError() {}
//   class NoSuchRule() : AlwaysRuleError() {}
//   class NoSuchApplicationRegulation() : AlwaysRuleError() {}
//   class TooManyRules() : AlwaysRuleError() {}
// }

public class SharedStateApi(
  val state: State,
  val database: Database,
  val mutex: Mutex,
) {
  // TODO: Create versions of the createAlwaysRuleAtMainUserProfileScreenRegulaton and deleteAlwaysRuleAtMainUserProfileScreenRegulaton (named createAlwaysRuleAtMainUserProfileApplicationRegulaton and deleteAlwaysRuleAtMainUserProfileApplicationRegulaton) functions that instead create countdown rules at the main user profile's ApplicationRegulation.AlwaysRules

  // suspend fun createAlwaysRuleAtMainUserProfileScreenRegulaton(
  //   idOrNull: UuidV4?,
  //   duration: Duration,
  // ): Tried<Pair<UuidV4, AlwaysRule>, AlwaysRuleError> {
  //   mutex.withLock() { 
  //     if (state.rulesStats.isFull()) {
  //       return Tried.failure(AlwaysRuleError.TooManyRules())
  //     }

  //     val id = idOrNull ?: UuidV4.generateOrThrow()
      
  //     if (state.mainUserProfile.screenRegulation.AlwaysRules.has(id)) {
  //       return Tried.failure(AlwaysRuleError.DuplicateRuleId())
  //     }
      
  //     val rule = AlwaysRule.create(Countdown.create(
  //       state.monotonicClock.getNow(),
  //       duration,
  //     ))

  //     try {
  //       database.addAlwaysRuleAtMainUserProfileScreenRegulatonOrThrow(id, rule)
  //     } catch (e: TextualError) {
  //       return Tried.failure(AlwaysRuleError.DatabaseError(e))
  //     }

  //     state.mainUserProfile.screenRegulation.AlwaysRules.add(id, rule)
  //     state.rulesStats.rulesNumber += 1
  //     return Tried.success(id to rule)
  //   }
  // }
  
  // suspend fun deleteAlwaysRuleAtMainUserProfileScreenRegulaton(
  //   id: UuidV4,
  // ): Tried<AlwaysRule, AlwaysRuleError> {
  //   mutex.withLock() {
  //     val rule = state.mainUserProfile.screenRegulation.AlwaysRules.get(id)
  //       ?: return Tried.failure(AlwaysRuleError.NoSuchRule())
      
  //     val now = state.monotonicClock.getNow()
  //     if (rule.isActive(now)) {
  //       return Tried.failure(AlwaysRuleError.PermissionDenied())
  //     }

  //     try {
  //       database.deleteAlwaysRuleAtMainUserProfileScreenRegulatonOrThrow(id)
  //     } catch (e: TextualError) {
  //       return Tried.failure(AlwaysRuleError.DatabaseError(e))
  //     }

  //     state.rulesStats.rulesNumber -= 1
  //     state.mainUserProfile.screenRegulation.AlwaysRules.remove(id)
  //     return Tried.success(rule)
  //   }
  // }


  // suspend fun createAlwaysRuleAtMainUserProfileApplicationRegulaton(
  //   app: ApplicationName,
  //   idOrNull: UuidV4?,
  //   duration: Duration,
  // ): Tried<Pair<UuidV4, AlwaysRule>, AlwaysRuleError> {
  //   mutex.withLock() { 
  //     if (state.rulesStats.isFull()) {
  //       return Tried.failure(AlwaysRuleError.TooManyRules())
  //     }

  //     val id = idOrNull ?: UuidV4.generateOrThrow()
  //     val applicationRegulation = state.mainUserProfile.applicationRegulations.get(app)
  //       ?: return Tried.failure(AlwaysRuleError.NoSuchApplicationRegulation())

  //     if (applicationRegulation.AlwaysRules.has(id)) {
  //       return Tried.failure(AlwaysRuleError.DuplicateRuleId())
  //     }
      
  //     val rule = AlwaysRule.create(Countdown.create(
  //       state.monotonicClock.getNow(),
  //       duration,
  //     ))

  //     try {
  //       database.addAlwaysRuleAtMainUserProfileApplicationRegulatonOrThrow(id, rule)
  //     } catch (e: TextualError) {
  //       return Tried.failure(AlwaysRuleError.DatabaseError(e))
  //     }

  //     applicationRegulation.AlwaysRules.add(id, rule)
  //     state.rulesStats.rulesNumber += 1
  //     return Tried.success(id to rule)
  //   }
  // }
  
  // suspend fun deleteAlwaysRuleAtMainUserProfileApplicationRegulaton(
  //   app: ApplicationName,
  //   id: UuidV4,
  // ): Tried<AlwaysRule, AlwaysRuleError> {
  //   mutex.withLock() {
  //     val applicationRegulation = state.mainUserProfile.applicationRegulations.get(app)
  //       ?: return Tried.failure(AlwaysRuleError.NoSuchApplicationRegulation())

  //     val rule = applicationRegulation.AlwaysRules.get(id)
  //       ?: return Tried.failure(AlwaysRuleError.NoSuchRule())
      
  //     val now = state.monotonicClock.getNow()
  //     if (rule.isActive(now)) {
  //       return Tried.failure(AlwaysRuleError.PermissionDenied())
  //     }

  //     try {
  //       database.deleteAlwaysRuleAtMainUserProfileApplicationRegulatonOrThrow(id)
  //     } catch (e: TextualError) {
  //       return Tried.failure(AlwaysRuleError.DatabaseError(e))
  //     }

  //     applicationRegulation.AlwaysRules.remove(id)
  //     state.rulesStats.rulesNumber -= 1
  //     return Tried.success(rule)
  //   }
  // }
}


// sealed class TimeAllowanceRuleOperation {
//   class Create(val idOrNull: UuidV4?, val timeAllowance: Duration, val protection: Duration) : AlwaysRuleOperation() {}
//   class Delete(val id: UuidV4) : AlwaysRuleOperation() {}
// }

// sealed class ScreenRegulationOperation {
//   class AlwaysRules(val it: AlwaysRuleOperation) : ScreenRegulationOperation() {}
//   class TimeRangeRules(val it: TimeRangeRuleOperation) : ScreenRegulationOperation() {}
//   class DailyTimeAllowanceRules(val it: TimeAllowanceRuleOperation) : ScreenRegulationOperation() {}
// }

// sealed class ApplicationRegulationOperation {
//   class AlwaysRules(val it: AlwaysRuleOperation) : ScreenRegulationOperation() {}
//   class TimeRangeRules(val it: TimeRangeRuleOperation) : ScreenRegulationOperation() {}
//   class DailyTimeAllowanceRules(val it: TimeAllowanceRuleOperation) : ScreenRegulationOperation() {}
// }

// sealed class ApplicationsRegulationOperation {
//   class Create(val app: ApplicationName) : ApplicationsRegulationOperation() {}
//   class Delete(val app: ApplicationName) : ApplicationsRegulationOperation() {}
//   class Modify(val app: ApplicationName, val it: ApplicationRegulationOperation) : ApplicationsRegulationOperation() {}
// }

// sealed class UserProfileOperation {
//   class ModifyScreenRegulation(val it: ScreenRegulationOperation) : UserProfileOperation() {}
//   class ModifyApplicationsRegulations(val it: ApplicationsRegulationOperation) : UserProfileOperation() {}
// }

// sealed class Operation {
//   class ModifyMainUserProfile(val it: UserProfileOperation) : Operation() {}
// }