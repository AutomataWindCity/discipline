package com.yourpackage.discipline

import arrow.core.Either
import arrow.core.raise.either
import arrow.core.raise.raise

/**
 * Represents vault data content with length constraints
 */
@JvmInline
value class VaultData private constructor(val value: String) {
    
    companion object {
        const val MINIMUM_LENGTH = 1
        const val MAXIMUM_LENGTH = 500
        
        /**
         * Creates VaultData with validation
         */
        fun create(string: String): Either<TextualError, VaultData> = either {
            if (string.length < MINIMUM_LENGTH || string.length > MAXIMUM_LENGTH) {
                raise(
                    TextualError.create("Creating VaultData from string")
                        .addMessage("String violates length invariants")
                        .addNumberAttachment("Minimum length", MINIMUM_LENGTH.toDouble())
                        .addNumberAttachment("Maximum length", MAXIMUM_LENGTH.toDouble())
                        .addNumberAttachment("Provided string length", string.length.toDouble())
                        .addStringAttachment("Provided string", string)
                )
            }
            
            VaultData(string)
        }
        
        /**
         * Creates VaultData without validation (use with caution)
         */
        fun construct(string: String): VaultData = VaultData(string)
    }
    
    override fun toString(): String = value
    
    fun isEqualTo(other: VaultData): Boolean = value == other.value
    
    fun length(): Int = value.length
    
    fun isEmpty(): Boolean = value.isEmpty()
}