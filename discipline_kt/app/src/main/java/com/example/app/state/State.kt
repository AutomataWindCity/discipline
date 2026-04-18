package com.example.app

public data class State(
  val monotonicClock: MonotonicClock,
  val mainUserProfile: UserProfile,
  val rulesStats: RulesStats,
  val applicationRegulationsStats: ApplicationRegulationsStats,
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
        applicationRegulationsStats = ApplicationRegulationsStats(0, 30)
      )
    }
  }
}

class AlwaysRuleGroupInfo(val locationInfo: AlwaysRuleLocation) {}

fun State.getMonotonicNow(): Instant {
  return monotonicClock.getNow()
}

fun State.getAlwaysRuleGroupInfo() {

}