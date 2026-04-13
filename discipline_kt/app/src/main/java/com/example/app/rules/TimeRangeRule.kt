package com.example.app

public data class TimeRangeRule private constructor(
  val enabler: RuleEnabler,
  val condition: TimeRange,
) {
  companion object {
    fun create(enabler: RuleEnabler, timeRange: TimeRange): TimeRangeRule {
      return TimeRangeRule(enabler, timeRange)
    }
    fun construct(enabler: RuleEnabler, timeRange: TimeRange): TimeRangeRule {
      return TimeRangeRule(enabler, timeRange)
    }
  }
  
  fun isEnabled(now: Instant): Boolean {
    return enabler.isActive(now)
  } 
  
  fun isActive(nowAsInstant: Instant, nowAsTime: Time): Boolean {
    return isEnabled(nowAsInstant) && condition.contains(nowAsTime)
  }
}