package com.example.app

import com.example.app.chronic.Instant
import com.example.app.chronic.Duration

/**
 * Represents the status of a countdown
 */
enum class CountdownStatus {
  Pending,   // Hasn't started yet
  Running,   // Currently in progress
  Finished   // Completed
}

/**
 * Represents a countdown from a start instant over a duration
 */
data class Countdown(
  val from: Instant,
  var duration: Duration
) {
  companion object {
    fun create(from: Instant, duration: Duration): Countdown = Countdown(from, duration)
  }
  
  fun getFrom(): Instant = from
  fun getTill(): Instant = from.plusOrMax(duration)
  fun getTotalDuration(): Duration = duration
  fun setTotalDuration(newDuration: Duration) { 
    duration = newDuration 
  }
  
  fun getTimeTillStartOrZero(now: Instant): Duration {
    return now.tillOrZero(from)
  }

  fun getTimeSinceStartOrZero(now: Instant): Duration {
    return now.sinceOrZero(from)
  }
  
  fun getElapsedTimeOrZero(now: Instant): Duration = 
    min(getTimeSinceStartOrZero(now), duration)
  
  fun getRemainingTimeOrZero(now: Instant): Duration = 
    duration.minusOrZero(getElapsedTimeOrZero(now))
  
  fun getTimeTillFinishOrZero(now: Instant): Duration = 
    now.tillOrZero(getTill())
  
  fun getStatus(now: Instant): CountdownStatus {
    return when {
      now.isEarlierThan(from) -> CountdownStatus.Pending
      getElapsedTimeOrZero(now).isShorterThanOrEqualTo(duration) -> CountdownStatus.Running
      else -> CountdownStatus.Finished
    }
  }
  
  fun isPending(now: Instant): Boolean = getStatus(now) == CountdownStatus.Pending
  fun isRunning(now: Instant): Boolean = getStatus(now) == CountdownStatus.Running
  fun isFinished(now: Instant): Boolean = getStatus(now) == CountdownStatus.Finished
  
  fun extendByOrSetToMax(factor: Duration) {
    duration = duration.plusOrMax(factor)
  }
  
  fun <R> match(
    now: Instant,
    onPending: (Countdown) -> R,
    onRunning: (Countdown) -> R,
    onFinished: (Countdown) -> R
  ): R {
    return when (getStatus(now)) {
      CountdownStatus.Pending -> onPending(this)
      CountdownStatus.Running -> onRunning(this)
      CountdownStatus.Finished -> onFinished(this)
    }
  }
}