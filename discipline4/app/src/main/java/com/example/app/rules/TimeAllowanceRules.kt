package com.example.app

import com.example.app.*

public data class TimeAllowanceRules(
  val rules: Map<String, TimeAllowanceRule>
) {
  companion object {
    fun createDefault(): TimeAllowanceRules {
      return TimeAllowanceRules(rules = emptyMap())
    }
  }

  fun isBlocking(instant: Instant, time: Duration): Boolean {
    return rules.values.any { rule -> 
      rule.isActive(instant) 
      && 
      rule.isAllowanceUp(time) 
    }
  }
}