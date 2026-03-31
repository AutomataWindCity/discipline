package com.yourpackage.discipline

import arrow.core.Either
import arrow.core.raise.either
import arrow.core.raise.raise

/**
 * A rule that grants a daily time allowance with a lifetime countdown
 */
data class TimeAllowanceRule private constructor(
    val isEnabled: Boolean,
    val allowance: Duration,
    val lifetime: Countdown
) {
    companion object {
        val MINIMUM_ALLOWANCE: Duration = Duration.fromHours(1).getOrNull()!!
        val MAXIMUM_ALLOWANCE: Duration = Duration.fromHours(24).getOrNull()!!
        val MAXIMUM_LIFETIME: Duration = Duration.fromHours(24 * 3).getOrNull()!!
        
        /**
         * Creates a new TimeAllowanceRule with validation
         */
        fun create(allowance: Duration, lifetime: Countdown): Either<TextualError, TimeAllowanceRule> = either {
            // Validate allowance minimum
            if (allowance.isShorterThan(MINIMUM_ALLOWANCE)) {
                raise(
                    TextualError.create("Creating a TimeAllowanceRule")
                        .addMessage("Allowance is too short")
                        .addStringAttachment("Minimum allowance", MINIMUM_ALLOWANCE.toString())
                        .addStringAttachment("Provided allowance", allowance.toString())
                )
            }
            
            // Validate allowance maximum
            if (allowance.isLongerThan(MAXIMUM_ALLOWANCE)) {
                raise(
                    TextualError.create("Creating a TimeAllowanceRule")
                        .addMessage("Allowance is too long")
                        .addStringAttachment("Maximum allowance", MAXIMUM_ALLOWANCE.toString())
                        .addStringAttachment("Provided allowance", allowance.toString())
                )
            }
            
            // Validate lifetime maximum
            if (lifetime.getTotalDuration().isLongerThan(MAXIMUM_LIFETIME)) {
                raise(
                    TextualError.create("Creating a TimeAllowanceRule")
                        .addMessage("Lifetime is too long")
                        .addStringAttachment("Maximum lifetime", MAXIMUM_LIFETIME.toString())
                        .addStringAttachment("Provided lifetime", lifetime.getTotalDuration().toString())
                )
            }
            
            TimeAllowanceRule(isEnabled = false, allowance = allowance, lifetime = lifetime)
        }
        
        /**
         * Creates a rule directly without validation (use with caution)
         */
        fun construct(isEnabled: Boolean, allowance: Duration, lifetime: Countdown): TimeAllowanceRule =
            TimeAllowanceRule(isEnabled, allowance, lifetime)
    }
    
    fun getTotalAllowance(): Duration = allowance
    
    fun getRemainingAllowance(dailyUptime: Duration): Duration = 
        allowance.minusOrZero(dailyUptime)
    
    fun isAllowanceUp(dailyUptime: Duration): Boolean = 
        dailyUptime.isLongerThan(allowance)
    
    fun getLifetime(): Countdown = lifetime
    
    fun isEnabled(): Boolean = isEnabled
    
    fun enable(): TimeAllowanceRule = copy(isEnabled = true)
    
    fun disable(): TimeAllowanceRule = copy(isEnabled = false)
    
    /**
     * Checks if this rule is active (enabled and not expired)
     */
    fun isActive(now: Instant): Boolean = isEnabled && !lifetime.isFinished(now)
    
    /**
     * Gets the remaining allowance for today
     */
    fun getRemainingAllowanceForToday(dailyUptime: Duration): Duration =
        if (isActive(Instant.zero())) getRemainingAllowance(dailyUptime) else Duration.zero()
    
    override fun toString(): String = buildString {
        append("TimeAllowanceRule(")
        append("enabled=$isEnabled, ")
        append("allowance=$allowance, ")
        append("lifetime=${lifetime.getTotalDuration()}")
        append(")")
    }
}