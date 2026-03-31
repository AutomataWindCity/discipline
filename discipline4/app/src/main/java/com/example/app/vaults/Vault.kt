package com.yourpackage.discipline

import arrow.core.Either
import arrow.core.raise.either
import arrow.core.raise.raise

/**
 * Represents a vault with a name, data, and protection countdown
 */
data class Vault private constructor(
    val name: VaultName,
    val data: VaultData,
    val protection: Countdown
) {
    companion object {
        val MAXIMUM_PROTECTION_DURATION: Duration = 
            Duration.fromMillisecondsOrThrow(1000L * 60 * 60 * 24 * 7)  // 7 days
        
        /**
         * Creates a Vault
         */
        fun create(
            name: VaultName,
            data: VaultData,
            protection: Countdown
        ): Vault = Vault(name, data, protection)
        
        /**
         * Creates a Vault with validation
         */
        fun createValidated(
            name: String,
            data: String,
            protectionDuration: Duration,
            protectionStart: Instant
        ): Either<TextualError, Vault> = either {
            val vaultName = VaultName.create(name).bind()
            val vaultData = VaultData.create(data).bind()
            
            if (protectionDuration.isLongerThan(MAXIMUM_PROTECTION_DURATION)) {
                raise(
                    TextualError.create("Creating Vault")
                        .addMessage("Protection duration exceeds maximum")
                        .addStringAttachment("Maximum duration", MAXIMUM_PROTECTION_DURATION.toString())
                        .addStringAttachment("Provided duration", protectionDuration.toString())
                )
            }
            
            val countdown = Countdown.create(protectionStart, protectionDuration)
            Vault(vaultName, vaultData, countdown)
        }
        
        fun construct(name: VaultName, data: VaultData, protection: Countdown): Vault = 
            Vault(name, data, protection)
    }
    
    fun getName(): VaultName = name
    fun getData(): VaultData = data
    fun getProtection(): Countdown = protection
    
    /**
     * Checks if the vault is currently protected
     */
    fun isProtected(now: Instant): Boolean = protection.isRunning(now)
    
    /**
     * Extends the protection duration safely, respecting maximum limits
     */
    fun extendProtectionByOrSetToMaxSafeValue(now: Instant, factor: Duration): Vault {
        return if (protection.isFinished(now)) {
            val newDuration = min(MAXIMUM_PROTECTION_DURATION, factor)
            val newProtection = Countdown.create(now, newDuration)
            copy(protection = newProtection)
        } else {
            val remaining = protection.getRemainingTimeOrZero(now)
            val maximum = MAXIMUM_PROTECTION_DURATION
            
            var actualFactor = factor
            if ((actualFactor + remaining).isLongerThan(maximum)) {
                actualFactor = maximum.minusOrZero(remaining)
            }
            
            val newProtection = protection.copy(duration = protection.getTotalDuration().plusOrMax(actualFactor))
            copy(protection = newProtection)
        }
    }
    
    /**
     * Locks the vault (sets protection to start now)
     */
    fun lock(now: Instant, duration: Duration): Vault {
        val newProtection = Countdown.create(now, duration)
        return copy(protection = newProtection)
    }
    
    /**
     * Unlocks the vault (sets protection to finished)
     */
    fun unlock(now: Instant): Vault {
        val expiredProtection = Countdown.create(
            now.minusOrZero(Duration.fromMillisecondsOrThrow(1000)),
            Duration.zero()
        )
        return copy(protection = expiredProtection)
    }
    
    /**
     * Updates vault data if not protected
     */
    fun updateData(now: Instant, newData: VaultData): Either<TextualError, Vault> = either {
        if (isProtected(now)) {
            raise(
                TextualError.create("Updating vault data")
                    .addMessage("Cannot update vault while it is protected")
                    .addStringAttachment("Vault name", name.toString())
                    .addStringAttachment("Protection remaining", protection.getRemainingTimeOrZero(now).toString())
            )
        }
        copy(data = newData).right().bind()
    }
    
    override fun toString(): String = "Vault(name=${name}, dataLength=${data.length()}, protected=${protection.isRunning(Instant.zero())})"
}