package com.example.app

import com.example.app.*
import androidx.room.Entity
import androidx.room.Ignore

/**
 * Complete user profile with all regulations and stats
 */
public data class UserProfile(
  val uptimeClock: UptimeClock,
  val vaultsStats: VaultsStats,
  val screenRule: ScreenRule,
  val applicationRegulations: ApplicationRegulations,
) {
  companion object {
    // fun create(
    //   uptimeClock: UptimeClock,
    //   vaultsStats: VaultsStats
    //   screenRule: ScreenRule,
    //   ApplicationRegulations: ApplicationRegulations,
    // ): UserProfile {
    //   return UserProfile(
    //     uptimeClock = uptimeClock,
    //     vaultsStats = vaultsStats,
    //     screenRule = screenRule,
    //     ApplicationRegulations = ApplicationRegulations,
    //   )
    // }
    
    fun create(now: Instant): UserProfile {
      return UserProfile(
        uptimeClock = UptimeClock.create(now),
        vaultsStats = VaultsStats.create(maximumVaultsNumber = 10),
        screenRule = ScreenRule.createDefault(),
        applicationRegulations = ApplicationRegulations.createDefault(),
      )
    }
  }
  
  fun getScreenRegulation(): ScreenRule {
    return screenRule
  }

  fun getApplicationRegulations(): ApplicationRegulations {
    return applicationRegulations
  }

  fun getUptimeClock(): UptimeClock {
    return uptimeClock
  }

  fun getVaultsStats(): VaultsStats {
    return vaultsStats
  }  
}