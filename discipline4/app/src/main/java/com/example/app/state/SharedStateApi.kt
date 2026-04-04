package com.example.app

import com.example.app.UuidV4
import com.example.app.State
import com.example.app.Tried
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

class Database {
  fun addMainUserProfileScreenCountdownRule(id: UuidV4, rule: CountdownRule) {}
  fun deleteMainUserProfileScreenCountdownRule(id: UuidV4) {}

  fun addMainUserProfileScreenTimeRangeRule(id: UuidV4, rule: TimeRangeRule) {}
  fun deleteMainUserProfileScreenTimeRangeRule(id: UuidV4) {}

  fun addMainUserProfileScreenTimeAllowanceRule(id: UuidV4, rule: TimeAllowanceRule) {}
  fun deleteMainUserProfileScreenTimeAllowanceRule(id: UuidV4) {}
}

public class SharedState(
  val state: State,
  val mutex: Mutex,
) {
  
  fun createScreenCountdownRule(duration: Duration) {
    
    // val nowAsInstant = monotonicClock.getNow()

    // val rule = CountdownRule.create(Countdown.create(
    //   nowAsInstant, 
    //   duration,
    // ))

    // mainUserProfile.screenRule.countdownRules.rules
  }
  
  fun deleteScreenCountdownRule() {}
  fun createScreenTimeRangeRule() {}
  fun deleteScreenTimeRangeRule() {}
  fun createScreenTimeAllowanceRule() {}
  fun deleteScreenTimeAllowanceRule() {}

  fun createApplicationRule() {}
  fun deleteApplicationRule() {}

  fun createApplicationCountdownRule() {}
  fun deleteApplicationCountdownRule() {}
  fun createApplicationTimeRangeRule() {}
  fun deleteApplicationTimeRangeRule() {}
  fun createApplicationTimeAllowanceRule() {}
  fun deleteApplicationTimeAllowanceRule() {}
}

sealed class CountdownRuleLocator {
  class MainUserScreenRule : CountdownRuleLocator() {}
  class MainUserInternetRule : CountdownRuleLocator() {}
  class MainUserApplicationRule(val applicationRuleId: UuidV4) : CountdownRuleLocator() {}
}

public class SharedStateApi(
  val state: State,
  val database: Database,
  val mutex: Mutex,
) {
  suspend fun createCountdownRule(
    optionalId: UuidV4?,
    duration: Duration,
  ): Pair<UuidV4, CountdownRule> {
    return mutex.withLock {
      val id = optionalId ?: UuidV4.generateOrThrow()

      if (state.mainUserProfile.screenRule.countdownRules.rules.containsKey(id)) {
        throw TextualError.create("Creating a CountdownRule")
          .addMessage("Identifier is already used by another rule")
      }

      val rule = CountdownRule.create(Countdown.create(
        state.monotonicClock.getNow(),
        duration,
      ))

      try {
        database.addMainUserProfileScreenCountdownRule(id, rule)
      } catch (e: Throwable) {
        return Tried.failure(e)
      }

      state.mainUserProfile.screenRule.countdownRules.rules
      id to rule
    }
  }

  suspend fun deleteCountdownRule(
    id: UuidV4,
  ) {
    mutex.withLock { 
      database.deleteMainUserProfileScreenCountdownRule(id)
    }
  }

  suspend fun createTimeRangeRule(
    optionalId: UuidV4?,
    from: Time,
    till: Time,
    lifetime: Duration,
  ) {
    mutex.withLock {
      val id = optionalId ?: UuidV4.generateOrThrow()

      val rule = TimeRangeRule.create(
        TimeRange.fromTimes(
          from, 
          till,
        ), 
        Countdown.create(
          state.monotonicClock.getNow(),
          lifetime,
        )
      )
    }
  }

  fun deleteTimeRangeRule() {}

  fun createTimeAllowanceRule() {}
  fun deleteTimeAllowanceRule() {}

  fun createApplicationRule() {}
  fun deleteApplicationRule() {}
}