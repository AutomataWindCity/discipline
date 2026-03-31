package com.yourpackage.discipline

/**
 * Tracks daily uptime, resetting at midnight
 */
class UptimeClock(
    var dailyUptime: Duration = Duration.zero(),
    var previousSynchronizationTime: DateTime = DateTime.now()
) {
    companion object {
        fun create(now: DateTime): UptimeClock = UptimeClock(Duration.zero(), now)
    }
    
    /**
     * Synchronizes the clock with the current time
     */
    fun synchronize(
        now: DateTime,
        synchronizationInterval: Duration,
        didSynchronizeSinceDevicePowerUp: Boolean
    ) {
        // Check if we've crossed midnight
        if (now.getDate().isLaterThan(previousSynchronizationTime.getDate())) {
            dailyUptime = Duration.zero()
            previousSynchronizationTime = now
            return
        }
        
        if (!didSynchronizeSinceDevicePowerUp) {
            previousSynchronizationTime = now
            return
        }
        
        val timeSincePreviousSynchronization = max(
            previousSynchronizationTime.tillOrZero(now),
            synchronizationInterval
        )
        
        dailyUptime = dailyUptime.plusOrMax(timeSincePreviousSynchronization)
        previousSynchronizationTime = now
    }
    
    fun getDailyUptime(): Duration = dailyUptime
    fun getPreviousSynchronizationTime(): DateTime = previousSynchronizationTime
}