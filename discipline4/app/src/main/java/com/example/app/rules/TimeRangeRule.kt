package com.yourpackage.discipline

/**
 * A rule that applies during a specific time range with a lifetime countdown
 */
data class TimeRangeRule private constructor(
    val condition: TimeRange,
    val lifetime: Countdown
) {
    companion object {
        fun create(timeRange: TimeRange, lifetime: Countdown): TimeRangeRule = 
            TimeRangeRule(timeRange, lifetime)
    }
    
    fun getCondition(): TimeRange = condition
    fun getLifetime(): Countdown = lifetime
    
    /**
     * Checks if this rule is active at the given time
     */
    fun isActiveAt(time: Time, now: Instant): Boolean = 
        condition.contains(time) && !lifetime.isFinished(now)
    
    /**
     * Checks if this rule has expired
     */
    fun isExpired(now: Instant): Boolean = lifetime.isFinished(now)
    
    /**
     * Gets the remaining time before this rule expires
     */
    fun getRemainingLifetime(now: Instant): Duration = lifetime.getRemainingTimeOrZero(now)
    
    override fun toString(): String = "TimeRangeRule(condition=$condition, lifetime=${lifetime.getTotalDuration()})"
}