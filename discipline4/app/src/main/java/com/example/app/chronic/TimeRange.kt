package com.example.app

import com.example.app.Time
import com.example.app.TextualError
import androidx.room.Entity

/**
 * Represents a range of time within a day, possibly spanning midnight
 */
@Entity
public data class TimeRange private constructor(
  val fromTimestamp: Int,
  val tillTimestamp: Int
) {
  companion object {
    const val MINIMUM_FROM_TIMESTAMP = 0
    const val MAXIMUM_FROM_TIMESTAMP = Time.MAXIMUM_TIMESTAMP
    
    const val MINIMUM_TILL_TIMESTAMP = 0
    const val MAXIMUM_TILL_TIMESTAMP = Time.MAXIMUM_TIMESTAMP * 2 + 1  // Up to 47:59:59.999
    
    /**
     * Creates a TimeRange from timestamps
     */
    fun fromTimestampsOrThrow(from: Int, till: Int): TimeRange {
      if (from < MINIMUM_FROM_TIMESTAMP) {
        throw TextualError 
          .create("Creating a TimeRange from timestamps")
          .addMessage("Argument 'from' is less than the minimum valid value")
          .addIntAttachment("Argument 'from'", from)
          .addIntAttachment("Minimum valid value", MINIMUM_FROM_TIMESTAMP)
      }
      
      if (from > MAXIMUM_FROM_TIMESTAMP) {
        throw TextualError 
          .create("Creating a TimeRange from timestamps")
          .addMessage("Argument 'from' is greater than the maximum valid value")
          .addIntAttachment("Argument 'from'", from)
          .addIntAttachment("Maximum valid value", MAXIMUM_FROM_TIMESTAMP)
      }
      
      if (till < MINIMUM_TILL_TIMESTAMP) {
        throw TextualError 
          .create("Creating a TimeRange from timestamps")
          .addMessage("Argument 'till' is less than the minimum valid value")
          .addIntAttachment("Argument 'till'", till)
          .addIntAttachment("Minimum valid value", MINIMUM_TILL_TIMESTAMP)
      }
      
      if (till > MAXIMUM_TILL_TIMESTAMP) {
        throw TextualError 
          .create("Creating a TimeRange from timestamps")
          .addMessage("Argument 'till' is greater than the maximum valid value")
          .addIntAttachment("Argument 'till'", till)
          .addIntAttachment("Maximum valid value", MAXIMUM_TILL_TIMESTAMP)
      }
      
      if (from > till) {
        throw TextualError 
          .create("Creating a TimeRange from timestamps")
          .addMessage("Argument 'from' is greater than 'till', thereby referring to a later time")
          .addIntAttachment("Argument 'from'", from)
          .addIntAttachment("Argument 'till'", till)
      }
      
      if (till - from >= Duration.MILLISECONDS_PER_DAY) {
        throw TextualError 
          .create("Creating a TimeRange from timestamps")
          .addMessage("Arguments 'from' and 'till' specify a time range that is longer than 24 hours")
          .addIntAttachment("Argument 'from'", from)
          .addIntAttachment("Argument 'till'", till)
      }
      
      return TimeRange(from, till)
    }
    
    /**
     * Creates a TimeRange from two Times
     */
    fun fromTimes(from: Time, till: Time): TimeRange {
      val fromTimestamp = from.toTimestamp()
      val tillTimestamp = till.toTimestamp()
      
      return if (fromTimestamp <= tillTimestamp) {
        TimeRange(fromTimestamp, tillTimestamp)
      } else {
        TimeRange(fromTimestamp, tillTimestamp + Duration.MILLISECONDS_PER_DAY.toInt())
      }
    }
  }
  
  /**
   * Checks if the given time is within this range
   */
  fun contains(time: Time): Boolean {
    val timeTimestamp = time.toTimestamp()
    return timeTimestamp >= fromTimestamp && timeTimestamp <= tillTimestamp
  }
  
  /**
   * Returns the start time of this range (normalized to a 24-hour clock)
   */
  fun getFrom(): Time {
    return Time.fromTimestampOrThrow(fromTimestamp)
  }
  
  /**
   * Returns the end time of this range (normalized to a 24-hour clock)
   */
  fun getTill(): Time {
    val timestamp = if (tillTimestamp <= MAXIMUM_FROM_TIMESTAMP) {
      tillTimestamp
    } else {
      tillTimestamp - Time.MAXIMUM_TIMESTAMP
    }
    
    return Time.fromTimestampOrThrow(timestamp)
  }
  
  /**
   * Returns the duration of this time range
   */
  fun getDuration(): Duration {
    return Duration.fromMillisecondsOrThrow((tillTimestamp - fromTimestamp).toLong())
  }
  
  /**
   * Checks if this range spans midnight
   */
  fun spansMidnight(): Boolean {
    return tillTimestamp > MAXIMUM_FROM_TIMESTAMP
  }
  
  override fun toString(): String {
    return "${getFrom()} .. ${getTill()}"
  }
}