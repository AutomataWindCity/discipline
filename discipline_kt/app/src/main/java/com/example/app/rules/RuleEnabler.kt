package com.example.app

import androidx.room.Entity

@Entity
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

    companion object {
      fun fromNumberOrThrow(number: Int): Variant {
        return when (number) {
          0 -> {
            Variant.Countdown
          }
          1 -> {
            Variant.CountdownAfterPlea
          }
          else -> {
            throw TextualError
              .create("Creating RuleEnablerVariant from number")
              .addMessage("Expected 0 (for Countdown) or 1 (for CountdownAfterPlea), but found $number")
          }
        }
      }
    }

    fun toNumber(): Int {
      TODO()
    }
  }
}