package com.example.app

import com.example.app.*

@JvmInline
public value class Duration private constructor(val milliseconds: Long) {
  companion object {
    const val MILLISECONDS_PER_SECOND = 1000L
    const val MILLISECONDS_PER_MINUTE = MILLISECONDS_PER_SECOND * 60
    const val MILLISECONDS_PER_HOUR = MILLISECONDS_PER_MINUTE * 60
    const val MILLISECONDS_PER_DAY = MILLISECONDS_PER_HOUR * 24
    const val MILLISECONDS_PER_WEEK = MILLISECONDS_PER_DAY * 7

    const val MAXIMUM_MILLISECONDS = Long.MAX_VALUE
    const val MAXIMUM_SECONDS = MAXIMUM_MILLISECONDS / MILLISECONDS_PER_SECOND
    const val MAXIMUM_MINUTES = MAXIMUM_SECONDS / 60
    const val MAXIMUM_HOURS = MAXIMUM_MINUTES / 60
    const val MAXIMUM_DAYS = MAXIMUM_HOURS / 24
    const val MAXIMUM_WEEKS = MAXIMUM_DAYS / 7

    val DAY = Duration(MILLISECONDS_PER_DAY)

    /**
     * Creates a Duration from milliseconds
     */
    fun fromMilliseconds(
      milliseconds: Long,
    ): Tried<Duration, TextualError> {
      if (milliseconds < 0) {
        return Tried.failure(
          TextualError.create("Creating a Duration from milliseconds")
          .addMessage("Argument 'milliseconds' is negative: This Duration type only supports representing positive durations")
          .addLongAttachment("Argument 'milliseconds'", milliseconds)
        )
      }
      
      if (milliseconds > MAXIMUM_MILLISECONDS) {
        return Tried.failure(
          TextualError.create("Creating a Duration from milliseconds")
          .addMessage("Argument 'milliseconds' is greater than maximum value")
          .addLongAttachment("Argument 'milliseconds'", milliseconds)
          .addLongAttachment("Maximum value", MAXIMUM_MILLISECONDS)
        )
      }

      return Tried.success(Duration(milliseconds))
    }

    fun fromMillisecondsOrThrow(milliseconds: Long): Duration {
      if (milliseconds < 0) {
        throw TextualError.create("Creating a Duration from milliseconds")
          .addMessage("Argument 'milliseconds' is negative: This Duration type only supports representing positive durations")
          .addLongAttachment("Argument 'milliseconds'", milliseconds)
      }

      if (milliseconds > MAXIMUM_MILLISECONDS) {
        throw TextualError.create("Creating a Duration from milliseconds")
          .addMessage("Argument 'milliseconds' is greater than maximum value")
          .addLongAttachment("Argument 'milliseconds'", milliseconds)
          .addLongAttachment("Maximum value", MAXIMUM_MILLISECONDS)
      }

      return Duration(milliseconds)
    }

    /**
     * Creates a Duration from seconds
     */
    fun fromSeconds(
      seconds: Long,
    ): Tried<Duration, TextualError> {
      if (seconds < 0) {
        return Tried.failure(
          TextualError.create("Creating a Duration from seconds")
          .addMessage("Argument 'seconds' is negative: This Duration type only supports representing positive durations")
          .addLongAttachment("Argument 'seconds'", seconds)
        )
      }
      
      if (seconds > MAXIMUM_SECONDS) {
        return Tried.failure(
          TextualError.create("Creating a Duration from seconds")
          .addMessage("Argument 'seconds' is greater than maximum value")
          .addLongAttachment("Argument 'seconds'", seconds)
          .addLongAttachment("Maximum value", MAXIMUM_SECONDS)
        )
      }

      return Tried.success(Duration(seconds * MILLISECONDS_PER_SECOND))
    }

    fun fromSecondsOrThrow(
      seconds: Long,
    ): Duration {
      if (seconds < 0) {
        throw TextualError.create("Creating a Duration from seconds")
          .addMessage("Argument 'seconds' is negative: This Duration type only supports representing positive durations")
          .addLongAttachment("Argument 'seconds'", seconds)
      }

      if (seconds > MAXIMUM_SECONDS) {
        throw TextualError.create("Creating a Duration from seconds")
          .addMessage("Argument 'seconds' is greater than maximum value")
          .addLongAttachment("Argument 'seconds'", seconds)
          .addLongAttachment("Maximum value", MAXIMUM_SECONDS)
      }

      return Duration(seconds * MILLISECONDS_PER_SECOND)
    }

    /**
     * Creates a Duration from minutes
     */
    fun fromMinutes(
      minutes: Long,
    ): Tried<Duration, TextualError> {
      if (minutes < 0) {
        return Tried.failure(
          TextualError.create("Creating a Duration from minutes")
          .addMessage("Argument 'minutes' is negative: This Duration type only supports representing positive durations")
          .addLongAttachment("Argument 'minutes'", minutes)
        )
      }
      
      if (minutes > MAXIMUM_MINUTES) {
        return Tried.failure(
          TextualError.create("Creating a Duration from minutes")
          .addMessage("Argument 'minutes' is greater than maximum value")
          .addLongAttachment("Argument 'minutes'", minutes)
          .addLongAttachment("Maximum value", MAXIMUM_MINUTES)
        )
      }

      return Tried.success(Duration(minutes * MILLISECONDS_PER_MINUTE))
    }

    fun fromMinutesOrThrow(
      minutes: Long,
    ): Duration {
      if (minutes < 0) {
        throw TextualError.create("Creating a Duration from minutes")
          .addMessage("Argument 'minutes' is negative: This Duration type only supports representing positive durations")
          .addLongAttachment("Argument 'minutes'", minutes)
      }

      if (minutes > MAXIMUM_MINUTES) {
        throw TextualError.create("Creating a Duration from minutes")
          .addMessage("Argument 'minutes' is greater than maximum value")
          .addLongAttachment("Argument 'minutes'", minutes)
          .addLongAttachment("Maximum value", MAXIMUM_MINUTES)
      }

      return Duration(minutes * MILLISECONDS_PER_MINUTE)
    }

    /**
     * Creates a Duration from hours
     */
    fun fromHours(
      hours: Long,
    ): Tried<Duration, TextualError> {
      if (hours < 0) {
        return Tried.failure(
          TextualError.create("Creating a Duration from hours")
          .addMessage("Argument 'hours' is negative: This Duration type only supports representing positive durations")
          .addLongAttachment("Argument 'hours'", hours)
        )
      }
      
      if (hours > MAXIMUM_HOURS) {
        return Tried.failure(
          TextualError.create("Creating a Duration from hours")
          .addMessage("Argument 'hours' is greater than maximum value")
          .addLongAttachment("Argument 'hours'", hours)
          .addLongAttachment("Maximum value", MAXIMUM_HOURS)
        )
      }

      return Tried.success(Duration(hours * MILLISECONDS_PER_HOUR))
    }

    fun fromHoursOrThrow(
      hours: Long,
    ): Duration {
      if (hours < 0) {
        throw TextualError.create("Creating a Duration from hours")
          .addMessage("Argument 'hours' is negative: This Duration type only supports representing positive durations")
          .addLongAttachment("Argument 'hours'", hours)
      }

      if (hours > MAXIMUM_HOURS) {
        throw TextualError.create("Creating a Duration from hours")
          .addMessage("Argument 'hours' is greater than maximum value")
          .addLongAttachment("Argument 'hours'", hours)
          .addLongAttachment("Maximum value", MAXIMUM_HOURS)
      }

      return Duration(hours * MILLISECONDS_PER_HOUR)
    }

    /**
     * Creates a Duration from days
     */
    fun fromDays(
      days: Long,
    ): Tried<Duration, TextualError> {
      if (days < 0) {
        return Tried.failure(
          TextualError.create("Creating a Duration from days")
          .addMessage("Argument 'days' is negative: This Duration type only supports representing positive durations")
          .addLongAttachment("Argument 'days'", days)
        )
      }
      
      if (days > MAXIMUM_DAYS) {
        return Tried.failure(
          TextualError.create("Creating a Duration from days")
          .addMessage("Argument 'days' is greater than maximum value")
          .addLongAttachment("Argument 'days'", days)
          .addLongAttachment("Maximum value", MAXIMUM_DAYS)
        )
      }

      return Tried.success(Duration(days * MILLISECONDS_PER_DAY))
    }

    fun fromDaysOrThrow(
      days: Long,
    ): Duration {
      if (days < 0) {
        throw TextualError.create("Creating a Duration from days")
          .addMessage("Argument 'days' is negative: This Duration type only supports representing positive durations")
          .addLongAttachment("Argument 'days'", days)
      }

      if (days > MAXIMUM_DAYS) {
        throw TextualError.create("Creating a Duration from days")
          .addMessage("Argument 'days' is greater than maximum value")
          .addLongAttachment("Argument 'days'", days)
          .addLongAttachment("Maximum value", MAXIMUM_DAYS)
      }

      return Duration(days * MILLISECONDS_PER_DAY)
    }

    /**
     * Creates a Duration from weeks
     */
    fun fromWeeks(
      weeks: Long,
    ): Tried<Duration, TextualError> {
      if (weeks < 0) {
        return Tried.failure(
          TextualError.create("Creating a Duration from weeks")
          .addMessage("Argument 'weeks' is negative: This Duration type only supports representing positive durations")
          .addLongAttachment("Argument 'weeks'", weeks)
        )
      }
      
      if (weeks > MAXIMUM_WEEKS) {
        return Tried.failure(
          TextualError.create("Creating a Duration from weeks")
          .addMessage("Argument 'weeks' is greater than maximum value")
          .addLongAttachment("Argument 'weeks'", weeks)
          .addLongAttachment("Maximum value", MAXIMUM_WEEKS)
        )
      }

      return Tried.success(Duration(weeks * MILLISECONDS_PER_WEEK))
    }

    fun fromWeeksOrThrow(
      weeks: Long,
    ): Duration {
      if (weeks < 0) {
        throw TextualError.create("Creating a Duration from weeks")
          .addMessage("Argument 'weeks' is negative: This Duration type only supports representing positive durations")
          .addLongAttachment("Argument 'weeks'", weeks)
      }

      if (weeks > MAXIMUM_WEEKS) {
        throw TextualError.create("Creating a Duration from weeks")
          .addMessage("Argument 'weeks' is greater than maximum value")
          .addLongAttachment("Argument 'weeks'", weeks)
          .addLongAttachment("Maximum value", MAXIMUM_WEEKS)
      }

      return Duration(weeks * MILLISECONDS_PER_WEEK)
    }

    /**
     * Returns a zero duration
     */
    fun zero(): Duration {
      return Duration(0)
    }
  }

  fun toTotalMilliseconds(): Long {
    return milliseconds
  }

  fun toTotalSeconds(): Long {
    return milliseconds / MILLISECONDS_PER_SECOND
  }

  fun toTotalMinutes(): Long {
    return this.milliseconds / MILLISECONDS_PER_MINUTE
  }

  fun toTotalHours(): Long {
    return this.milliseconds / MILLISECONDS_PER_HOUR
  }

  fun toTotalDays(): Long {
    return this.milliseconds / MILLISECONDS_PER_DAY
  }

  fun toTotalWeeks(): Long {
    return this.milliseconds / MILLISECONDS_PER_WEEK
  }


  fun isZero(): Boolean {
    return milliseconds == 0L
  }

  fun saturatingSub(rhs: Duration): Duration {
    return if (milliseconds > rhs.milliseconds) {
      Duration(milliseconds - rhs.milliseconds)
    } else {
      zero()
    }
  }

  fun saturatingAdd(rhs: Duration): Duration {
    return if (milliseconds >= MAXIMUM_MILLISECONDS - rhs.milliseconds) {
      Duration(MAXIMUM_MILLISECONDS)
    } else {
      Duration(milliseconds + rhs.milliseconds)
    }
  }

  /**
   * Returns true if this duration is equal to another
   */
  fun isEqualTo(rhs: Duration): Boolean {
    return milliseconds == rhs.milliseconds
  }

  /**
   * Returns true if this duration is longer than another
   */
  fun isLongerThan(rhs: Duration): Boolean {
    return milliseconds > rhs.milliseconds
  }

  /**
   * Returns true if this duration is longer than or equal to another
   */
  fun isLongerThanOrEqualTo(rhs: Duration): Boolean {
    return milliseconds >= rhs.milliseconds
  }

  /**
   * Returns true if this duration is shorter than another
   */
  fun isShorterThan(rhs: Duration): Boolean {
    return milliseconds < rhs.milliseconds
  }

  /**
   * Returns true if this duration is shorter than or equal to another
   */
  fun isShorterThanOrEqualTo(rhs: Duration): Boolean {
    return milliseconds <= rhs.milliseconds
  }

  /**
   * Returns a human-readable string representation (e.g., "5d 3h 2m")
   */
  override fun toString(): String {
    var remainingMs = milliseconds
    val parts = mutableListOf<String>()
    
    val days = remainingMs / MILLISECONDS_PER_DAY
    remainingMs %= MILLISECONDS_PER_DAY
    if (days > 0) parts.add("${days}d")
    
    val hours = remainingMs / MILLISECONDS_PER_HOUR
    remainingMs %= MILLISECONDS_PER_HOUR
    if (hours > 0) parts.add("${hours}h")
    
    val minutes = remainingMs / MILLISECONDS_PER_MINUTE
    remainingMs %= MILLISECONDS_PER_MINUTE
    if (minutes > 0) parts.add("${minutes}m")
    
    val seconds = remainingMs / MILLISECONDS_PER_SECOND
    if (seconds > 0 || parts.isEmpty()) parts.add("${seconds}s")
    
    return parts.joinToString(" ")
  }

  /**
   * Alternative string representation with full units (e.g., "5 Days 3 Hours 2 Minutes")
   */
  fun toLongString(): String {
    var remainingMs = milliseconds
    val parts = mutableListOf<String>()
    
    val days = remainingMs / MILLISECONDS_PER_DAY
    remainingMs %= MILLISECONDS_PER_DAY
    if (days > 0) parts.add("${days} D")
    
    val hours = remainingMs / MILLISECONDS_PER_HOUR
    remainingMs %= MILLISECONDS_PER_HOUR
    if (hours > 0) parts.add("${hours} H")
    
    val minutes = remainingMs / MILLISECONDS_PER_MINUTE
    remainingMs %= MILLISECONDS_PER_MINUTE
    if (minutes > 0) parts.add("${minutes} M")
    
    return parts.joinToString(" ")
  }

  fun min(rhs: Duration): Duration {
    return if (this.isShorterThan(rhs)) {
      this 
    } else {
      rhs
    }
  }

  fun max(rhs: Duration): Duration {
    return if (this.isLongerThan(rhs)) {
      this
    } else {
      rhs
    }
  }

  // operator fun plus(other: Duration): Duration {
  //   return saturatingAdd(other)
  // }
  // operator fun minus(other: Duration): Duration = saturatingSub(other)
  // operator fun compareTo(other: Duration): Int = milliseconds.compareTo(other.milliseconds)
}
