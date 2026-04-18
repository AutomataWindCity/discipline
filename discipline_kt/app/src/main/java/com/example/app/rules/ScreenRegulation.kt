package com.example.app

public data class ScreenRegulation private constructor(
  val alwaysRules: AlwaysRules,
  val timeRangeRules: TimeRangeRules,
  val dailyTimeAllowanceRules: TimeAllowanceRules
) {
  companion object {
    fun create(
      AlwaysRules: AlwaysRules,
      timeRangeRules: TimeRangeRules,
      timeAllowanceRules: TimeAllowanceRules
    ): ScreenRegulation {
      return ScreenRegulation(
        AlwaysRules,
        timeRangeRules,
        timeAllowanceRules
      )
    }
    
    fun createDefault(): ScreenRegulation {
      return ScreenRegulation(
        alwaysRules = AlwaysRules.createDefault(),
        timeRangeRules = TimeRangeRules.createDefault(),
        dailyTimeAllowanceRules = TimeAllowanceRules.createDefault()
      )
    }
  }

  fun isActive(
    nowAsInstant: Instant,
    nowAsTime: Time,
    dailyUsedAllowance: Duration
  ): Boolean {
    return (
      alwaysRules.someAreActive(nowAsInstant) 
      ||
      timeRangeRules.someAreActive(nowAsInstant, nowAsTime) 
      ||
      dailyTimeAllowanceRules.someAreActive(nowAsInstant, dailyUsedAllowance)
    )
  }  

  fun isEnabled(now: Instant): Boolean {
    return (
      alwaysRules.someAreEnabled(now)
      ||
      timeRangeRules.someAreEnabled(now)
      ||
      dailyTimeAllowanceRules.someAreEnabled(now)
    )
  }
}