package com.example.app

import com.example.app.*

/**
 * Screen time regulations for device usage
 */
public data class ScreenRule private constructor(
  val countdownRules: CountdownRules,
  val timeRangeRules: TimeRangeRules,
  val timeAllowanceRules: TimeAllowanceRules
) {
  companion object {
    fun create(
      countdownRules: CountdownRules,
      timeRangeRules: TimeRangeRules,
      timeAllowanceRules: TimeAllowanceRules
    ): ScreenRule {
      return ScreenRule(
        countdownRules,
        timeRangeRules,
        timeAllowanceRules
      )
    }
    
    fun createDefault(): ScreenRule {
      return ScreenRule(
        countdownRules = CountdownRules.createDefault(),
        timeRangeRules = TimeRangeRules.createDefault(),
        timeAllowanceRules = TimeAllowanceRules.createDefault()
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
    return timeAllowanceRules
  }
  
  /**
   * Checks if screen is restricted at the given time
   */
  fun isScreenRestricted(
    instant: Instant,
    time: Time,
    screenUsageTime: Duration
  ): Boolean {
    return (
      countdownRules.isActive(instant) 
      ||
      timeRangeRules.isActiveAt(instant, time) 
      ||
      timeAllowanceRules.isBlocking(instant, screenUsageTime)
    )
  }  
}