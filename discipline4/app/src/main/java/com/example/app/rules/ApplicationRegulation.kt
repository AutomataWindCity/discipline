package com.example.app

import com.example.app.*

/**
 * Represents regulations for a single application
 */
public data class ApplicationRegulation private constructor(
  val countdownRules: CountdownRules,
  val timeRangeRules: TimeRangeRules,
  val dailyTimeAllowanceRules: TimeAllowanceRules,
) {
  companion object {
    fun create(
      countdownRules: CountdownRules,
      timeRangeRules: TimeRangeRules,
      dailyTimeAllowanceRules: TimeAllowanceRules
    ): ApplicationRegulation {
      return ApplicationRegulation(
        countdownRules, 
        timeRangeRules, 
        dailyTimeAllowanceRules,
      )
    }
    
    fun createDefault(): ApplicationRegulation {
      return ApplicationRegulation(
        countdownRules = CountdownRules.createDefault(),
        timeRangeRules = TimeRangeRules.createDefault(),
        dailyTimeAllowanceRules = TimeAllowanceRules.createDefault()
      )
    }
  }
  
  fun getCountdownRules(): CountdownRules {
    return countdownRules
  }
  
  fun getTimeRangeRules(): TimeRangeRules {
    return timeRangeRules
  }

  fun getTimeAllowanceRules(): TimeAllowanceRules {
    return dailyTimeAllowanceRules
  }
  
  fun isRestricted(
    instant: Instant,
    time: Time,
    dailyUsage: Duration,
  ): Boolean {
    return (
      countdownRules.isActive(instant) 
      ||
      timeRangeRules.isActiveAt(instant, time) 
      ||
      dailyTimeAllowanceRules.isBlocking(instant, dailyUsage)
    )
  }
}