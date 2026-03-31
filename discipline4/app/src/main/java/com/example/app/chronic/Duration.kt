package com.example.app

import arrow.core.Either
import arrow.core.raise.either
import arrow.core.right
import kotlin.math.floor
import kotlin.math.min
import kotlin.math.max
import com.example.app.TextualError

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

    /**
     * Creates a Duration from milliseconds
     */
    fun fromMilliseconds(milliseconds: Long): Either<TextualError, Duration> = either {
      if (milliseconds < 0) {
        TextualError.create("Creating a Duration from milliseconds")
          .addMessage("Argument 'milliseconds' is negative: This Duration type only supports representing positive durations")
          .addNumberAttachment("Argument 'milliseconds'", milliseconds.toDouble())
          .left()
          .bind()
      }
      
      if (milliseconds > MAXIMUM_MILLISECONDS) {
        TextualError.create("Creating a Duration from milliseconds")
          .addMessage("Argument 'milliseconds' is greater than maximum value")
          .addNumberAttachment("Argument 'milliseconds'", milliseconds.toDouble())
          .addNumberAttachment("Maximum value", MAXIMUM_MILLISECONDS.toDouble())
          .left()
          .bind()
      }

      Duration(milliseconds).right().bind()
    }

    /**
     * Creates a Duration from seconds
     */
    fun fromSeconds(seconds: Long): Either<TextualError, Duration> = either {
      if (seconds < 0) {
        TextualError.create("Creating a Duration from seconds")
          .addMessage("Argument 'seconds' is negative: This Duration type only supports representing positive durations")
          .addNumberAttachment("Argument 'seconds'", seconds.toDouble())
          .left()
          .bind()
      }
      
      if (seconds > MAXIMUM_SECONDS) {
        TextualError.create("Creating a Duration from seconds")
          .addMessage("Argument 'seconds' is greater than maximum value")
          .addNumberAttachment("Argument 'seconds'", seconds.toDouble())
          .addNumberAttachment("Maximum value", MAXIMUM_SECONDS.toDouble())
          .left()
          .bind()
      }

      Duration(seconds * MILLISECONDS_PER_SECOND).right().bind()
    }

    /**
     * Creates a Duration from minutes
     */
    fun fromMinutes(minutes: Long): Either<TextualError, Duration> = either {
      if (minutes < 0) {
        TextualError.create("Creating a Duration from minutes")
          .addMessage("Argument 'minutes' is negative: This Duration type only supports representing positive durations")
          .addNumberAttachment("Argument 'minutes'", minutes.toDouble())
          .left()
          .bind()
      }
      
      if (minutes > MAXIMUM_MINUTES) {
        TextualError.create("Creating a Duration from minutes")
          .addMessage("Argument 'minutes' is greater than maximum value")
          .addNumberAttachment("Argument 'minutes'", minutes.toDouble())
          .addNumberAttachment("Maximum value", MAXIMUM_MINUTES.toDouble())
          .left()
          .bind()
      }

      Duration(minutes * MILLISECONDS_PER_MINUTE).right().bind()
    }

    /**
     * Creates a Duration from hours
     */
    fun fromHours(hours: Long): Either<TextualError, Duration> = either {
      if (hours < 0) {
        TextualError.create("Creating a Duration from hours")
          .addMessage("Argument 'hours' is negative: This Duration type only supports representing positive durations")
          .addNumberAttachment("Argument 'hours'", hours.toDouble())
          .left()
          .bind()
      }
      
      if (hours > MAXIMUM_HOURS) {
        TextualError.create("Creating a Duration from hours")
          .addMessage("Argument 'hours' is greater than maximum value")
          .addNumberAttachment("Argument 'hours'", hours.toDouble())
          .addNumberAttachment("Maximum value", MAXIMUM_HOURS.toDouble())
          .left()
          .bind()
      }

      Duration(hours * MILLISECONDS_PER_HOUR).right().bind()
    }

    /**
     * Creates a Duration from days
     */
    fun fromDays(days: Long): Either<TextualError, Duration> = either {
      if (days < 0) {
        TextualError.create("Creating a Duration from days")
          .addMessage("Argument 'days' is negative: This Duration type only supports representing positive durations")
          .addNumberAttachment("Argument 'days'", days.toDouble())
          .left()
          .bind()
      }
      
      if (days > MAXIMUM_DAYS) {
        TextualError.create("Creating a Duration from days")
          .addMessage("Argument 'days' is greater than maximum value")
          .addNumberAttachment("Argument 'days'", days.toDouble())
          .addNumberAttachment("Maximum value", MAXIMUM_DAYS.toDouble())
          .left()
          .bind()
      }

      Duration(days * MILLISECONDS_PER_DAY).right().bind()
    }

    /**
     * Creates a Duration from weeks
     */
    fun fromWeeks(weeks: Long): Either<TextualError, Duration> = either {
      if (weeks < 0) {
        TextualError.create("Creating a Duration from weeks")
          .addMessage("Argument 'weeks' is negative: This Duration type only supports representing positive durations")
          .addNumberAttachment("Argument 'weeks'", weeks.toDouble())
          .left()
          .bind()
      }
      
      if (weeks > MAXIMUM_WEEKS) {
        TextualError.create("Creating a Duration from weeks")
          .addMessage("Argument 'weeks' is greater than maximum value")
          .addNumberAttachment("Argument 'weeks'", weeks.toDouble())
          .addNumberAttachment("Maximum value", MAXIMUM_WEEKS.toDouble())
          .left()
          .bind()
      }

      Duration(weeks * MILLISECONDS_PER_WEEK).right().bind()
    }

    /**
     * Creates a Duration from milliseconds or throws an exception
     */
    fun fromMillisecondsOrThrow(milliseconds: Long): Duration {
      return fromMilliseconds(milliseconds).fold(
        ifLeft = { error -> throw IllegalStateException(error.prettyPrint()) },
        ifRight = { it }
      )
    }

    /**
     * Returns a zero duration
     */
    fun zero(): Duration = Duration(0)

    private fun construct(milliseconds: Long): Duration = Duration(milliseconds)
  }

  /**
   * Returns the total milliseconds of this duration
   */
  fun toTotalMilliseconds(): Long = milliseconds

  /**
   * Returns the total minutes (floor) of this duration
   */
  fun toTotalMinutes(): Long = milliseconds / MILLISECONDS_PER_MINUTE

  /**
   * Returns true if this duration is zero
   */
  fun isZero(): Boolean = milliseconds == 0L

  /**
   * Subtracts another duration, returns zero if result would be negative
   */
  fun minusOrZero(rhs: Duration): Duration {
    return if (milliseconds > rhs.milliseconds) {
      Duration(milliseconds - rhs.milliseconds)
    } else {
      zero()
    }
  }

  /**
   * Adds another duration, caps at maximum value
   */
  fun plusOrMax(rhs: Duration): Duration {
    val result = milliseconds + rhs.milliseconds
    return if (result <= MAXIMUM_MILLISECONDS) {
      Duration(result)
    } else {
      Duration(MAXIMUM_MILLISECONDS)
    }
  }

  /**
   * Returns true if this duration is equal to another
   */
  fun isEqualTo(rhs: Duration): Boolean = milliseconds == rhs.milliseconds

  /**
   * Returns true if this duration is longer than another
   */
  fun isLongerThan(rhs: Duration): Boolean = milliseconds > rhs.milliseconds

  /**
   * Returns true if this duration is longer than or equal to another
   */
  fun isLongerThanOrEqualTo(rhs: Duration): Boolean = milliseconds >= rhs.milliseconds

  /**
   * Returns true if this duration is shorter than another
   */
  fun isShorterThan(rhs: Duration): Boolean = milliseconds < rhs.milliseconds

  /**
   * Returns true if this duration is shorter than or equal to another
   */
  fun isShorterThanOrEqualTo(rhs: Duration): Boolean = milliseconds <= rhs.milliseconds

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

  operator fun plus(other: Duration): Duration = plusOrMax(other)
  operator fun minus(other: Duration): Duration = minusOrZero(other)
  operator fun compareTo(other: Duration): Int = milliseconds.compareTo(other.milliseconds)
}

// Extension functions for easier usage
fun Duration.totalSeconds(): Long = this.milliseconds / Duration.MILLISECONDS_PER_SECOND
fun Duration.totalMinutes(): Long = this.milliseconds / Duration.MILLISECONDS_PER_MINUTE
fun Duration.totalHours(): Long = this.milliseconds / Duration.MILLISECONDS_PER_HOUR
fun Duration.totalDays(): Long = this.milliseconds / Duration.MILLISECONDS_PER_DAY
fun Duration.totalWeeks(): Long = this.milliseconds / Duration.MILLISECONDS_PER_WEEK

// Convenience functions for creating durations
fun milliseconds(value: Long): Either<TextualError, Duration> = Duration.fromMilliseconds(value)
fun seconds(value: Long): Either<TextualError, Duration> = Duration.fromSeconds(value)
fun minutes(value: Long): Either<TextualError, Duration> = Duration.fromMinutes(value)
fun hours(value: Long): Either<TextualError, Duration> = Duration.fromHours(value)
fun days(value: Long): Either<TextualError, Duration> = Duration.fromDays(value)
fun weeks(value: Long): Either<TextualError, Duration> = Duration.fromWeeks(value)

// Infix functions for natural language operations
infix fun Duration.plusOrMax(other: Duration): Duration = this.plusOrMax(other)
infix fun Duration.minusOrZero(other: Duration): Duration = this.minusOrZero(other)
infix fun Duration.isLongerThan(other: Duration): Boolean = this.isLongerThan(other)
infix fun Duration.isShorterThan(other: Duration): Boolean = this.isShorterThan(other)

// Top-level functions for min/max
fun min(lhs: Duration, rhs: Duration): Duration = if (lhs.isShorterThan(rhs)) lhs else rhs
fun max(lhs: Duration, rhs: Duration): Duration = if (lhs.isLongerThan(rhs)) lhs else rhs