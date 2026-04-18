package com.example.app

public data class AlwaysRule private constructor(
  val enabler: RuleEnabler
) {
  companion object {
    fun create(enabler: RuleEnabler): AlwaysRule {
      return AlwaysRule(
        enabler = enabler,
      )
    }

    fun construct(
      enabler: RuleEnabler,
    ): AlwaysRule {
      return AlwaysRule(
        enabler,
      )
    }
  }

  fun isEnabled(now: Instant): Boolean {
    return enabler.isActive(now)
  }
  
  fun isActive(now: Instant): Boolean {
    return isEnabled(now)
  }
}