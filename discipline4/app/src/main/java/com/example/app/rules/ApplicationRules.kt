package com.example.app

import com.example.app.*

/**
 * Collection of rules for multiple applications
 */
public data class ApplicationRules private constructor(
  val rules: MutableMap<AppName, ApplicationRule>
) {
  companion object {
    fun create(rules: MutableMap<AppName, ApplicationRule>): ApplicationRules {
      return ApplicationRules(rules)
    }
    
    fun createDefault(): ApplicationRules {
      return ApplicationRules(mutableMapOf())
    }
  }
  
  fun addRule(app: AppName, rule: ApplicationRule) {
    rules.set(app, rule)
  }
  
  fun removeRule(app: AppName) {
    rules.remove(app)
  }

  override fun toString(): String = "ApplicationRules(${rules.size} rules)"
}