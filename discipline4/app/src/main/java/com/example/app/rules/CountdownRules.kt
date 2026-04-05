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

  fun has(id: UuidV4): Boolean {
    return rules.containsKey(id)
  }

  fun get(id: UuidV4): CountdownRule? {
    return rules.get(id)
  }

  fun add(id: UuidV4, rule: CountdownRule) {
    rules.set(id, rule)
  }

  fun remove(id: UuidV4) {
    rules.remove(id)
  }
}
