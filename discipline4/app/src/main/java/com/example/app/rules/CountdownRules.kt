package com.example.app

import com.example.app.*

public data class CountdownRules(
  val rules: Map<String, CountdownRule>,
) {
  companion object {
    fun createDefault(): CountdownRules {
      return CountdownRules(rules = emptyMap())
    }
  }

  fun isActive(now: Instant): Boolean {
    return rules.values.any { rule -> rule.isActive(now) }
  }
}
