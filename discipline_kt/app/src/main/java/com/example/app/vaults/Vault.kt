package com.example.app

public data class Vault private constructor(
  val name: VaultName,
  val data: VaultData,
  var protector: VaultProtector
) {
  companion object {
    // val MAXIMUM_PROTECTION_DURATION = Duration.fromDays(7).getOrThrow()
    
    fun create(
      name: VaultName,
      data: VaultData,
      protector: VaultProtector,
    ): Vault {
      // if (protection.getTotalDuration().isLongerThan(MAXIMUM_PROTECTION_DURATION)) {
      //   return Tried.failure(
      //     TextualError.create("Creating a Vault")
      //       .addMessage("Vault protection countdown's total duration is longer than the maximum protection duration")
      //       .addStringAttachment("Protection countdown duration", protection.getTotalDuration().toString())
      //       .addStringAttachment("Maximum protection duration", MAXIMUM_PROTECTION_DURATION.toString())
      //   )
      // }

      return Vault(name, data, protector)
    }

    // fun createOrThrow(
    //   name: VaultName,
    //   data: VaultData,
    //   protection: Countdown
    // ): Vault {
    //   if (protection.getTotalDuration().isLongerThan(MAXIMUM_PROTECTION_DURATION)) {
    //     throw TextualError.create("Creating a Vault")
    //       .addMessage("Vault protection countdown's total duration is longer than the maximum protection duration")
    //       .addStringAttachment("Protection countdown duration", protection.getTotalDuration().toString())
    //       .addStringAttachment("Maximum protection duration", MAXIMUM_PROTECTION_DURATION.toString())
    //   }

    //   return Vault(name, data, protection)
    // }
    
    // /**
    //  * Creates a Vault with validation
    //  */
    // fun createValidated(
    //   name: String,
    //   data: String,
    //   protectionDuration: Duration,
    //   protectionStart: Instant
    // ): Either<TextualError, Vault> = either {
    //   val vaultName = VaultName.create(name).bind()
    //   val vaultData = VaultData.create(data).bind()
      
    //   if (protectionDuration.isLongerThan(MAXIMUM_PROTECTION_DURATION)) {
    //     raise(
    //       TextualError.create("Creating Vault")
    //         .addMessage("Protection duration exceeds maximum")
    //         .addStringAttachment("Maximum duration", MAXIMUM_PROTECTION_DURATION.toString())
    //         .addStringAttachment("Provided duration", protectionDuration.toString())
    //     )
    //   }
      
    //   val countdown = Countdown.create(protectionStart, protectionDuration)
    //   Vault(vaultName, vaultData, countdown)
    // }
  }

  fun isProtected(now: Instant): Boolean {
    return protector.isRunning(now)
  }
  
  // /**
  //  * Extends the protection duration safely, respecting maximum limits
  //  */
  // // TODO: Check whether this is implemented correctly
  // fun extendProtectionByOrSetToMaxSafeValue(now: Instant, factor: Duration) {
  //   if (protection.isFinished(now)) {
  //     val newDuration = MAXIMUM_PROTECTION_DURATION.min(factor)
  //     val newProtection = Countdown.create(now, newDuration)
  //     protection = newProtection
  //     return
  //   } 
    
  //   val remaining = protection.getRemainingTimeOrZero(now)
  //   val maximum = MAXIMUM_PROTECTION_DURATION
    
  //   var actualFactor = factor
  //   if (actualFactor.saturatingAdd(remaining).isLongerThan(maximum)) {
  //     actualFactor = maximum.saturatingSub(remaining)
  //   }
    
  //   val newProtection = protection.copy(duration = protection.getTotalDuration().saturatingAdd(actualFactor))
  //   protection.set = newProtection
  // }

}
