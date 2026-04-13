package com.example.app

public data class AlwaysRules(
  private val rules: MutableMap<Id, AlwaysRule>,
) {
  companion object {
    fun createDefault(): AlwaysRules {
      return AlwaysRules(rules = mutableMapOf())
    }
  }

  fun has(id: Id): Boolean {
    return rules.containsKey(id)
  }

  fun get(id: Id): AlwaysRule? {
    return rules.get(id)
  }

  fun add(id: Id, rule: AlwaysRule) {
    rules.set(id, rule)
  }

  fun remove(id: Id) {
    rules.remove(id)
  }

  fun someAreEnabled(now: Instant): Boolean {
    return rules.values.any { it.isEnabled(now) }
  }

  fun someAreActive(now: Instant): Boolean {
    return rules.values.any { rule -> rule.isActive(now) }
  }

  @JvmInline
  public value class Id(val id: Long) {
    fun asNumber(): Long {
      return id
    }
  }
}

typealias AlwaysRuleId = AlwaysRules.Id