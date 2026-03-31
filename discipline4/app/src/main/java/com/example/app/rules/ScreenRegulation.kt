package com.yourpackage.discipline

/**
 * Screen time regulations for device usage
 */
data class ScreenRegulation private constructor(
    val countdownRules: CountdownRules,
    val timeRangeRules: TimeRangeRules,
    val timeAllowanceRules: ScreenTimeAllowanceRules
) {
    companion object {
        fun create(
            countdownRules: CountdownRules,
            timeRangeRules: TimeRangeRules,
            timeAllowanceRules: ScreenTimeAllowanceRules
        ): ScreenRegulation = ScreenRegulation(
            countdownRules,
            timeRangeRules,
            timeAllowanceRules
        )
        
        fun createDefault(): ScreenRegulation = ScreenRegulation(
            countdownRules = CountdownRules.createDefault(),
            timeRangeRules = TimeRangeRules.createDefault(),
            timeAllowanceRules = ScreenTimeAllowanceRules.createDefault()
        )
    }
    
    fun getCountdownRules(): CountdownRules = countdownRules
    fun getTimeRangeRules(): TimeRangeRules = timeRangeRules
    fun getTimeAllowanceRules(): ScreenTimeAllowanceRules = timeAllowanceRules
    
    /**
     * Checks if screen is restricted at the given time
     */
    fun isScreenRestricted(
        now: Instant,
        currentTime: Time,
        dailyUptime: Duration
    ): Boolean {
        return countdownRules.isActive(now) ||
               timeRangeRules.isActiveAt(currentTime, now) ||
               timeAllowanceRules.isAllowanceExceeded(dailyUptime)
    }
    
    /**
     * Gets the reason why screen is restricted (first active restriction)
     */
    fun getRestrictionReason(
        now: Instant,
        currentTime: Time,
        dailyUptime: Duration
    ): RestrictionReason? {
        return when {
            countdownRules.isActive(now) -> RestrictionReason.Countdown(
                countdownRules.getActiveCountdowns(now)
            )
            timeRangeRules.isActiveAt(currentTime, now) -> RestrictionReason.TimeRange(
                timeRangeRules.getActiveRanges(currentTime, now)
            )
            timeAllowanceRules.isAllowanceExceeded(dailyUptime) -> RestrictionReason.AllowanceExceeded(
                timeAllowanceRules.getRemainingAllowance(dailyUptime)
            )
            else -> null
        }
    }
    
    /**
     * Gets all active restrictions
     */
    fun getAllActiveRestrictions(
        now: Instant,
        currentTime: Time,
        dailyUptime: Duration
    ): List<RestrictionReason> {
        val restrictions = mutableListOf<RestrictionReason>()
        
        if (countdownRules.isActive(now)) {
            restrictions.add(RestrictionReason.Countdown(
                countdownRules.getActiveCountdowns(now)
            ))
        }
        
        if (timeRangeRules.isActiveAt(currentTime, now)) {
            restrictions.add(RestrictionReason.TimeRange(
                timeRangeRules.getActiveRanges(currentTime, now)
            ))
        }
        
        if (timeAllowanceRules.isAllowanceExceeded(dailyUptime)) {
            restrictions.add(RestrictionReason.AllowanceExceeded(
                timeAllowanceRules.getRemainingAllowance(dailyUptime)
            ))
        }
        
        return restrictions
    }
    
    sealed class RestrictionReason {
        data class Countdown(val activeCountdowns: List<CountdownRule>) : RestrictionReason()
        data class TimeRange(val activeRanges: List<TimeRangeRule>) : RestrictionReason()
        data class AllowanceExceeded(val remainingAllowance: Duration) : RestrictionReason()
    }
}