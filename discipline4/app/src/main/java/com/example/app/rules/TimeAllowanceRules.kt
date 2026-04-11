package com.example.app

public data class TimeAllowanceRules(
  private val rules: MutableMap<Id, TimeAllowanceRule>
) {
  companion object {
    fun createDefault(): TimeAllowanceRules {
      return TimeAllowanceRules(rules = mutableMapOf())
    }
  }

  fun has(id: Id): Boolean {
    return rules.containsKey(id)
  }

  fun get(id: Id): TimeAllowanceRule? {
    return rules.get(id)
  }

  fun add(id: Id, rule: TimeAllowanceRule) {
    rules.set(id, rule)
  }

  fun remove(id: Id) {
    rules.remove(id)
  }

  fun someAreEnabled(now: Instant): Boolean {
    return rules.values.any { it.isEnabled(now) }
  }

  fun someAreActive(now: Instant, usedAllowance: Duration): Boolean {
    return rules.values.any { it.isActive(now, usedAllowance) }
  }

  @JvmInline
  public value class Id(val id: Long) {
    fun asNumber(): Long {
      return id
    }
  }
}

typealias TimeAllowanceRuleId = TimeAllowanceRules.Id