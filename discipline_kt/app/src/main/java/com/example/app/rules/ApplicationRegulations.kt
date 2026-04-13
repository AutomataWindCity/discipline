package com.example.app

@JvmInline
value class ApplicationRegulationId(val value: Long) {
  fun toNumber(): Long {
    return value
  }
}

/**
 * Collection of rules for multiple applications
 */
public data class ApplicationRegulations private constructor(
  private val regulations: MutableMap<ApplicationName, ApplicationRegulation>
) {
  companion object {
    fun create(rules: MutableMap<ApplicationName, ApplicationRegulation>): ApplicationRegulations {
      return ApplicationRegulations(rules)
    }
    
    fun createDefault(): ApplicationRegulations {
      return ApplicationRegulations(mutableMapOf())
    }
  }
  
  fun add(applicationName: ApplicationName, regulation: ApplicationRegulation) {
    regulations.set(applicationName, regulation)
  }
  
  fun remove(app: ApplicationName) {
    regulations.remove(app)
  }

  fun get(app: ApplicationName): ApplicationRegulation? {
    return regulations.get(app)
  }

  fun has(app: ApplicationName): Boolean {
    return regulations.containsKey(app)
  }

  fun isApplicationRestricted(
    app: ApplicationName,
    nowAsInstant: Instant,
    nowAsTime: Time,
    dailyUsedAllowance: Duration,
  ): Boolean {
    val regulation = regulations.get(app) ?: return false
    return regulation.isActive(nowAsInstant, nowAsTime, dailyUsedAllowance)
  }

  fun isActive(
    nowAsInstant: Instant,
    nowAsTime: Time,
    dailyUsedAllowance: Duration,
  ): Boolean {
    return regulations.values.any { 
      it.isActive(nowAsInstant, nowAsTime, dailyUsedAllowance) 
    }
  }

  fun isEnabled(now: Instant): Boolean {
    return regulations.values.any {
      it.isEnabled(now)
    }
  }
}