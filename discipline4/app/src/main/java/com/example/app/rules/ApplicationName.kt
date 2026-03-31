package com.yourpackage.discipline

import arrow.core.Either
import arrow.core.raise.either
import arrow.core.raise.raise

/**
 * Represents an application name with length constraints
 */
@JvmInline
value class ApplicationName private constructor(val value: String) {
    
    companion object {
        const val MINIMUM_LENGTH = 1
        const val MAXIMUM_LENGTH = 30
        
        /**
         * Creates an ApplicationName with validation
         */
        fun create(string: String): Either<TextualError, ApplicationName> = either {
            if (string.length < MINIMUM_LENGTH) {
                raise(
                    TextualError.create("Creating an ApplicationName from string")
                        .addMessage("String's length is less than the minimum valid length")
                        .addStringAttachment("String", string)
                        .addNumberAttachment("String length", string.length.toDouble())
                        .addNumberAttachment("Minimum valid length", MINIMUM_LENGTH.toDouble())
                )
            }
            
            if (string.length > MAXIMUM_LENGTH) {
                raise(
                    TextualError.create("Creating an ApplicationName from string")
                        .addMessage("String's length is greater than the maximum allowed length")
                        .addStringAttachment("String", string)
                        .addNumberAttachment("String length", string.length.toDouble())
                        .addNumberAttachment("Maximum valid length", MAXIMUM_LENGTH.toDouble())
                )
            }
            
            ApplicationName(string)
        }
        
        /**
         * Creates an ApplicationName without validation (use with caution)
         */
        fun construct(string: String): ApplicationName = ApplicationName(string)
    }
    
    override fun toString(): String = value
    
    fun isEqualTo(other: ApplicationName): Boolean = value == other.value
    
    fun length(): Int = value.length
}