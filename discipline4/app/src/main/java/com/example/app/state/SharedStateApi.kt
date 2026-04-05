package com.example.app

import com.example.app.UuidV4
import com.example.app.State
import com.example.app.Tried
import com.example.app.success
import com.example.app.failure
import com.example.app.RuleError
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

class Database {
  fun addCountdownRuleOrThrow(id: UuidV4, rule: CountdownRule, locator: CountdownRuleLocator) {}
  fun deleteCountdownRuleOrThrow(id: UuidV4, locator: CountdownRuleLocator) {}

  fun addTimeRangeRuleOrThrow(id: UuidV4, rule: TimeRangeRule, locator: TimeRangeRuleLocator) {}
  fun deleteTimeRangeRuleOrThrow(id: UuidV4, locator: TimeRangeRuleLocator) {}

  fun addTimeAllowanceRuleOrThrow(id: UuidV4, rule: TimeAllowanceRule, locator: TimeAllowanceRuleLocator) {}
  fun deleteTimeAllowanceRuleOrThrow(id: UuidV4, locator: TimeAllowanceRuleLocator) {}

  fun addCountdownRuleAtMainUserProfileScreenRegulatonOrThrow(id: UuidV4, rule: CountdownRule) {}
  fun deleteCountdownRuleAtMainUserProfileScreenRegulatonOrThrow(id: UuidV4) {}

  fun addCountdownRuleAtMainUserProfileApplicationRegulatonOrThrow(id: UuidV4, rule: CountdownRule) {}
  fun deleteCountdownRuleAtMainUserProfileApplicationRegulatonOrThrow(id: UuidV4) {}
}

sealed class CountdownRuleLocator {
  class MainUserScreen : CountdownRuleLocator() {}
  class MainUserApplicationRegulation(val app: AppName) : CountdownRuleLocator() {}
}

sealed class CountdownRuleContext {
  class MainUserScreen : CountdownRuleContext() {}
  class MainUserApplicationRegulation(val app: AppName, val appRule: ApplicationRegulation) : CountdownRuleContext() {}
}

sealed class TimeRangeRuleLocator {
  class MainUserScreen : TimeRangeRuleLocator() {}
  class MainUserApplicationRegulation(val app: AppName) : TimeRangeRuleLocator() {}
}

sealed class TimeRangeRuleContext {
  class MainUserScreen : TimeRangeRuleContext() {}
  class MainUserApplicationRegulation(val app: AppName, val appRule: ApplicationRegulation) : TimeRangeRuleContext() {}
}

sealed class TimeAllowanceRuleLocator {
  class MainUserScreenDaily : TimeAllowanceRuleLocator() {}
  class MainUserApplicationRegulationDaily(val app: AppName) : TimeAllowanceRuleLocator() {}
}

sealed class TimeAllowanceRuleContext {
  class MainUserScreenDaily : TimeAllowanceRuleContext() {}
  class MainUserApplicationRegulationDaily(val app: AppName, val appRule: ApplicationRegulation) : TimeAllowanceRuleContext() {}
}

sealed class RuleError {
  class DatabaseError(val textualError: TextualError) : RuleError() {}
  class DuplicateRuleId() : RuleError() {}
  class PermissionDenied() : RuleError() {}
  class NoSuchRule() : RuleError() {}
  class NoSuchApplicationRegulation() : RuleError() {}
  class TooManyRules() : RuleError() {}
}

public class SharedStateApi(
  val state: State,
  val database: Database,
  val mutex: Mutex,
) {
  // TODO: Create versions of the createCountdownRuleAtMainUserProfileScreenRegulaton and deleteCountdownRuleAtMainUserProfileScreenRegulaton (named createCountdownRuleAtMainUserProfileApplicationRegulaton and deleteCountdownRuleAtMainUserProfileApplicationRegulaton) functions that instead create countdown rules at the main user profile's ApplicationRegulation.countdownRules

  suspend fun createCountdownRuleAtMainUserProfileScreenRegulaton(
    idOrNull: UuidV4?,
    duration: Duration,
  ): Tried<Pair<UuidV4, CountdownRule>, RuleError> {
    mutex.withLock() { 
      if (state.rulesStats.isFull()) {
        return Tried.failure(RuleError.TooManyRules())
      }

      val id = idOrNull ?: UuidV4.generateOrThrow()
      
      if (state.mainUserProfile.screenRule.countdownRules.has(id)) {
        return Tried.failure(RuleError.DuplicateRuleId())
      }
      
      val rule = CountdownRule.create(Countdown.create(
        state.monotonicClock.getNow(),
        duration,
      ))

      try {
        database.addCountdownRuleAtMainUserProfileScreenRegulatonOrThrow(id, rule)
      } catch (e: TextualError) {
        return Tried.failure(RuleError.DatabaseError(e))
      }

      state.mainUserProfile.screenRule.countdownRules.add(id, rule)
      state.rulesStats.rulesNumber += 1
      return Tried.success(id to rule)
    }
  }
  
  suspend fun deleteCountdownRuleAtMainUserProfileScreenRegulaton(
    id: UuidV4,
  ): Tried<CountdownRule, RuleError> {
    mutex.withLock() {
      val rule = state.mainUserProfile.screenRule.countdownRules.get(id)
        ?: return Tried.failure(RuleError.NoSuchRule())
      
      val now = state.monotonicClock.getNow()
      if (rule.isActive(now)) {
        return Tried.failure(RuleError.PermissionDenied())
      }

      try {
        database.deleteCountdownRuleAtMainUserProfileScreenRegulatonOrThrow(id)
      } catch (e: TextualError) {
        return Tried.failure(RuleError.DatabaseError(e))
      }

      state.rulesStats.rulesNumber -= 1
      state.mainUserProfile.screenRule.countdownRules.remove(id)
      return Tried.success(rule)
    }
  }


  suspend fun createCountdownRuleAtMainUserProfileApplicationRegulaton(
    app: AppName,
    idOrNull: UuidV4?,
    duration: Duration,
  ): Tried<Pair<UuidV4, CountdownRule>, RuleError> {
    mutex.withLock() { 
      if (state.rulesStats.isFull()) {
        return Tried.failure(RuleError.TooManyRules())
      }

      val id = idOrNull ?: UuidV4.generateOrThrow()
      val applicationRegulation = state.mainUserProfile.applicationRegulations.get(app)
        ?: return Tried.failure(RuleError.NoSuchApplicationRegulation())

      if (applicationRegulation.countdownRules.has(id)) {
        return Tried.failure(RuleError.DuplicateRuleId())
      }
      
      val rule = CountdownRule.create(Countdown.create(
        state.monotonicClock.getNow(),
        duration,
      ))

      try {
        database.addCountdownRuleAtMainUserProfileApplicationRegulatonOrThrow(id, rule)
      } catch (e: TextualError) {
        return Tried.failure(RuleError.DatabaseError(e))
      }

      applicationRegulation.countdownRules.add(id, rule)
      state.rulesStats.rulesNumber += 1
      return Tried.success(id to rule)
    }
  }
  
  suspend fun deleteCountdownRuleAtMainUserProfileApplicationRegulaton(
    app: AppName,
    id: UuidV4,
  ): Tried<CountdownRule, RuleError> {
    mutex.withLock() {
      val applicationRegulation = state.mainUserProfile.applicationRegulations.get(app)
        ?: return Tried.failure(RuleError.NoSuchApplicationRegulation())

      val rule = applicationRegulation.countdownRules.get(id)
        ?: return Tried.failure(RuleError.NoSuchRule())
      
      val now = state.monotonicClock.getNow()
      if (rule.isActive(now)) {
        return Tried.failure(RuleError.PermissionDenied())
      }

      try {
        database.deleteCountdownRuleAtMainUserProfileApplicationRegulatonOrThrow(id)
      } catch (e: TextualError) {
        return Tried.failure(RuleError.DatabaseError(e))
      }

      applicationRegulation.countdownRules.remove(id)
      state.rulesStats.rulesNumber -= 1
      return Tried.success(rule)
    }
  }
}

sealed class CountdownRuleOperation {
  class Create(val idOrNull: UuidV4?, val duration: Duration) : CountdownRuleOperation() {}
  class Delete(val id: UuidV4) : CountdownRuleOperation() {}

  fun Create.execute(
    database: Database,
    locator: CountdownRuleLocator,
    globalStats: RulesStats,
    rules: CountdownRules,
  ) {
    
  }
}

sealed class TimeRangeRuleOperation {
  class Create(val idOrNull: UuidV4?, val timeRange: TimeRange, val protection: Duration) : CountdownRuleOperation() {}
  class Delete(val id: UuidV4) : CountdownRuleOperation() {}

  fun Create.execute(

  ) {

  }
}

sealed class TimeAllowanceRuleOperation {
  class Create(val idOrNull: UuidV4?, val timeAllowance: Duration, val protection: Duration) : CountdownRuleOperation() {}
  class Delete(val id: UuidV4) : CountdownRuleOperation() {}
}

sealed class ScreenRegulationOperation {
  class CountdownRules(val it: CountdownRuleOperation) : ScreenRegulationOperation() {}
  class TimeRangeRules(val it: TimeRangeRuleOperation) : ScreenRegulationOperation() {}
  class DailyTimeAllowanceRules(val it: TimeAllowanceRuleOperation) : ScreenRegulationOperation() {}
}

sealed class ApplicationRegulationOperation {
  class CountdownRules(val it: CountdownRuleOperation) : ScreenRegulationOperation() {}
  class TimeRangeRules(val it: TimeRangeRuleOperation) : ScreenRegulationOperation() {}
  class DailyTimeAllowanceRules(val it: TimeAllowanceRuleOperation) : ScreenRegulationOperation() {}
}

sealed class ApplicationsRegulationOperation {
  class Create(val app: AppName) : ApplicationsRegulationOperation() {}
  class Delete(val app: AppName) : ApplicationsRegulationOperation() {}
  class Modify(val app: AppName, val it: ApplicationRegulationOperation) : ApplicationsRegulationOperation() {}
}

sealed class UserProfileOperation {
  class ModifyScreenRegulation(val it: ScreenRegulationOperation) : UserProfileOperation() {}
  class ModifyApplicationsRegulations(val it: ApplicationsRegulationOperation) : UserProfileOperation() {}
}

sealed class Operation {
  class ModifyMainUserProfile(val it: UserProfileOperation) : Operation() {}
}