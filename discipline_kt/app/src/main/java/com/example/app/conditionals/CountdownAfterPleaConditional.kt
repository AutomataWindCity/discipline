package com.example.app

public data class CountdownAfterPleaConditional private constructor(
  var intervalFromPleaTillDeactivation: Duration,
  var countdownTillDeactivation: Countdown?,
) {
  enum class State {
    Active, 
    Deactivating,
    Deactivated,
  }

  companion object {
    fun create(intervalFromPleaTillDeactivation: Duration): CountdownAfterPleaConditional {
      return CountdownAfterPleaConditional(intervalFromPleaTillDeactivation, null)
    }

    fun construct(
      intervalFromPleaTillDeactivation: Duration,
      countdownTillDeactivation: Countdown?,
    ): CountdownAfterPleaConditional {
      return CountdownAfterPleaConditional(intervalFromPleaTillDeactivation, countdownTillDeactivation)
    }
  }

  fun getState(now: Instant): State {
    val countdown = countdownTillDeactivation ?: return State.Active

    return when (countdown.getStatus(now)) {
      Countdown.Status.Pending -> {
        State.Active
      }
      Countdown.Status.Running -> {
        State.Deactivating
      }
      Countdown.Status.Finished -> {
        State.Deactivated
      }
    }
  }

  fun isActive(now: Instant): Boolean {
    return when (getState(now)) {
      State.Active -> true,
      else -> false
    }
  }

  fun isActiveOrDeactivating(now: Instant): Boolean {
    return when (getState(now)) {
      State.Active -> true,
      State.Deactivating -> true,
      else -> false
    }
  }

  fun isDeactivating(now: Instant): Boolean {
    return when (getState(now)) {
      State.Deactivating -> true,
      else -> false
    }
  }

  fun isDeactivated(now: Instant): Boolean {
    return when (getState(now)) {
      State.Deactivated -> true,
      else -> false
    }
  }

  fun reactivate() {
    countdownTillDeactivation = null
  }

  fun reDeactivate(now: Instant) {
    countdownTillDeactivation = Countdown.create(now, intervalFromPleaTillDeactivation)
  }

  class ReDeactivateState(val countdown: Countdown) {}

  fun createReDeactivateState(now: Instant): ReDeactivateState {
    return ReDeactivateState(Countdown.create(now, intervalFromPleaTillDeactivation))
  }

  fun reDeactivateFromState(reDeactivateState: ReDeactivateState) {
    countdownTillDeactivation = reDeactivateState.countdown
  }
}