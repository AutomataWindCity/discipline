// package com.example.app

// import arrow.core.Either
// import arrow.core.raise.either
// import arrow.core.left
// import arrow.core.right
// import java.time.Instant
// import java.time.LocalDate
// import java.time.ZoneOffset
// import com.example.app.Duration
// import com.example.app.DateTime

// /**
//  * Represents a date (no time component) at UTC midnight
//  */
// @JvmInline
// value class Date private constructor(private val timestamp: Long) {
  
//   companion object {
//     const val MINIMUM_TIMESTAMP = -8_640_000_000_000_000L
//     const val MAXIMUM_TIMESTAMP = 8_640_000_000_000_000L
    
//     /**
//      * Creates a Date from a timestamp (must be at UTC midnight)
//      */
//     fun fromTimestamp(timestamp: Long): Either<TextualError, Date> = either {
//       if (timestamp < MINIMUM_TIMESTAMP) {
//         TextualError.create("Creating a Date from a millisecond timestamp since the Unix epoch")
//           .addMessage("Argument 'timestamp' is less than the minimum value")
//           .addNumberAttachment("Argument 'timestamp'", timestamp.toDouble())
//           .addNumberAttachment("Minimum value", MINIMUM_TIMESTAMP.toDouble())
//           .left()
//           // .bind()
//       }
      
//       if (timestamp > MAXIMUM_TIMESTAMP) {
//         TextualError.create("Creating a Date from a millisecond timestamp since the Unix epoch")
//           .addMessage("Argument 'timestamp' is greater than the maximum value")
//           .addNumberAttachment("Argument 'timestamp'", timestamp.toDouble())
//           .addNumberAttachment("Maximum value", MAXIMUM_TIMESTAMP.toDouble())
//           .left()
//           // .bind()
//       }
      
//       // Verify it's at midnight UTC
//       val instant = Instant.ofEpochMilli(timestamp)
//       val localDateTime = instant.atZone(ZoneOffset.UTC)
      
//       if (localDateTime.hour != 0 || localDateTime.minute != 0 || localDateTime.second != 0 || localDateTime.nano != 0) {
//         TextualError.create("Creating a Date from a millisecond timestamp since the Unix epoch")
//           .addMessage("Argument 'timestamp' produced a date with a non-zero time component")
//           .addNumberAttachment("Argument 'timestamp'", timestamp.toDouble())
//           .addStringAttachment("DateTime", localDateTime.toString())
//           .left()
//           // .bind()
//       }
      
//       Date(timestamp).right().bind()
//     }
    
//     fun fromLocalDate(localDate: LocalDate): Date = 
//       Date(localDate.atStartOfDay(ZoneOffset.UTC).toInstant().toEpochMilli())
    
//     fun now(): Date {
//       return fromLocalDate(LocalDate.now(ZoneOffset.UTC))
//     }
    
//     fun fromDateTime(dateTime: DateTime): Date = dateTime.getDate()
//   }
  
//   fun toTimestamp(): Long = timestamp
  
//   fun toLocalDate(): LocalDate = Instant.ofEpochMilli(timestamp).atZone(ZoneOffset.UTC).toLocalDate()
  
//   fun tillOrZero(other: Date): Duration {
//     return if (timestamp < other.timestamp) {
//       Duration.fromMillisecondsOrThrow(other.timestamp - timestamp)
//     } else {
//       Duration.zero()
//     }
//   }
  
//   fun sinceOrZero(other: Date): Duration {
//     return if (timestamp > other.timestamp) {
//       Duration.fromMillisecondsOrThrow(timestamp - other.timestamp)
//     } else {
//       Duration.zero()
//     }
//   }
  
//   fun isLaterThan(other: Date): Boolean {
//     return timestamp > other.timestamp
//   }

//   fun isEarlierThan(other: Date): Boolean {
//     return timestamp < other.timestamp
//   }

//   fun isAt(other: Date): Boolean {
//     return timestamp == other.timestamp
//   }
  
//   override fun toString(): String {
//     return toLocalDate().toString()
//   }
  
//   operator fun plus(days: Long): Date {
//     return Date(timestamp + days * Duration.MILLISECONDS_PER_DAY)
//   }
  
//   operator fun minus(days: Long): Date {
//     return Date(timestamp - days * Duration.MILLISECONDS_PER_DAY)
//   }

//   operator fun compareTo(other: Date): Int {
//     return timestamp.compareTo(other.timestamp)
//   }
// }