package com.example.app

import com.example.app.*
import androidx.room.Entity

/**
 * A monotonic clock that only moves forward
 */
@Entity
public data class MonotonicClock(
  var elapsedTime: Duration,
  var previousSynchronizationTime: Instant,
  var synchronizationInterval: Duration,
) {
  companion object {
    fun create(synchronizationInterval: Duration): MonotonicClock {
      return MonotonicClock(
        elapsedTime = Duration.zero(), 
        previousSynchronizationTime = Instant.fromElapsedTime(Duration.zero()),
        synchronizationInterval,
      )
    }

    fun constructOrThrow(
      elapsedTime: Duration,
      previousSynchronizationTime: Instant,
      synchronizationInterval: Duration,
    ): MonotonicClock {
      // TODO: Vallidate and throw
      return MonotonicClock(
        elapsedTime = elapsedTime,
        previousSynchronizationTime = previousSynchronizationTime,
        synchronizationInterval = synchronizationInterval,
      )
    }
  }
  
  /**
   * Synchronizes the clock, adding elapsed time since last sync
   */
  fun synchronize(now: Instant) {
    val interval = previousSynchronizationTime.tillOrZero(now)
    elapsedTime = elapsedTime.plusOrMax(interval)
    previousSynchronizationTime = now
  }
  
  fun getElapsedTime(): Duration {
    return elapsedTime
  }

  fun getPreviousSynchronizationTime(): Instant {
    return previousSynchronizationTime
  }
  
  /**
   * Returns the current instant based on elapsed time
   */
  fun getNow(): Instant {
    return Instant.fromElapsedTime(elapsedTime)
  }
  
  // /**
  //  * Returns the current DateTime (clamped to not exceed last sync time)
  //  */
  // fun getNowAsDateTime(): DateTime {
  //   val now = DateTime.now()
  //   return if (now.isEarlierThan(previousSynchronizationTime)) {
  //     now
  //   } else {
  //     previousSynchronizationTime
  //   }
  // }
}