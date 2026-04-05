package com.example.app

import com.example.app.*
import kotlinx.coroutines.sync.Mutex

public data class State(
  val monotonicClock: MonotonicClock,
  val mainUserProfile: UserProfile,
  val rulesStats: RulesStats,
  val appRegulationsStats: ApplicationRegulationsStats,
) {
  companion object {
    val MONOTONIC_CLOCK_SYNCHRONIZATION_INTERVAL = Duration.fromMinutes(10).getOrThrow()

    fun createDefault(): State {
      val monotonicClock = MonotonicClock.create(MONOTONIC_CLOCK_SYNCHRONIZATION_INTERVAL)
      val mainUserProfile = UserProfile.create(monotonicClock.getNow())

      return State(
        monotonicClock,
        mainUserProfile,
        rulesStats = RulesStats(0, 100),
        appRegulationsStats = ApplicationRegulationsStats(0, 30)
      )
    }
  }
}
