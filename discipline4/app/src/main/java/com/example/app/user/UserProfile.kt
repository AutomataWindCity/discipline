package com.yourpackage.discipline

import arrow.core.Either

/**
 * Complete user profile with all regulations and stats
 */
data class UserProfile private constructor(
    val screenRegulation: ScreenRegulation,
    val applicationRegulations: ApplicationRules,
    val uptimeClock: UptimeClock,
    val vaultsStats: VaultsStats
) {
    companion object {
        fun create(
            screenRegulation: ScreenRegulation,
            applicationRegulations: ApplicationRules,
            uptimeClock: UptimeClock,
            vaultsStats: VaultsStats
        ): UserProfile = UserProfile(
            screenRegulation,
            applicationRegulations,
            uptimeClock,
            vaultsStats
        )
        
        fun createDefault(): UserProfile = UserProfile(
            screenRegulation = ScreenRegulation.createDefault(),
            applicationRegulations = ApplicationRules.createDefault(),
            uptimeClock = UptimeClock.create(DateTime.now()),
            vaultsStats = VaultsStats.create(maximumVaultsNumber = 10)
        )
    }
    
    fun getScreenRegulation(): ScreenRegulation = screenRegulation
    fun getApplicationRegulations(): ApplicationRules = applicationRegulations
    fun getUptimeClock(): UptimeClock = uptimeClock
    fun getVaultsStats(): VaultsStats = vaultsStats
    
    /**
     * Updates screen regulation
     */
    fun updateScreenRegulation(update: (ScreenRegulation) -> ScreenRegulation): UserProfile =
        copy(screenRegulation = update(screenRegulation))
    
    /**
     * Updates application regulations
     */
    fun updateApplicationRegulations(update: (ApplicationRules) -> ApplicationRules): UserProfile =
        copy(applicationRegulations = update(applicationRegulations))
    
    /**
     * Updates uptime clock
     */
    fun updateUptimeClock(update: (UptimeClock) -> UptimeClock): UserProfile =
        copy(uptimeClock = update(uptimeClock))
    
    /**
     * Updates vaults stats
     */
    fun updateVaultsStats(update: (VaultsStats) -> VaultsStats): UserProfile =
        copy(vaultsStats = update(vaultsStats))
    
    /**
     * Synchronizes uptime clock with current time
     */
    fun synchronizeUptime(now: DateTime, synchronizationInterval: Duration, didSync: Boolean): UserProfile {
        uptimeClock.synchronize(now, synchronizationInterval, didSync)
        return this
    }
    
    /**
     * Checks if device usage is currently restricted
     */
    fun isDeviceRestricted(
        now: Instant,
        currentTime: Time,
        dailyUptime: Duration
    ): Boolean {
        return screenRegulation.isScreenRestricted(now, currentTime, dailyUptime)
    }
    
    /**
     * Checks if a specific application is restricted
     */
    fun isApplicationRestricted(
        applicationName: ApplicationName,
        now: Instant,
        currentTime: Time,
        dailyUptime: Duration
    ): Boolean {
        val appRule = applicationRegulations.getRuleFor(applicationName)
        return appRule?.isRestricted(now, currentTime, dailyUptime, applicationName) ?: false
    }
    
    /**
     * Gets all restricted applications
     */
    fun getRestrictedApplications(
        now: Instant,
        currentTime: Time,
        dailyUptime: Duration
    ): List<ApplicationName> {
        return applicationRegulations.getRestrictedApplications(now, currentTime, dailyUptime)
    }
    
    /**
     * Adds a rule for an application
     */
    fun addApplicationRule(
        applicationName: ApplicationName,
        rule: ApplicationRule
    ): UserProfile = copy(
        applicationRegulations = applicationRegulations.addRule(applicationName, rule)
    )
    
    /**
     * Removes rule for an application
     */
    fun removeApplicationRule(applicationName: ApplicationName): UserProfile = copy(
        applicationRegulations = applicationRegulations.removeRule(applicationName)
    )
    
    /**
     * Updates rule for an application
     */
    fun updateApplicationRule(
        applicationName: ApplicationName,
        update: (ApplicationRule) -> ApplicationRule
    ): UserProfile = copy(
        applicationRegulations = applicationRegulations.updateRule(applicationName, update)
    )
    
    override fun toString(): String = buildString {
        appendLine("UserProfile:")
        appendLine("  Screen Regulation: ${screenRegulation}")
        appendLine("  Application Regulations: ${applicationRegulations.size()} apps")
        appendLine("  Uptime: ${uptimeClock.getDailyUptime()}")
        appendLine("  Vaults: ${vaultsStats}")
    }
}