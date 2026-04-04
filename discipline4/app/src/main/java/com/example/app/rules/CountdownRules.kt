package com.example.app

import com.example.app.*

public data class CountdownRules(
  val rules: MutableMap<UuidV4, CountdownRule>,
) {
  companion object {
    fun createDefault(): CountdownRules {
      return CountdownRules(rules = mutableMapOf())
    }
  }

  fun isActive(now: Instant): Boolean {
    return rules.values.any { rule -> rule.isActive(now) }
  }
}
