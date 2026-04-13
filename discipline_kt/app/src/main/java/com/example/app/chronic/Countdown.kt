package com.example.app

public data class Countdown(
  val from: Instant,
  var duration: Duration
) {
  public enum class Status {
    Pending,   // Hasn't started yet
    Running,   // Currently in progress
    Finished   // Completed
  }

  companion object {
    fun create(from: Instant, duration: Duration): Countdown = Countdown(from, duration)
    fun construct(from: Instant, duration: Duration): Countdown = Countdown(from, duration)
  }
  
  fun getFrom(): Instant {
    return from
  }

  fun getTill(): Instant {
    return from.saturatingAdd(duration)
  }

  fun getTotalDuration(): Duration {
    return duration
  }

  fun setTotalDuration(newDuration: Duration) { 
    duration = newDuration 
  }
  
  fun getTimeTillStartOrZero(now: Instant): Duration {
    return now.tillOrZero(from)
  }

  fun getTimeSinceStartOrZero(now: Instant): Duration {
    return now.sinceOrZero(from)
  }
  
  fun getElapsedTimeOrZero(now: Instant): Duration {
    return getTimeSinceStartOrZero(now).min(duration)
  }
  
  fun getRemainingTimeOrZero(now: Instant): Duration {
    return duration.saturatingSub(getElapsedTimeOrZero(now))
  }
  
  fun getTimeTillFinishOrZero(now: Instant): Duration {
    return now.tillOrZero(getTill())
  }
  
  fun getStatus(now: Instant): Status {
    return when {
      now.isEarlierThan(from) -> {
        Status.Pending
      }
      getElapsedTimeOrZero(now).isShorterThanOrEqualTo(duration) -> {
        Status.Running
      }
      else -> {
        Status.Finished
      }
    }
  }
  
  fun isPending(now: Instant): Boolean {
    return getStatus(now) == Status.Pending
  }

  fun isRunning(now: Instant): Boolean {
    return getStatus(now) == Status.Running
  }

  fun isFinished(now: Instant): Boolean {
    return getStatus(now) == Status.Finished
  }
  
  fun extendByOrSetToMax(factor: Duration) {
    duration = duration.saturatingAdd(factor)
  }
  
  fun <R> match(
    now: Instant,
    onPending: (Countdown) -> R,
    onRunning: (Countdown) -> R,
    onFinished: (Countdown) -> R
  ): R {
    return when (getStatus(now)) {
      Status.Pending -> onPending(this)
      Status.Running -> onRunning(this)
      Status.Finished -> onFinished(this)
    }
  }
}