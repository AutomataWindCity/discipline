package com.yourpackage.discipline

import arrow.core.Either
import arrow.core.raise.either
import arrow.core.raise.raise

/**
 * Represents a vault name with length constraints
 */
@JvmInline
value class VaultName private constructor(val value: String) {
    
    companion object {
        const val MINIMUM_LENGTH = 1
        const val MAXIMUM_LENGTH = 300
        
        /**
         * Creates a VaultName with validation
         */
        fun create(string: String): Either<TextualError, VaultName> = either {
            if (string.length < MINIMUM_LENGTH || string.length > MAXIMUM_LENGTH) {
                raise(
                    TextualError.create("Creating VaultName from string")
                        .addMessage("String violates length invariants")
                        .addNumberAttachment("Minimum length", MINIMUM_LENGTH.toDouble())
                        .addNumberAttachment("Maximum length", MAXIMUM_LENGTH.toDouble())
                        .addNumberAttachment("Provided string length", string.length.toDouble())
                        .addStringAttachment("Provided string", string)
                )
            }
            
            VaultName(string)
        }
        
        /**
         * Creates a VaultName without validation (use with caution, only for trusted input)
         */
        fun construct(string: String): VaultName = VaultName(string)
    }
    
    override fun toString(): String = value
    
    fun isEqualTo(other: VaultName): Boolean = value == other.value
    
    fun length(): Int = value.length
}