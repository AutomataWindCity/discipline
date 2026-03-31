package com.example.app

import arrow.core.Either
import com.example.app.TextualError

/**
 * Represents a point in time measured as elapsed time from a reference point
 * (like system uptime or epoch)
 */
@JvmInline
value class Instant private constructor(private val elapsedTime: Duration) {
    
    companion object {
        /**
         * Creates an Instant from an elapsed time duration
         */
        fun fromElapsedTime(elapsedTime: Duration): Instant = Instant(elapsedTime)
        
        /**
         * Creates an Instant from milliseconds elapsed
         */
        fun fromMilliseconds(milliseconds: Long): Either<TextualError, Instant> {
            return Duration.fromMilliseconds(milliseconds)
                .map { Instant(it) }
        }
        
        /**
         * Creates an Instant from seconds elapsed
         */
        fun fromSeconds(seconds: Long): Either<TextualError, Instant> {
            return Duration.fromSeconds(seconds)
                .map { Instant(it) }
        }
        
        /**
         * Creates an Instant from minutes elapsed
         */
        fun fromMinutes(minutes: Long): Either<TextualError, Instant> {
            return Duration.fromMinutes(minutes)
                .map { Instant(it) }
        }
        
        /**
         * Creates an Instant from hours elapsed
         */
        fun fromHours(hours: Long): Either<TextualError, Instant> {
            return Duration.fromHours(hours)
                .map { Instant(it) }
        }
        
        /**
         * Creates an Instant from days elapsed
         */
        fun fromDays(days: Long): Either<TextualError, Instant> {
            return Duration.fromDays(days)
                .map { Instant(it) }
        }
        
        /**
         * Creates an Instant from weeks elapsed
         */
        fun fromWeeks(weeks: Long): Either<TextualError, Instant> {
            return Duration.fromWeeks(weeks)
                .map { Instant(it) }
        }
        
        /**
         * Creates a zero instant (reference point)
         */
        fun zero(): Instant = Instant(Duration.zero())
    }
    
    /**
     * Returns the elapsed time as a Duration
     */
    fun toElapsedTime(): Duration = elapsedTime
    
    /**
     * Returns the total milliseconds elapsed
     */
    fun toTotalMilliseconds(): Long = elapsedTime.toTotalMilliseconds()
    
    /**
     * Checks if this instant is at the same point as another
     */
    fun isAt(other: Instant): Boolean = 
        elapsedTime.toTotalMilliseconds() == other.elapsedTime.toTotalMilliseconds()
    
    /**
     * Checks if this instant is earlier than another
     */
    fun isEarlierThan(other: Instant): Boolean = 
        elapsedTime.toTotalMilliseconds() < other.elapsedTime.toTotalMilliseconds()
    
    /**
     * Checks if this instant is earlier than or at the same point as another
     */
    fun isEarlierThanOrAt(other: Instant): Boolean = 
        elapsedTime.toTotalMilliseconds() <= other.elapsedTime.toTotalMilliseconds()
    
    /**
     * Checks if this instant is later than another
     */
    fun isLaterThan(other: Instant): Boolean = 
        elapsedTime.toTotalMilliseconds() > other.elapsedTime.toTotalMilliseconds()
    
    /**
     * Checks if this instant is later than or at the same point as another
     */
    fun isLaterThanOrAt(other: Instant): Boolean = 
        elapsedTime.toTotalMilliseconds() >= other.elapsedTime.toTotalMilliseconds()
    
    /**
     * Returns the duration from this instant to another if this instant is earlier,
     * otherwise returns zero
     */
    fun tillOrZero(other: Instant): Duration {
        val thisMs = elapsedTime.toTotalMilliseconds()
        val otherMs = other.elapsedTime.toTotalMilliseconds()
        
        return if (thisMs < otherMs) {
            Duration.fromMillisecondsOrThrow(otherMs - thisMs)
        } else {
            Duration.zero()
        }
    }
    
    /**
     * Returns the duration from another instant to this one if this instant is later,
     * otherwise returns zero
     */
    fun sinceOrZero(other: Instant): Duration {
        val thisMs = elapsedTime.toTotalMilliseconds()
        val otherMs = other.elapsedTime.toTotalMilliseconds()
        
        return if (thisMs > otherMs) {
            Duration.fromMillisecondsOrThrow(thisMs - otherMs)
        } else {
            Duration.zero()
        }
    }
    
    /**
     * Subtracts a duration from this instant, clamping at zero
     */
    fun minusOrZero(duration: Duration): Instant = 
        Instant(elapsedTime.minusOrZero(duration))
    
    /**
     * Adds a duration to this instant, capping at maximum value
     */
    fun plusOrMax(duration: Duration): Instant = 
        Instant(elapsedTime.plusOrMax(duration))
    
    /**
     * Returns the minimum of two instants (earlier one)
     */
    fun min(other: Instant): Instant = 
        if (isEarlierThan(other)) this else other
    
    /**
     * Returns the maximum of two instants (later one)
     */
    fun max(other: Instant): Instant = 
        if (isLaterThan(other)) this else other
    
    /**
     * String representation of this instant
     */
    override fun toString(): String = "Instant(${elapsedTime.toString()})"
    
    /**
     * Detailed string representation
     */
    fun toLongString(): String = "Instant(elapsed: ${elapsedTime.toLongString()})"
    
    // Operator overloading for convenience
    operator fun plus(duration: Duration): Instant = plusOrMax(duration)
    operator fun minus(duration: Duration): Instant = minusOrZero(duration)
    operator fun compareTo(other: Instant): Int = 
        elapsedTime.toTotalMilliseconds().compareTo(other.elapsedTime.toTotalMilliseconds())
}

// Extension functions for working with instants
fun Instant.distanceTo(other: Instant): Duration = 
    if (this.isEarlierThan(other)) this.tillOrZero(other) else this.sinceOrZero(other)

fun Instant.isBetween(start: Instant, end: Instant): Boolean = 
    (this.isLaterThanOrAt(start) && this.isEarlierThanOrAt(end))

// Convenience functions for creating instants
fun instant(milliseconds: Long): Either<TextualError, Instant> = 
    Instant.fromMilliseconds(milliseconds)

fun instantSeconds(seconds: Long): Either<TextualError, Instant> = 
    Instant.fromSeconds(seconds)

fun instantMinutes(minutes: Long): Either<TextualError, Instant> = 
    Instant.fromMinutes(minutes)

fun instantHours(hours: Long): Either<TextualError, Instant> = 
    Instant.fromHours(hours)

fun instantDays(days: Long): Either<TextualError, Instant> = 
    Instant.fromDays(days)

fun instantWeeks(weeks: Long): Either<TextualError, Instant> = 
    Instant.fromWeeks(weeks)