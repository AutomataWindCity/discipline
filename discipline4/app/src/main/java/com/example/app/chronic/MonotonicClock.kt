package com.yourpackage.discipline

/**
 * A monotonic clock that only moves forward
 */
class MonotonicClock(
    var elapsedTime: Duration = Duration.zero(),
    var previousSynchronizationTime: DateTime = DateTime.now()
) {
    companion object {
        fun create(now: DateTime): MonotonicClock = MonotonicClock(Duration.zero(), now)
    }
    
    /**
     * Synchronizes the clock, adding elapsed time since last sync
     */
    fun synchronize(now: DateTime) {
        val interval = previousSynchronizationTime.tillOrZero(now)
        elapsedTime = elapsedTime.plusOrMax(interval)
        previousSynchronizationTime = now
    }
    
    fun getElapsedTime(): Duration = elapsedTime
    fun getPreviousSynchronizationTime(): DateTime = previousSynchronizationTime
    
    /**
     * Returns the current instant based on elapsed time
     */
    fun getNow(): Instant = Instant.fromElapsedTime(elapsedTime)
    
    /**
     * Returns the current DateTime (clamped to not exceed last sync time)
     */
    fun getNowAsDateTime(): DateTime {
        val now = DateTime.now()
        return if (now.isEarlierThan(previousSynchronizationTime)) {
            now
        } else {
            previousSynchronizationTime
        }
    }
}