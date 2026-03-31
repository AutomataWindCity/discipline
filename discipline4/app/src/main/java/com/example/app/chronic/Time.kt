package com.yourpackage.discipline

import arrow.core.Either
import arrow.core.raise.either
import arrow.core.right
import com.example.app.TextualError
import com.example.app.chronic.Duration

/**
 * Represents a time of day (no date component) with millisecond precision
 */
@JvmInline
value class Time private constructor(private val millisecondsSinceMidnight: Int) {
    
    companion object {
        const val MINIMUM_TIMESTAMP = 0
        const val MAXIMUM_TIMESTAMP = 1000 * 60 * 60 * 24 - 1  // 23:59:59.999
        
        private const val MILLISECONDS_PER_HOUR = Duration.MILLISECONDS_PER_HOUR.toInt()
        private const val MILLISECONDS_PER_MINUTE = Duration.MILLISECONDS_PER_MINUTE.toInt()
        private const val MILLISECONDS_PER_SECOND = Duration.MILLISECONDS_PER_SECOND.toInt()
        
        /**
         * Creates a Time from a timestamp (milliseconds since midnight)
         */
        fun fromTimestamp(timestamp: Int): Either<TextualError, Time> = either {
            if (timestamp < MINIMUM_TIMESTAMP) {
                TextualError.create("Creating a Time from a millisecond timestamp since midnight")
                    .addMessage("Argument 'timestamp' is less than the minimum valid value")
                    .addNumberAttachment("Argument 'timestamp'", timestamp.toDouble())
                    .addNumberAttachment("Minimum valid value", MINIMUM_TIMESTAMP.toDouble())
                    .left()
                    .bind()
            }
            
            if (timestamp > MAXIMUM_TIMESTAMP) {
                TextualError.create("Creating a Time from a millisecond timestamp since midnight")
                    .addMessage("Argument 'timestamp' is greater than the maximum valid value")
                    .addNumberAttachment("Argument 'timestamp'", timestamp.toDouble())
                    .addNumberAttachment("Maximum valid value", MAXIMUM_TIMESTAMP.toDouble())
                    .left()
                    .bind()
            }
            
            Time(timestamp).right().bind()
        }
        
        /**
         * Creates a Time from hour and minute (24-hour format)
         */
        fun fromHourAndMinute(hour: Int, minute: Int): Either<TextualError, Time> = either {
            if (hour !in 0..23) {
                TextualError.create("Creating a Time from hour and minute arguments")
                    .addMessage("Argument 'hour' must be between 0 and 23")
                    .addNumberAttachment("Argument 'hour'", hour.toDouble())
                    .left()
                    .bind()
            }
            
            if (minute !in 0..59) {
                TextualError.create("Creating a Time from hour and minute arguments")
                    .addMessage("Argument 'minute' must be between 0 and 59")
                    .addNumberAttachment("Argument 'minute'", minute.toDouble())
                    .left()
                    .bind()
            }
            
            Time(hour * MILLISECONDS_PER_HOUR + minute * MILLISECONDS_PER_MINUTE).right().bind()
        }
        
        /**
         * Creates a Time from hour and minute (12-hour AM format)
         */
        fun fromHourAndMinuteAm(hour: Int, minute: Int): Either<TextualError, Time> = either {
            if (hour !in 0..11) {
                TextualError.create("Creating a Time from hour (AM) and minute arguments")
                    .addMessage("Argument 'hour' must be between 0 and 11")
                    .addNumberAttachment("Argument 'hour'", hour.toDouble())
                    .left()
                    .bind()
            }
            
            if (minute !in 0..59) {
                TextualError.create("Creating a Time from hour (AM) and minute arguments")
                    .addMessage("Argument 'minute' must be between 0 and 59")
                    .addNumberAttachment("Argument 'minute'", minute.toDouble())
                    .left()
                    .bind()
            }
            
            Time(hour * MILLISECONDS_PER_HOUR + minute * MILLISECONDS_PER_MINUTE).right().bind()
        }
        
        /**
         * Creates a Time from hour and minute (12-hour PM format)
         */
        fun fromHourAndMinutePm(hour: Int, minute: Int): Either<TextualError, Time> = either {
            if (hour !in 0..11) {
                TextualError.create("Creating a Time from hour (PM) and minute arguments")
                    .addMessage("Argument 'hour' must be between 0 and 11")
                    .addNumberAttachment("Argument 'hour'", hour.toDouble())
                    .left()
                    .bind()
            }
            
            if (minute !in 0..59) {
                TextualError.create("Creating a Time from hour (PM) and minute arguments")
                    .addMessage("Argument 'minute' must be between 0 and 59")
                    .addNumberAttachment("Argument 'minute'", minute.toDouble())
                    .left()
                    .bind()
            }
            
            Time((12 + hour) * MILLISECONDS_PER_HOUR + minute * MILLISECONDS_PER_MINUTE).right().bind()
        }
        
        /**
         * Creates a Time from hour, minute, and second
         */
        fun fromHourMinuteSecond(hour: Int, minute: Int, second: Int): Either<TextualError, Time> = either {
            fromHourAndMinute(hour, minute).bind()
            if (second !in 0..59) {
                TextualError.create("Creating a Time from hour, minute, and second arguments")
                    .addMessage("Argument 'second' must be between 0 and 59")
                    .addNumberAttachment("Argument 'second'", second.toDouble())
                    .left()
                    .bind()
            }
            
            Time(millisecondsSinceMidnight + second * MILLISECONDS_PER_SECOND).right().bind()
        }
        
        fun zero(): Time = Time(0)
    }
    
    fun toTimestamp(): Int = millisecondsSinceMidnight
    
    fun getHour(): Int = millisecondsSinceMidnight / MILLISECONDS_PER_HOUR
    
    fun getMinute(): Int = (millisecondsSinceMidnight % MILLISECONDS_PER_HOUR) / MILLISECONDS_PER_MINUTE
    
    fun getSecond(): Int = (millisecondsSinceMidnight % MILLISECONDS_PER_MINUTE) / MILLISECONDS_PER_SECOND
    
    fun getMillisecond(): Int = millisecondsSinceMidnight % MILLISECONDS_PER_SECOND
    
    fun toDuration(): Duration = Duration.fromMillisecondsOrThrow(millisecondsSinceMidnight.toLong())
    
    override fun toString(): String = String.format("%02d:%02d:%02d", getHour(), getMinute(), getSecond())
    
    fun to12HourString(): String {
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
    
    operator fun compareTo(other: Time): Int = millisecondsSinceMidnight.compareTo(other.millisecondsSinceMidnight)
}

// Convenience functions
fun time(hour: Int, minute: Int): Either<TextualError, Time> = Time.fromHourAndMinute(hour, minute)
fun timeAm(hour: Int, minute: Int): Either<TextualError, Time> = Time.fromHourAndMinuteAm(hour, minute)
fun timePm(hour: Int, minute: Int): Either<TextualError, Time> = Time.fromHourAndMinutePm(hour, minute)