package com.example.app

public data class CountdownConditional(
  var duration: Duration,
  var countdown: Countdown?
) {
  companion object {
    fun create(duration: Duration): CountdownConditional {
      return CountdownConditional(duration, null)
    }

    fun construct(duration: Duration, countdown: Countdown?): CountdownConditional {
      return CountdownConditional(duration, countdown)
    }
  }

  fun isActive(now: Instant): Boolean {
    val countdown = countdown ?: return false
    return countdown.isRunning(now)
  }

  fun reactivate(now: Instant) {
    countdown = Countdown.create(now, duration)
  }

  class ReactivateState(val countdown: Countdown) {}

  fun createReactivateState(now: Instant): ReactivateState {
    return ReactivateState(Countdown.create(now, duration))
  }

  fun reactivateFromState(reactivateState: ReactivateState) {
    countdown = reactivateState.countdown
  }
}