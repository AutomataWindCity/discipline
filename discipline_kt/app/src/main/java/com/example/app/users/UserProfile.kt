package com.example.app

import com.example.app.ScreenRegulation

public data class UserProfile(
  val uptimeClock: UptimeClock,
  val vaultsStats: VaultsStats,
  val screenRegulation: ScreenRegulation,
  val applicationRegulations: ApplicationRegulations,
) {
  companion object {
    // fun create(
    //   uptimeClock: UptimeClock,
    //   vaultsStats: VaultsStats
    //   screenRegulation: screenRegulation,
    //   ApplicationRegulations: ApplicationRegulations,
    // ): UserProfile {
    //   return UserProfile(
    //     uptimeClock = uptimeClock,
    //     vaultsStats = vaultsStats,
    //     screenRegulation = screenRegulation,
    //     ApplicationRegulations = ApplicationRegulations,
    //   )
    // }
    
    fun create(now: Instant): UserProfile {
      return UserProfile(
        uptimeClock = UptimeClock.create(now),
        vaultsStats = VaultsStats.create(maximumVaultsNumber = 10),
        screenRegulation = ScreenRegulation.createDefault(),
        applicationRegulations = ApplicationRegulations.createDefault(),
      )
    }
  }
  
  fun isScreenRestricted(
    nowAsInstant: Instant,
    nowAsTime: Time,
    dailyUsedAllowance: Duration,
  ): Boolean {
    return screenRegulation.isActive(nowAsInstant, nowAsTime, dailyUsedAllowance)
  }

  // fun isApplicationRestricted(
  //   app: ApplicationName,
  //   nowAsInstant: Instant,
  //   nowAsTime: Time,
  //   dailyUsedAllowance: Duration,
  // ): Boolean {
  //   return applicationRegulations.regulations
  // }
}