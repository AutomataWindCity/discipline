package com.example.app

import com.example.app.*

/**
 * Collection of rules for multiple applications
 */
public data class ApplicationRegulations private constructor(
  val regulations: MutableMap<AppName, ApplicationRegulation>
) {
  companion object {
    fun create(rules: MutableMap<AppName, ApplicationRegulation>): ApplicationRegulations {
      return ApplicationRegulations(rules)
    }
    
    fun createDefault(): ApplicationRegulations {
      return ApplicationRegulations(mutableMapOf())
    }
  }
  
  fun addRule(app: AppName, rule: ApplicationRegulation) {
    regulations.set(app, rule)
  }
  
  fun removeRule(app: AppName) {
    regulations.remove(app)
  }

  fun get(app: AppName): ApplicationRegulation? {
    return regulations.get(app)
  }

  override fun toString(): String = "ApplicationRegulations(${regulations.size} rules)"
}