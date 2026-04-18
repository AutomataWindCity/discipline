package com.example.app

public data class ApplicationRegulation private constructor(
  val alwaysRules: AlwaysRules,
  val timeRangeRules: TimeRangeRules,
  val dailyTimeAllowanceRules: TimeAllowanceRules,
) {
  companion object {
    fun create(
      AlwaysRules: AlwaysRules,
      timeRangeRules: TimeRangeRules,
      dailyTimeAllowanceRules: TimeAllowanceRules
    ): ApplicationRegulation {
      return ApplicationRegulation(
        AlwaysRules, 
        timeRangeRules, 
        dailyTimeAllowanceRules,
      )
    }
    
    fun createDefault(): ApplicationRegulation {
      return ApplicationRegulation(
        alwaysRules = AlwaysRules.createDefault(),
        timeRangeRules = TimeRangeRules.createDefault(),
        dailyTimeAllowanceRules = TimeAllowanceRules.createDefault()
      )
    }
  }
  
  fun isEnabled(now: Instant): Boolean {
    return (
      alwaysRules.someAreEnabled(now)
      &&
      timeRangeRules.someAreEnabled(now)
      &&
      dailyTimeAllowanceRules.someAreEnabled(now)
    )
  }

  fun isActive(
    nowAsInstant: Instant,
    nowAsTime: Time,
    dailyUsedAllowance: Duration,
  ): Boolean {
    return (
      alwaysRules.someAreActive(nowAsInstant) 
      ||
      timeRangeRules.someAreActive(nowAsInstant, nowAsTime) 
      ||
      dailyTimeAllowanceRules.someAreActive(nowAsInstant, dailyUsedAllowance)
    )
  }
}