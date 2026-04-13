package com.example.app

public sealed class VaultProtector {
  class Countdown(val it: CountdownConditional) : VaultProtector() {}
  class CountdownAfterPlea(val it: CountdownAfterPleaConditional) : VaultProtector() {}

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
    class Countdown(val it: Duration) : Creator() {}
    class CountdownAfterPlea(val it: Duration) : Creator() {}

    fun create(): VaultProtector {
      return when (this) {
        is Countdown -> {
          VaultProtector.Countdown(CountdownConditional.create(duration))
        }
        is CountdownAfterPlea -> {
          VaultProtector.CountdownAfterPlea(CountdownAfterPleaConditional.create(duration))
        }
      }
    }
  }
}