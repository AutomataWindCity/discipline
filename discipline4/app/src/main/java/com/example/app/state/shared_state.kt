package com.example.app

import com.example.app.*
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

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