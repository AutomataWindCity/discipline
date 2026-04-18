package com.example.app

import com.example.app.Duration
import androidx.room.Entity

/**
 * Represents a point in time measured as elapsed time from a reference point
 * (like system uptime or epoch)
 */
@JvmInline
@Entity()
value class Instant private constructor(
  private val elapsedTime: Duration
) {
  
  companion object {
    /**
     * Creates an Instant from an elapsed time duration
     */
    fun fromElapsedTime(elapsedTime: Duration): Instant {
      return Instant(elapsedTime)
    }    
  }
  
  /**
   * Returns the elapsed time as a Duration
   */
  fun toElapsedTime(): Duration = elapsedTime
  
  /**
   * Checks if this instant is at the same point as another
   */
  fun isAt(other: Instant): Boolean {
    return elapsedTime.toTotalMilliseconds() == other.elapsedTime.toTotalMilliseconds()
  }
  
  /**
   * Checks if this instant is earlier than another
   */
  fun isEarlierThan(other: Instant): Boolean {
    return elapsedTime.toTotalMilliseconds() < other.elapsedTime.toTotalMilliseconds()
  }
  
  /**
   * Checks if this instant is earlier than or at the same point as another
   */
  fun isEarlierThanOrAt(other: Instant): Boolean = 
    elapsedTime.toTotalMilliseconds() <= other.elapsedTime.toTotalMilliseconds()
  
  /**
   * Checks if this instant is later than another
   */
  fun isLaterThan(other: Instant): Boolean {
    return elapsedTime.toTotalMilliseconds() > other.elapsedTime.toTotalMilliseconds()
  }
  
  /**
   * Checks if this instant is later than or at the same point as another
   */
  fun isLaterThanOrAt(other: Instant): Boolean {
    return elapsedTime.toTotalMilliseconds() >= other.elapsedTime.toTotalMilliseconds()
  }
  
  /**
   * Returns the duration from this instant to another if this instant is earlier,
   * otherwise returns zero
   */
  fun tillOrZero(other: Instant): Duration {
    return elapsedTime.saturatingSub(other.elapsedTime)
  }
  
  /**
   * Returns the duration from another instant to this one if this instant is later,
   * otherwise returns zero
   */
  fun sinceOrZero(other: Instant): Duration {
    return other.elapsedTime.saturatingSub(elapsedTime)
  }
  
  /**
   * Subtracts a duration from this instant, clamping at zero
   */
  fun saturatingSub(duration: Duration): Instant {
    return Instant(elapsedTime.saturatingSub(duration))
  }
  
  /**
   * Adds a duration to this instant, capping at maximum value
   */
  fun saturatingAdd(duration: Duration): Instant {
    return Instant(elapsedTime.saturatingAdd(duration))
  }
  
  /**
   * Returns the minimum of two instants (earlier one)
   */
  fun min(other: Instant): Instant {
    return if (isEarlierThan(other)) {
      this
    } else {
      other
    }
  }
  
  /**
   * Returns the maximum of two instants (later one)
   */
  fun max(other: Instant): Instant {
    return if (isLaterThan(other)) {
      this
    } else {
      other
    }
  }
  
  /**
   * String representation of this instant
   */
  override fun toString(): String {
    return "Instant(${elapsedTime.toString()})"
  }
  
  /**
   * Detailed string representation
   */
  fun toLongString(): String {
    return "Instant(elapsed: ${elapsedTime.toLongString()})"
  }
}