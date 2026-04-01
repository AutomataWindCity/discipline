package com.example.app

import com.example.app.TimeRange
import com.example.app.Countdown
import androidx.room.Entity

/**
 * A rule that applies during a specific time range with a lifetime countdown
 */
@Entity
public data class TimeRangeRule private constructor(
  val condition: TimeRange,
  val lifetime: Countdown
) {
  companion object {
    fun create(timeRange: TimeRange, lifetime: Countdown): TimeRangeRule {
      return TimeRangeRule(timeRange, lifetime)
    }
  }
  
  fun getCondition(): TimeRange {
    return condition
  }

  fun getLifetime(): Countdown {
    return lifetime
  }
  
  /**
   * Checks if this rule is active at the given time
   */
  fun isActiveAt(instant: Instant, time: Time): Boolean {
    return condition.contains(time) && !lifetime.isFinished(instant)
  }
  
  /**
   * Checks if this rule has expired
   */
  fun isExpired(now: Instant): Boolean {
    return lifetime.isFinished(now)
  }
  
  /**
   * Gets the remaining time before this rule expires
   */
  fun getRemainingLifetime(now: Instant): Duration {
    return lifetime.getRemainingTimeOrZero(now)
  }
  
  override fun toString(): String {
    return "TimeRangeRule(condition=$condition, lifetime=${lifetime.getTotalDuration()})"
  }
}