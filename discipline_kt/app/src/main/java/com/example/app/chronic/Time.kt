package com.example.app

import com.example.app.TextualError
import com.example.app.Duration
import androidx.room.Entity

/**
 * Represents a time of day (no date component) with millisecond precision
 */
@Entity
@JvmInline
value class Time private constructor(
  private val timestamp: Int
) {
  
  companion object {
    // 00:00:00.000
    const val MINIMUM_TIMESTAMP = 0

    // 23:59:59.999
    const val MAXIMUM_TIMESTAMP = 1000 * 60 * 60 * 24 - 1  
    
    private const val MILLISECONDS_PER_HOUR = Duration.MILLISECONDS_PER_HOUR.toInt()
    private const val MILLISECONDS_PER_MINUTE = Duration.MILLISECONDS_PER_MINUTE.toInt()
    private const val MILLISECONDS_PER_SECOND = Duration.MILLISECONDS_PER_SECOND.toInt()
    
    /**
     * Creates a Time from a timestamp (milliseconds since midnight)
     */
    fun fromTimestamp(timestamp: Int): Tried<Time, TextualError> {
      if (timestamp < MINIMUM_TIMESTAMP) {
        return Tried.failure(
          TextualError
            .create("Creating a Time from a millisecond timestamp since midnight")
            .addMessage("Argument 'timestamp' is less than the minimum valid value")
            .addIntAttachment("Argument 'timestamp'", timestamp)
            .addIntAttachment("Minimum valid value", MINIMUM_TIMESTAMP)
        )
      }
      
      if (timestamp > MAXIMUM_TIMESTAMP) {
        return Tried.failure(
          TextualError
            .create("Creating a Time from a millisecond timestamp since midnight")
            .addMessage("Argument 'timestamp' is greater than the maximum valid value")
            .addIntAttachment("Argument 'timestamp'", timestamp)
            .addIntAttachment("Maximum valid value", MAXIMUM_TIMESTAMP)
        )
      }
      
      return Tried.success(Time(timestamp))
    }

    fun fromTimestampOrThrow(timestamp: Int): Time {
      if (timestamp < MINIMUM_TIMESTAMP) {
        throw TextualError
          .create("Creating a Time from a millisecond timestamp since midnight")
          .addMessage("Argument 'timestamp' is less than the minimum valid value")
          .addIntAttachment("Argument 'timestamp'", timestamp)
          .addIntAttachment("Minimum valid value", MINIMUM_TIMESTAMP)
      }

      if (timestamp > MAXIMUM_TIMESTAMP) {
        throw TextualError
          .create("Creating a Time from a millisecond timestamp since midnight")
          .addMessage("Argument 'timestamp' is greater than the maximum valid value")
          .addIntAttachment("Argument 'timestamp'", timestamp)
          .addIntAttachment("Maximum valid value", MAXIMUM_TIMESTAMP)
      }

      return Time(timestamp)
    }
    
    /**
     * Creates a Time from hour and minute (24-hour format)
     */
    fun fromHourAndMinute(hour: Int, minute: Int): Tried<Time, TextualError> {
      if (hour !in 0..23) {
        return Tried.failure(
          TextualError
            .create("Creating a Time from hour and minute arguments")
            .addMessage("Argument 'hour' must be between 0 and 23")
            .addIntAttachment("Argument 'hour'", hour)
        )
      }
      
      if (minute !in 0..59) {
        return Tried.failure(
          TextualError
            .create("Creating a Time from hour and minute arguments")
            .addMessage("Argument 'minute' must be between 0 and 59")
            .addIntAttachment("Argument 'minute'", minute)
        )
      }
      
      return Tried.success(
        Time(hour * MILLISECONDS_PER_HOUR + minute * MILLISECONDS_PER_MINUTE)
      )
    }

    fun fromHourAndMinuteOrThrow(hour: Int, minute: Int): Time {
      if (hour !in 0..23) {
        throw TextualError
          .create("Creating a Time from hour and minute arguments")
          .addMessage("Argument 'hour' must be between 0 and 23")
          .addIntAttachment("Argument 'hour'", hour)
      }

      if (minute !in 0..59) {
        throw TextualError
          .create("Creating a Time from hour and minute arguments")
          .addMessage("Argument 'minute' must be between 0 and 59")
          .addIntAttachment("Argument 'minute'", minute)
      }

      return Time(hour * MILLISECONDS_PER_HOUR + minute * MILLISECONDS_PER_MINUTE)
    }
    
    /**
     * Creates a Time from hour and minute (12-hour AM format)
     */
    fun fromHourAndMinuteAm(hour: Int, minute: Int): Tried<Time, TextualError> {
      if (hour !in 0..11) {
        return Tried.failure(
          TextualError
            .create("Creating a Time from hour (AM) and minute arguments")
            .addMessage("Argument 'hour' must be between 0 and 11")
            .addIntAttachment("Argument 'hour'", hour)
        )
      }
      
      if (minute !in 0..59) {
        return Tried.failure(
          TextualError
            .create("Creating a Time from hour (AM) and minute arguments")
            .addMessage("Argument 'minute' must be between 0 and 59")
            .addIntAttachment("Argument 'minute'", minute)
        )
      }
      
      return Tried.success(
        Time(hour * MILLISECONDS_PER_HOUR + minute * MILLISECONDS_PER_MINUTE)
      )
    }

    fun fromHourAndMinuteAmOrThrow(hour: Int, minute: Int): Time {
      if (hour !in 0..11) {
        throw TextualError
          .create("Creating a Time from hour (AM) and minute arguments")
          .addMessage("Argument 'hour' must be between 0 and 11")
          .addIntAttachment("Argument 'hour'", hour)
      }

      if (minute !in 0..59) {
        throw TextualError
          .create("Creating a Time from hour (AM) and minute arguments")
          .addMessage("Argument 'minute' must be between 0 and 59")
          .addIntAttachment("Argument 'minute'", minute)
      }

      return Time(hour * MILLISECONDS_PER_HOUR + minute * MILLISECONDS_PER_MINUTE)
    }
    
    /**
     * Creates a Time from hour and minute (12-hour PM format)
     */
    fun fromHourAndMinutePm(hour: Int, minute: Int): Tried<Time, TextualError> {
      if (hour !in 0..11) {
        return Tried.failure(
          TextualError
            .create("Creating a Time from hour (PM) and minute arguments")
            .addMessage("Argument 'hour' must be between 0 and 11")
            .addIntAttachment("Argument 'hour'", hour)
        )
      }
      
      if (minute !in 0..59) {
        return Tried.failure(
          TextualError
            .create("Creating a Time from hour (PM) and minute arguments")
            .addMessage("Argument 'minute' must be between 0 and 59")
            .addIntAttachment("Argument 'minute'", minute)
        )
      }
      
      return Tried.success(
        Time((12 + hour) * MILLISECONDS_PER_HOUR + minute * MILLISECONDS_PER_MINUTE)
      )
    }

    fun fromHourAndMinutePmOrThrow(hour: Int, minute: Int): Time {
      if (hour !in 0..11) {
        throw TextualError
          .create("Creating a Time from hour (PM) and minute arguments")
          .addMessage("Argument 'hour' must be between 0 and 11")
          .addIntAttachment("Argument 'hour'", hour)
      }

      if (minute !in 0..59) {
        throw TextualError
          .create("Creating a Time from hour (PM) and minute arguments")
          .addMessage("Argument 'minute' must be between 0 and 59")
          .addIntAttachment("Argument 'minute'", minute)
      }

      return Time((12 + hour) * MILLISECONDS_PER_HOUR + minute * MILLISECONDS_PER_MINUTE)
    }
  }
  
  fun toTimestamp(): Int {
    return timestamp
  }
  
  fun getHour(): Int {
    return timestamp / MILLISECONDS_PER_HOUR
  }
  
  fun getMinute(): Int {
    return (timestamp % MILLISECONDS_PER_HOUR) / MILLISECONDS_PER_MINUTE
  }
  
  fun getSecond(): Int {
    return (timestamp % MILLISECONDS_PER_MINUTE) / MILLISECONDS_PER_SECOND
  }
  
  fun getMillisecond(): Int {
    return timestamp % MILLISECONDS_PER_SECOND
  }  
  
  /* BE CRAEFULL: THIS MIGHT THROW */
  override fun toString(): String {
    return String.format("%02d:%02d:%02d", getHour(), getMinute(), getSecond())
  }
  
  fun to12HourStringOrThrow(): String {
    val hour24 = getHour()
    val hour12 = when (hour24) {
      0 -> 12
      in 1..11 -> hour24
      in 12..23 -> hour24 - 12
      else -> 0
    }
    val period = if (hour24 < 12) "AM" else "PM"
    return String.format("%02d:%02d %s", hour12, getMinute(), period)
  }  
}
