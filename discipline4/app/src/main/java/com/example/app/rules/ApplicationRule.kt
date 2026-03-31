package com.yourpackage.discipline

/**
 * Represents regulations for a single application
 */
data class ApplicationRule private constructor(
    val countdownRules: CountdownRules,
    val timeRangeRules: TimeRangeRules,
    val timeAllowanceRules: ScreenTimeAllowanceRules
) {
    companion object {
        fun create(
            countdownRules: CountdownRules,
            timeRangeRules: TimeRangeRules,
            timeAllowanceRules: ScreenTimeAllowanceRules
        ): ApplicationRule = ApplicationRule(countdownRules, timeRangeRules, timeAllowanceRules)
        
        fun createDefault(): ApplicationRule = ApplicationRule(
            countdownRules = CountdownRules.createDefault(),
            timeRangeRules = TimeRangeRules.createDefault(),
            timeAllowanceRules = ScreenTimeAllowanceRules.createDefault()
        )
    }
    
    fun getCountdownRules(): CountdownRules = countdownRules
    fun getTimeRangeRules(): TimeRangeRules = timeRangeRules
    fun getTimeAllowanceRules(): ScreenTimeAllowanceRules = timeAllowanceRules
    
    fun isRestricted(
        now: Instant,
        currentTime: Time,
        dailyUptime: Duration,
        appName: ApplicationName
    ): Boolean {
        return countdownRules.isActive(now) ||
               timeRangeRules.isActiveAt(currentTime, now) ||
               timeAllowanceRules.isAllowanceExceeded(dailyUptime)
    }
    
    fun getActiveRestrictions(
        now: Instant,
        currentTime: Time,
        dailyUptime: Duration
    ): List<RestrictionType> {
        val restrictions = mutableListOf<RestrictionType>()
        
        if (countdownRules.isActive(now)) {
            restrictions.add(RestrictionType.Countdown)
        }
        
        if (timeRangeRules.isActiveAt(currentTime, now)) {
            restrictions.add(RestrictionType.TimeRange)
        }
        
        if (timeAllowanceRules.isAllowanceExceeded(dailyUptime)) {
            restrictions.add(RestrictionType.TimeAllowance)
        }
        
        return restrictions
    }
    
    enum class RestrictionType {
        Countdown,
        TimeRange,
        TimeAllowance
    }
}