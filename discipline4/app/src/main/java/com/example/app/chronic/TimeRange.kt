package com.yourpackage.discipline

import arrow.core.Either
import arrow.core.raise.either
import arrow.core.right

/**
 * Represents a range of time within a day, possibly spanning midnight
 */
data class TimeRange private constructor(
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
        fun fromTimestamps(from: Int, till: Int): Either<TextualError, TimeRange> = either {
            if (from < MINIMUM_FROM_TIMESTAMP) {
                TextualError.create("Creating a TimeRange from timestamps")
                    .addMessage("Argument 'from' is less than the minimum valid value")
                    .addNumberAttachment("Argument 'from'", from.toDouble())
                    .addNumberAttachment("Minimum valid value", MINIMUM_FROM_TIMESTAMP.toDouble())
                    .left()
                    .bind()
            }
            
            if (from > MAXIMUM_FROM_TIMESTAMP) {
                TextualError.create("Creating a TimeRange from timestamps")
                    .addMessage("Argument 'from' is greater than the maximum valid value")
                    .addNumberAttachment("Argument 'from'", from.toDouble())
                    .addNumberAttachment("Maximum valid value", MAXIMUM_FROM_TIMESTAMP.toDouble())
                    .left()
                    .bind()
            }
            
            if (till < MINIMUM_TILL_TIMESTAMP) {
                TextualError.create("Creating a TimeRange from timestamps")
                    .addMessage("Argument 'till' is less than the minimum valid value")
                    .addNumberAttachment("Argument 'till'", till.toDouble())
                    .addNumberAttachment("Minimum valid value", MINIMUM_TILL_TIMESTAMP.toDouble())
                    .left()
                    .bind()
            }
            
            if (till > MAXIMUM_TILL_TIMESTAMP) {
                TextualError.create("Creating a TimeRange from timestamps")
                    .addMessage("Argument 'till' is greater than the maximum valid value")
                    .addNumberAttachment("Argument 'till'", till.toDouble())
                    .addNumberAttachment("Maximum valid value", MAXIMUM_TILL_TIMESTAMP.toDouble())
                    .left()
                    .bind()
            }
            
            if (from > till) {
                TextualError.create("Creating a TimeRange from timestamps")
                    .addMessage("Argument 'from' is greater than 'till', thereby referring to a later time")
                    .addNumberAttachment("Argument 'from'", from.toDouble())
                    .addNumberAttachment("Argument 'till'", till.toDouble())
                    .left()
                    .bind()
            }
            
            if (till - from >= Duration.MILLISECONDS_PER_DAY) {
                TextualError.create("Creating a TimeRange from timestamps")
                    .addMessage("Arguments 'from' and 'till' specify a time range that is longer than 24 hours")
                    .addNumberAttachment("Argument 'from'", from.toDouble())
                    .addNumberAttachment("Argument 'till'", till.toDouble())
                    .left()
                    .bind()
            }
            
            TimeRange(from, till).right().bind()
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
    fun getFrom(): Time = Time.fromTimestamp(fromTimestamp).getOrNull()!!
    
    /**
     * Returns the end time of this range (normalized to a 24-hour clock)
     */
    fun getTill(): Time = Time.fromTimestamp(
        if (tillTimestamp <= MAXIMUM_FROM_TIMESTAMP) tillTimestamp
        else tillTimestamp - Time.MAXIMUM_TIMESTAMP
    ).getOrNull()!!
    
    /**
     * Returns the duration of this time range
     */
    fun getDuration(): Duration = Duration.fromMillisecondsOrThrow((tillTimestamp - fromTimestamp).toLong())
    
    /**
     * Checks if this range spans midnight
     */
    fun spansMidnight(): Boolean = tillTimestamp > MAXIMUM_FROM_TIMESTAMP
    
    override fun toString(): String = "${getFrom()} .. ${getTill()}"
}