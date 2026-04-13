// package com.example.app

// import java.time.LocalDate
// import java.time.ZoneOffset
// import java.time.Instant
// import java.time.OffsetDateTime
// import java.time.ZoneId
// import java.time.format.DateTimeFormatter
// import java.util.Locale

// /**
//  * Represents a point in time with calendar and clock information
//  * Uses UTC internally for consistency
//  */
// @JvmInline
// value class DateTime private constructor(private val instant: OffsetDateTime) {
  
//   companion object {
//     // Timestamp bounds (milliseconds since epoch)
//     const val MINIMUM_TIMESTAMP = -8_640_000_000_000_000L  // -8.64e15
//     const val MAXIMUM_TIMESTAMP = 8_640_000_000_000_000L   // 8.64e15
    
//     // Formatter for Arabic locale (Hijri calendar with Gregorian adjustment)
//     private val arabicFormatter = DateTimeFormatter.ofPattern("yyyy/MM/dd hh:mm:ss a")
//       .withLocale(Locale("ar", "SA"))
//       .withZone(ZoneId.of("UTC"))
    
//     // /**
//     //  * Creates a DateTime from a Java Date
//     //  */
//     // fun fromDate(date: Date): DateTime {
//     //   LocalDateTime.MIN
//     // }
    
//     /**
//      * Creates a DateTime from an Instant (our custom type)
//      */
//     // fun fromInstant(instant: Instant): DateTime = DateTime(instant)
    
//     /**
//      * Creates a DateTime from a timestamp in milliseconds since Unix epoch
//      */
//     fun fromTimestamp(timestamp: Long, textualError: TextualError): DateTime? {
//       // if (timestamp < MINIMUM_TIMESTAMP) {
//       //   textualError 
//       //     .changeContext("Creating a DateTime from a millisecond timestamp since the Unix epoch")
//       //     .addMessage("Argument 'timestamp' is less than the minimum valid value")
//       //     .addNumberAttachment("Argument 'timestamp'", timestamp.toDouble())
//       //     .addNumberAttachment("Minimum valid value", MINIMUM_TIMESTAMP.toDouble())
//       //   return null
//       // }
      
//       // if (timestamp > MAXIMUM_TIMESTAMP) {
//       //   textualError 
//       //     .changeContext("Creating a DateTime from a millisecond timestamp since the Unix epoch")
//       //     .addMessage("Argument 'timestamp' is greater than the maximum valid value")
//       //     .addNumberAttachment("Argument 'timestamp'", timestamp.toDouble())
//       //     .addNumberAttachment("Maximum valid value", MAXIMUM_TIMESTAMP.toDouble())
//       //   return null
//       // }
      
//       val datetime = try {
//         OffsetDateTime.ofInstant(
//           Instant.ofEpochMilli(timestamp),
//           ZoneOffset.UTC
//         )
//       } catch (e: Exception) {
//         textualError 
//           .changeContext("Creating a DateTime from a millisecond timestamp since the Unix epoch")
//           .addMessage("Argument 'timestamp' is valid but didn't produce a valid DateTime")
//           .addNumberAttachment("Argument 'timestamp'", timestamp.toDouble())
//           .addErrorAttachment("Exception", e)
//         return null
//       }
      
//       return DateTime(datetime)
//     }
    
//     /**
//      * Returns the current date and time
//      */
//     fun now(): DateTime {
//       // TODO: Check for exceptions
//       return DateTime(OffsetDateTime.now(ZoneOffset.UTC))
//     }    
//   }
  
//   /**
//    * Returns the timestamp in milliseconds since Unix epoch
//    */
//   fun toTimestamp(): Long {
//     // TODO: Check for exceptions
//     return instant.toInstant().toEpochMilli()
//   }
  
//   /**
//    * Returns the time component (hour and minute)
//    */
//   fun getTime(): Either<TextualError, Time> {
//     val javaInstant = toJavaInstant()
//     val hour = javaInstant.atZone(ZoneOffset.UTC).hour
//     val minute = javaInstant.atZone(ZoneOffset.UTC).minute
//     return Time.fromHourAndMinute(hour, minute)
//   }
  
//   /**
//    * Returns the date component (without time)
//    */
//   fun getDate(): Date {
//     val javaInstant = toJavaInstant()
//     val localDate = javaInstant.atZone(ZoneOffset.UTC).toLocalDate()
//     val dateAtMidnight = localDate.atStartOfDay(ZoneOffset.UTC).toInstant()
//     return Date.fromInstant(Instant.fromEpochMilliseconds(dateAtMidnight.toEpochMilli()))
//   }
  
//   /**
//    * Returns the duration from this DateTime to another if this is earlier,
//    * otherwise returns zero
//    */
//   fun tillOrZero(other: DateTime): Duration {
//     val thisMs = toTimestamp()
//     val otherMs = other.toTimestamp()
    
//     return if (thisMs < otherMs) {
//       Duration.fromMillisecondsOrThrow(otherMs - thisMs)
//     } else {
//       Duration.zero()
//     }
//   }
  
//   /**
//    * Returns the duration from another DateTime to this one if this is later,
//    * otherwise returns zero
//    */
//   fun sinceOrZero(other: DateTime): Duration {
//     val thisMs = toTimestamp()
//     val otherMs = other.toTimestamp()
    
//     return if (thisMs > otherMs) {
//       Duration.fromMillisecondsOrThrow(thisMs - otherMs)
//     } else {
//       Duration.zero()
//     }
//   }
  
//   /**
//    * Adds a duration to this DateTime, capping at maximum timestamp
//    */
//   fun saturatingAdd(duration: Duration): DateTime {
//     val newTimestamp = toTimestamp() + duration.toTotalMilliseconds()
    
//     return if (newTimestamp >= MAXIMUM_TIMESTAMP) {
//       // Cap at maximum
//       DateTime(Instant.fromEpochMilliseconds(MAXIMUM_TIMESTAMP))
//     } else {
//       DateTime(Instant.fromEpochMilliseconds(newTimestamp))
//     }
//   }
  
//   /**
//    * Subtracts a duration from this DateTime, capping at minimum timestamp
//    */
//   fun minusOrMin(duration: Duration): DateTime {
//     val newTimestamp = toTimestamp() - duration.toTotalMilliseconds()
    
//     return if (newTimestamp <= MINIMUM_TIMESTAMP) {
//       DateTime(Instant.fromEpochMilliseconds(MINIMUM_TIMESTAMP))
//     } else {
//       DateTime(Instant.fromEpochMilliseconds(newTimestamp))
//     }
//   }
  
//   // Comparison methods
//   fun isAt(other: DateTime): Boolean = toTimestamp() == other.toTimestamp()
//   fun isEarlierThan(other: DateTime): Boolean = toTimestamp() < other.toTimestamp()
//   fun isEarlierThanOrAt(other: DateTime): Boolean = toTimestamp() <= other.toTimestamp()
//   fun isLaterThan(other: DateTime): Boolean = toTimestamp() > other.toTimestamp()
//   fun isLaterThanOrAt(other: DateTime): Boolean = toTimestamp() >= other.toTimestamp()
  
//   // Comparison with duration constraints
//   fun isLaterThanBy(other: DateTime, duration: Duration): Boolean = 
//     sinceOrZero(other).isEqualTo(duration)
  
//   fun isLaterThanByOrLess(other: DateTime, duration: Duration): Boolean = 
//     sinceOrZero(other).isShorterThanOrEqualTo(duration)
  
//   fun isLaterThanByOrMore(other: DateTime, duration: Duration): Boolean = 
//     sinceOrZero(other).isLongerThanOrEqualTo(duration)
  
//   fun isEarlierThanBy(other: DateTime, duration: Duration): Boolean = 
//     tillOrZero(other).isEqualTo(duration)
  
//   fun isEarlierThanByOrLess(other: DateTime, duration: Duration): Boolean = 
//     tillOrZero(other).isShorterThanOrEqualTo(duration)
  
//   fun isEarlierThanByOrMore(other: DateTime, duration: Duration): Boolean = 
//     tillOrZero(other).isLongerThanOrEqualTo(duration)
  
//   // String representations
//   override fun toString(): String = toIsoString()
  
//   /**
//    * Returns ISO 8601 formatted string
//    */
//   fun toIsoString(): String {
//     val javaInstant = toJavaInstant()
//     return javaInstant.toString()
//   }
  
//   /**
//    * Returns formatted string in Arabic locale
//    */
//   fun toArabicString(): String {
//     val zonedDateTime = toJavaInstant().atZone(ZoneOffset.UTC)
//     return arabicFormatter.format(zonedDateTime)
//   }
  
//   /**
//    * Returns formatted string with custom formatter
//    */
//   fun format(formatter: DateTimeFormatter): String = formatter.format(toJavaInstant())
  
//   // Operator overloads
//   operator fun plus(duration: Duration): DateTime = saturatingAdd(duration)
//   operator fun minus(duration: Duration): DateTime = minusOrMin(duration)
//   operator fun compareTo(other: DateTime): Int = toTimestamp().compareTo(other.toTimestamp())
  
//   // Utility methods
//   fun toEpochSeconds(): Long = toTimestamp() / 1000
//   fun toEpochMillis(): Long = toTimestamp()
  
//   fun getYear(): Int = toJavaInstant().atZone(ZoneOffset.UTC).year
//   fun getMonth(): Int = toJavaInstant().atZone(ZoneOffset.UTC).monthValue
//   fun getDay(): Int = toJavaInstant().atZone(ZoneOffset.UTC).dayOfMonth
//   fun getHour(): Int = toJavaInstant().atZone(ZoneOffset.UTC).hour
//   fun getMinute(): Int = toJavaInstant().atZone(ZoneOffset.UTC).minute
//   fun getSecond(): Int = toJavaInstant().atZone(ZoneOffset.UTC).second
// }

// // Extension functions
// fun DateTime.plus(duration: Duration): DateTime = this + duration
// fun DateTime.minus(duration: Duration): DateTime = this - duration

// fun DateTime.isBetween(start: DateTime, end: DateTime): Boolean = 
//   (this.isLaterThanOrAt(start) && this.isEarlierThanOrAt(end))

// fun DateTime.daysSince(other: DateTime): Long = 
//   sinceOrZero(other).totalDays()

// fun DateTime.hoursSince(other: DateTime): Long = 
//   sinceOrZero(other).totalHours()

// // Convenience functions
// fun dateTime(timestamp: Long): Either<TextualError, DateTime> = 
//   DateTime.fromTimestamp(timestamp)

// fun dateTimeNow(): DateTime = DateTime.now()

// fun dateTimeFromComponents(
//   year: Int,
//   month: Int,
//   day: Int,
//   hour: Int = 0,
//   minute: Int = 0,
//   second: Int = 0
// ): Either<TextualError, DateTime> = 
//   DateTime.fromComponents(year, month, day, hour, minute, second)