package com.example.app

import com.example.app.*
import androidx.room.Entity

/**
 * A rule that grants a daily time allowance with a lifetime countdown
 */
@Entity
public data class TimeAllowanceRule private constructor(
  var isEnabled: Boolean,
  val allowance: Duration,
  val lifetime: Countdown
) {
  companion object {
    val MAXIMUM_LIFETIME: Duration = Duration.fromMilliseconds(1000 * 60 * 60 * 24 * 3).getOrThrow()
    val MINIMUM_ALLOWANCE: Duration = Duration.fromMilliseconds(1000 * 60 * 60).getOrThrow() 
    val MAXIMUM_ALLOWANCE: Duration = Duration.fromMilliseconds(1000 * 60 * 60 * 24).getOrThrow()
    
    /**
     * Creates a new TimeAllowanceRule with validation
     */
    fun create(allowance: Duration, lifetime: Countdown): Tried<TimeAllowanceRule, TextualError> {
      // Validate allowance minimum
      if (allowance.isShorterThan(MINIMUM_ALLOWANCE)) {
        return Tried.failure(
          TextualError.create("Creating a TimeAllowanceRule")
            .addMessage("Allowance is too short")
            .addStringAttachment("Minimum allowance", MINIMUM_ALLOWANCE.toString())
            .addStringAttachment("Provided allowance", allowance.toString())
        )
      }
      
      // Validate allowance maximum
      if (allowance.isLongerThan(MAXIMUM_ALLOWANCE)) {
        return Tried.failure(
          TextualError.create("Creating a TimeAllowanceRule")
            .addMessage("Allowance is too long")
            .addStringAttachment("Maximum allowance", MAXIMUM_ALLOWANCE.toString())
            .addStringAttachment("Provided allowance", allowance.toString())
        )
      }
      
      // Validate lifetime maximum
      if (lifetime.getTotalDuration().isLongerThan(MAXIMUM_LIFETIME)) {
        return Tried.failure(
          TextualError.create("Creating a TimeAllowanceRule")
            .addMessage("Lifetime is too long")
            .addStringAttachment("Maximum lifetime", MAXIMUM_LIFETIME.toString())
            .addStringAttachment("Provided lifetime", lifetime.getTotalDuration().toString())
        )
      }
      
      return Tried.success(TimeAllowanceRule(
        isEnabled = false, 
        allowance = allowance, 
        lifetime = lifetime,
      ))
    }
  }
  
  fun getTotalAllowance(): Duration {
    return allowance
  }
  
  fun getRemainingAllowance(elapsedTime: Duration): Duration {
    return allowance.minusOrZero(elapsedTime)
  }
  
  fun isAllowanceUp(elapsedTime: Duration): Boolean {
    return elapsedTime.isLongerThan(allowance)
  }
  
  fun getLifetime(): Countdown {
    return lifetime
  }
  
  fun isEnabled(): Boolean {
    return isEnabled
  }
  
  fun enable() {
    isEnabled = true
  }

  fun disable() {
    isEnabled = false
  }
  /**
   * Checks if this rule is active (enabled and not expired)
   */
  fun isActive(now: Instant): Boolean {
    return isEnabled && !lifetime.isFinished(now)
  }
  
  // /**
  //  * Gets the remaining allowance for today
  //  */
  // fun getRemainingAllowanceForToday(now: Instant, dailyUptime: Duration): Duration {
  //   return if (isActive(now)) {
  //     getRemainingAllowance(dailyUptime)
  //   } else {
  //     Duration.zero()
  //   }
  // }
  
  override fun toString(): String {
    return "TimeAllowanceRule(enabled=$isEnabled, allowance=$allowance, lifetime=${lifetime.getTotalDuration()})"
  }
}