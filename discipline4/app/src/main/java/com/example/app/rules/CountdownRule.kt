package com.example.app

import com.example.app.*
import androidx.room.Entity

/**
 * A rule based on a countdown timer
 */
@Entity
public data class CountdownRule private constructor(
  var isEnabled: Boolean,
  val countdown: Countdown
) {
  companion object {
    fun create(countdown: Countdown): CountdownRule {
      return CountdownRule(
        isEnabled = false, 
        countdown = countdown,
      )
    }

    fun construct(
      isEnabled: Boolean, 
      countdown: Countdown,
    ): CountdownRule {
      return CountdownRule(isEnabled, countdown)
    }
  }
  
  fun getIsEnabled(): Boolean {
    return isEnabled
  }

  fun setIsEnabled(newValue: Boolean) {
    isEnabled = newValue
  }

  fun getCountdown(): Countdown {
    return countdown
  }

  /**
   * Checks if this rule is active (enabled and countdown not finished)
   */
  fun isActive(now: Instant): Boolean {
    return isEnabled && !countdown.isFinished(now)
  }
  
  override fun toString(): String {
    return "CountdownRule(enabled=$isEnabled, countdown=${countdown.getTotalDuration()})"
  }
}