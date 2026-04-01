package com.example.app

import com.example.app.*

public data class TimeRangeRules(
  val rules: Map<String, TimeRangeRule>,
) {
  companion object {
    fun createDefault(): TimeRangeRules {
      return TimeRangeRules(rules = emptyMap())
    }
  }

  fun isActiveAt(instant: Instant, time: Time): Boolean {
    return rules.values.any { rule -> rule.isActiveAt(instant, time) }
  }
}