package com.example.app

public data class TimeAllowanceRule(
  val enabler: RuleEnabler,
  val allowance: Duration,
) {
  companion object {
    // val MAXIMUM_LIFETIME: Duration = Duration.fromMilliseconds(1000 * 60 * 60 * 24 * 3).getOrThrow()
    // val MINIMUM_ALLOWANCE: Duration = Duration.fromMilliseconds(1000 * 60 * 60).getOrThrow() 
    // val MAXIMUM_ALLOWANCE: Duration = Duration.fromMilliseconds(1000 * 60 * 60 * 24).getOrThrow()
    
    fun create(allowance: Duration, enabler: RuleEnabler): TimeAllowanceRule {
      // // Validate allowance minimum
      // if (allowance.isShorterThan(MINIMUM_ALLOWANCE)) {
      //   return Tried.failure(
      //     TextualError.create("Creating a TimeAllowanceRule")
      //       .addMessage("Allowance is too short")
      //       .addStringAttachment("Minimum allowance", MINIMUM_ALLOWANCE.toString())
      //       .addStringAttachment("Provided allowance", allowance.toString())
      //   )
      // }
      
      // // Validate allowance maximum
      // if (allowance.isLongerThan(MAXIMUM_ALLOWANCE)) {
      //   return Tried.failure(
      //     TextualError.create("Creating a TimeAllowanceRule")
      //       .addMessage("Allowance is too long")
      //       .addStringAttachment("Maximum allowance", MAXIMUM_ALLOWANCE.toString())
      //       .addStringAttachment("Provided allowance", allowance.toString())
      //   )
      // }
      
      // // Validate enabler maximum
      // if (enabler.getTotalDuration().isLongerThan(MAXIMUM_LIFETIME)) {
      //   return Tried.failure(
      //     TextualError.create("Creating a TimeAllowanceRule")
      //       .addMessage("Lifetime is too long")
      //       .addStringAttachment("Maximum enabler", MAXIMUM_LIFETIME.toString())
      //       .addStringAttachment("Provided enabler", enabler.getTotalDuration().toString())
      //   )
      // }
      
      return TimeAllowanceRule(
        allowance = allowance, 
        enabler = enabler,
      )
    }

    fun createOrThrow(allowance: Duration, enabler: RuleEnabler): TimeAllowanceRule {
      // if (allowance.isShorterThan(MINIMUM_ALLOWANCE)) {
      //   throw TextualError.create("Creating a TimeAllowanceRule")
      //     .addMessage("Allowance is too short")
      //     .addStringAttachment("Minimum allowance", MINIMUM_ALLOWANCE.toString())
      //     .addStringAttachment("Provided allowance", allowance.toString())
      // }

      // if (allowance.isLongerThan(MAXIMUM_ALLOWANCE)) {
      //   throw TextualError.create("Creating a TimeAllowanceRule")
      //     .addMessage("Allowance is too long")
      //     .addStringAttachment("Maximum allowance", MAXIMUM_ALLOWANCE.toString())
      //     .addStringAttachment("Provided allowance", allowance.toString())
      // }

      // if (enabler.getTotalDuration().isLongerThan(MAXIMUM_LIFETIME)) {
      //   throw TextualError.create("Creating a TimeAllowanceRule")
      //     .addMessage("Lifetime is too long")
      //     .addStringAttachment("Maximum enabler", MAXIMUM_LIFETIME.toString())
      //     .addStringAttachment("Provided enabler", enabler.getTotalDuration().toString())
      // }

      return TimeAllowanceRule(
        allowance = allowance,
        enabler = enabler,
      )
    }

    fun construct(
      enabler: RuleEnabler,
      allowance: Duration,
    ): TimeAllowanceRule {
      return TimeAllowanceRule(enabler = enabler, allowance = allowance)
    }
  }
  
  fun getTotalAllowance(): Duration {
    return allowance
  }
  
  fun getRemainingAllowance(elapsedTime: Duration): Duration {
    return allowance.saturatingSub(elapsedTime)
  }
  
  fun isAllowanceUp(elapsedTime: Duration): Boolean {
    return elapsedTime.isLongerThan(allowance)
  }
  
  fun isEnabled(now: Instant): Boolean {
    return enabler.isActive(now)
  }
  
  fun isActive(now: Instant, usedAllowance: Duration): Boolean {
    return isEnabled(now) && usedAllowance.isLongerThanOrEqualTo(allowance)
  }
}
