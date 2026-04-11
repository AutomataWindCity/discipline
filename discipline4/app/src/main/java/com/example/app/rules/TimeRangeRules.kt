package com.example.app

public data class TimeRangeRules(
  private val rules: MutableMap<Id, TimeRangeRule>,
) {
  companion object {
    fun createDefault(): TimeRangeRules {
      return TimeRangeRules(rules = mutableMapOf())
    }
  }

  fun has(id: Id): Boolean {
    return rules.containsKey(id)
  }

  fun get(id: Id): TimeRangeRule? {
    return rules.get(id)
  }

  fun add(id: Id, rule: TimeRangeRule) {
    rules.set(id, rule)
  }

  fun remove(id: Id) {
    rules.remove(id)
  }

  fun someAreEnabled(now: Instant): Boolean {
    return rules.values.any { it.isEnabled(now) }
  }

  fun someAreActive(instant: Instant, time: Time): Boolean {
    return rules.values.any { rule -> rule.isActive(instant, time) }
  }

  @JvmInline
  public value class Id(val id: Long) {
    fun asNumber(): Long {
      return id
    }
  }
}

typealias TimeRangeRuleId = TimeRangeRules.Id