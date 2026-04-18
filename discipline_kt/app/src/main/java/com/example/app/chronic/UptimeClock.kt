package com.example.app

import com.example.app.Instant
import com.example.app.Duration
import androidx.room.Entity

/**
 * Tracks daily uptime, resetting at midnight
 */
@Entity
public data class UptimeClock(
  var totalDailyUptime: Duration,
  var totalWeeklyUptime: Duration,
  var previousSynchronizationTime: Instant,
) {
  companion object {
    fun create(now: Instant): UptimeClock {
      return UptimeClock(
        Duration.zero(),
        Duration.zero(),
        now,
      )
    }
  }
  
  // /**
  //  * Synchronizes the clock with the current time
  //  */
  // fun synchronize(
  //   now: Instant,
  //   synchronizationInterval: Duration,
  //   didSynchronizeSinceDevicePowerUp: Boolean
  // ) {
  //   // Check if we've crossed midnight
  //   if (now.tillOrZero(previousSynchronizationTime).isLongerThan(Duration.DAY)) {
  //     totalDailyUptime = Duration.zero()
  //     previousSynchronizationTime = now
  //     return
  //   }
    
  //   if (!didSynchronizeSinceDevicePowerUp) {
  //     previousSynchronizationTime = now
  //     return
  //   }
    
  //   val timeSincePreviousSynchronization = max(
  //     previousSynchronizationTime.tillOrZero(now),
  //     synchronizationInterval
  //   )
    
  //   totalDailyUptime = totalDailyUptime.saturatingAdd(timeSincePreviousSynchronization)
  //   previousSynchronizationTime = now
  // }
  
  // fun getDailyUptime(): Duration = totalDailyUptime
  // fun getPreviousSynchronizationTime(): DateTime = previousSynchronizationTime
}