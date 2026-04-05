package com.example.app

public data class RulesStats(
  var rulesNumber: Int,
  var maximumRulesNumber: Int,
) {
  fun isFull(): Boolean {
    return rulesNumber >= maximumRulesNumber
  }
}