package com.yourpackage.discipline

/**
 * A rule based on a countdown timer
 */
data class CountdownRule private constructor(
    val isEnabled: Boolean,
    val countdown: Countdown
) {
    companion object {
        fun create(countdown: Countdown): CountdownRule = CountdownRule(isEnabled = false, countdown = countdown)
        fun construct(isEnabled: Boolean, countdown: Countdown): CountdownRule = CountdownRule(isEnabled, countdown)
    }
    
    fun getIsEnabled(): Boolean = isEnabled
    fun getCountdown(): Countdown = countdown
    
    fun enable(): CountdownRule = copy(isEnabled = true)
    fun disable(): CountdownRule = copy(isEnabled = false)
    
    /**
     * Checks if this rule is active (enabled and countdown not finished)
     */
    fun isActive(now: Instant): Boolean = isEnabled && !countdown.isFinished(now)
    
    /**
     * Gets the status of the countdown at the given instant
     */
    fun getStatus(now: Instant): CountdownStatus = countdown.getStatus(now)
    
    /**
     * Gets the elapsed time of the countdown
     */
    fun getElapsedTime(now: Instant): Duration = countdown.getElapsedTimeOrZero(now)
    
    /**
     * Gets the remaining time of the countdown
     */
    fun getRemainingTime(now: Instant): Duration = countdown.getRemainingTimeOrZero(now)
    
    /**
     * Gets the total duration of the countdown
     */
    fun getTotalDuration(): Duration = countdown.getTotalDuration()
    
    override fun toString(): String = "CountdownRule(enabled=$isEnabled, countdown=${countdown.getTotalDuration()})"
}