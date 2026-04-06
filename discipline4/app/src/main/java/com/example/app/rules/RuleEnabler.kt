package com.example.app

public sealed class RuleEnabler {
  class Countdown(val it: CountdownConditional) : RuleEnabler() {}
  class CountdownAfterPlea(val it: CountdownAfterPleaConditional) : RuleEnabler() {}

  fun isActive(now: Instant): Boolean {
    return when (this) {
      is Countdown -> {
        it.isActive(now)
      }
      is CountdownAfterPlea -> {
        it.isActiveOrDeactivating(now)
      }
    }
  }

  sealed class Creator {
    class Countdown(val duration: Duration) : Creator() {}
    class CountdownAfterPlea(val intervalFromPleaTillDeactivation: Duration) : Creator() {}

    fun create(): RuleEnabler {
      return when (this) {
        is Countdown -> {
          RuleEnabler.Countdown(CountdownConditional.create(duration))
        }
        is CountdownAfterPlea -> {
          RuleEnabler.CountdownAfterPlea(CountdownAfterPleaConditional.create(intervalFromPleaTillDeactivation))
        }
      }
    }
  }

  enum class Variant {
    Countdown,
    CountdownAfterPlea,
  }
}