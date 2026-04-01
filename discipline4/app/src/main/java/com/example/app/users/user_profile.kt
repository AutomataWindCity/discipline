package com.example.app

import com.example.app.*
import androidx.room.Entity
import androidx.room.Ignore

/**
 * Complete user profile with all regulations and stats
 */
@Entity
public data class UserProfile(
  val uptimeClock: UptimeClock,
  val vaultsStats: VaultsStats,
  @Ignore
  val screenRule: ScreenRule,
  @Ignore
  val applicationRules: ApplicationRules,
) {
  companion object {
    // fun create(
    //   uptimeClock: UptimeClock,
    //   vaultsStats: VaultsStats
    //   screenRule: ScreenRule,
    //   applicationRules: ApplicationRules,
    // ): UserProfile {
    //   return UserProfile(
    //     uptimeClock = uptimeClock,
    //     vaultsStats = vaultsStats,
    //     screenRule = screenRule,
    //     applicationRules = applicationRules,
    //   )
    // }
    
    fun create(now: Instant): UserProfile {
      return UserProfile(
        uptimeClock = UptimeClock.create(now),
        vaultsStats = VaultsStats.create(maximumVaultsNumber = 10),
        screenRule = ScreenRule.createDefault(),
        applicationRules = ApplicationRules.createDefault(),
      )
    }
  }
  
  fun getScreenRegulation(): ScreenRule {
    return screenRule
  }

  fun getApplicationRules(): ApplicationRules {
    return applicationRules
  }

  fun getUptimeClock(): UptimeClock {
    return uptimeClock
  }

  fun getVaultsStats(): VaultsStats {
    return vaultsStats
  }  
}