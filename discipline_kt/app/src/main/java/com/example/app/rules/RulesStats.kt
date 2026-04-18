package com.example.app

public data class RulesStats(
  var rulesNumber: Int,
  var maximumRulesNumber: Int,
) {
  fun isFull(): Boolean {
    return rulesNumber >= maximumRulesNumber
  }

  fun updateAfterAlwaysRuleCreated() {
    rulesNumber += 1
  }
  fun updateAfterAlwaysRuleDeleted() {
    rulesNumber -= 1
  }
  fun updateAfterTimeRangeRuleCreated() {
    rulesNumber += 1
  }
  fun updateAfterTimeRangeRuleDeleted() {
    rulesNumber -= 1
  }
  fun updateAfterTimeAllowanceRuleCreated() {
    rulesNumber += 1
  }
  fun updateAfterTimeAllowanceRuleDeleted() {
    rulesNumber -= 1
  }
  
  
}