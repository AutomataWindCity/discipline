package com.yourpackage.discipline

/**
 * Collection of rules for multiple applications
 */
data class ApplicationRules private constructor(
    val regulations: Map<ApplicationName, ApplicationRule>
) {
    companion object {
        fun create(regulations: Map<ApplicationName, ApplicationRule>): ApplicationRules = 
            ApplicationRules(regulations)
        
        fun createDefault(): ApplicationRules = ApplicationRules(emptyMap())
    }
    
    fun getRuleFor(applicationName: ApplicationName): ApplicationRule? = 
        regulations[applicationName]
    
    fun hasRuleFor(applicationName: ApplicationName): Boolean = 
        regulations.containsKey(applicationName)
    
    fun addRule(applicationName: ApplicationName, rule: ApplicationRule): ApplicationRules =
        copy(regulations = regulations + (applicationName to rule))
    
    fun removeRule(applicationName: ApplicationName): ApplicationRules =
        copy(regulations = regulations - applicationName)
    
    fun updateRule(
        applicationName: ApplicationName,
        update: (ApplicationRule) -> ApplicationRule
    ): ApplicationRules {
        val currentRule = regulations[applicationName] ?: return this
        return copy(regulations = regulations + (applicationName to update(currentRule)))
    }
    
    fun getAllApplicationNames(): Set<ApplicationName> = regulations.keys
    
    fun getRestrictedApplications(
        now: Instant,
        currentTime: Time,
        dailyUptime: Duration
    ): List<ApplicationName> {
        return regulations.filter { (_, rule) ->
            rule.isRestricted(now, currentTime, dailyUptime, it)
        }.keys.toList()
    }
    
    fun getActiveRestrictionsFor(
        applicationName: ApplicationName,
        now: Instant,
        currentTime: Time,
        dailyUptime: Duration
    ): List<ApplicationRule.RestrictionType> {
        return regulations[applicationName]?.getActiveRestrictions(now, currentTime, dailyUptime) 
            ?: emptyList()
    }
    
    fun isEmpty(): Boolean = regulations.isEmpty()
    
    fun size(): Int = regulations.size
    
    override fun toString(): String = "ApplicationRules(${regulations.size} applications)"
}