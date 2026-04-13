package com.example.app

public data class State(
  val monotonicClock: MonotonicClock,
  val mainUserProfile: UserProfile,
  val rulesStats: RulesStats,
  val applicationRegulationsStats: ApplicationRegulationsStats,
  val alwaysRuleGroupInfoMap: AlwaysRuleGroupInfoMap,
  val timeRangeRuleGroupInfoMap: TimeRangeRuleGroupInfoMap,
  val timeAllowanceRuleGroupInfoMap: TimeAllowanceRuleGroupInfoMap,
) {
  companion object {
    val MONOTONIC_CLOCK_SYNCHRONIZATION_INTERVAL = Duration.fromMinutes(10).getOrThrow()

    fun createDefault(): State {
      val monotonicClock = MonotonicClock.create(MONOTONIC_CLOCK_SYNCHRONIZATION_INTERVAL)
      val mainUserProfile = UserProfile.create(monotonicClock.getNow())

      TODO()
      // return State(
      //   monotonicClock,
      //   mainUserProfile,
      //   rulesStats = RulesStats(0, 100),
      //   applicationRegulationsStats = ApplicationRegulationsStats(0, 30)
      // )
    }
  }
}