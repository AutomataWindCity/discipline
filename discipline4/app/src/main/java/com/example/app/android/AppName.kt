package com.example.app

import com.example.app.*

/**
 * Represents an application name with length constraints
 */
@JvmInline
public value class AppName private constructor(val value: String) {
  companion object {
    const val MINIMUM_LENGTH = 1
    const val MAXIMUM_LENGTH = 30
    
    /**
     * Creates an ApplicationName with validation
     */
    fun create(string: String): Tried<AppName, TextualError> {
      if (string.length < MINIMUM_LENGTH) {
        return Tried.failure(
          TextualError.create("Creating an ApplicationName from string")
            .addMessage("String's length is less than the minimum valid length")
            .addStringAttachment("String", string)
            .addNumberAttachment("String length", string.length.toDouble())
            .addNumberAttachment("Minimum valid length", MINIMUM_LENGTH.toDouble())
        )
      }
      
      if (string.length > MAXIMUM_LENGTH) {
        return Tried.failure(
          TextualError.create("Creating an ApplicationName from string")
            .addMessage("String's length is greater than the maximum allowed length")
            .addStringAttachment("String", string)
            .addNumberAttachment("String length", string.length.toDouble())
            .addNumberAttachment("Maximum valid length", MAXIMUM_LENGTH.toDouble())
        )
      }
      
      return Tried.success(AppName(string))
    }

    fun createOrThrow(string: String): AppName {
      if (string.length < MINIMUM_LENGTH) {
        throw TextualError.create("Creating an ApplicationName from string")
          .addMessage("String's length is less than the minimum valid length")
          .addStringAttachment("String", string)
          .addNumberAttachment("String length", string.length.toDouble())
          .addNumberAttachment("Minimum valid length", MINIMUM_LENGTH.toDouble())
      }

      if (string.length > MAXIMUM_LENGTH) {
        throw TextualError.create("Creating an ApplicationName from string")
          .addMessage("String's length is greater than the maximum allowed length")
          .addStringAttachment("String", string)
          .addNumberAttachment("String length", string.length.toDouble())
          .addNumberAttachment("Maximum valid length", MAXIMUM_LENGTH.toDouble())
      }

      return AppName(string)
    }
    
    /**
     * Creates an ApplicationName without validation (use with caution)
     */
    fun construct(string: String): AppName {
      return AppName(string)
    }
  }
  
  fun isEqualTo(other: AppName): Boolean {
    return value == other.value
  }
  
  fun length(): Int {
    return value.length
  }

  override fun toString(): String {
    return "AndroidAppName($value)"
  }
}
