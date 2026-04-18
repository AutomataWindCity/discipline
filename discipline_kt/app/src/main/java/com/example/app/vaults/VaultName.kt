package com.example.app

import com.example.app.*
import androidx.room.Entity

/**
 * Represents a vault name with length constraints
 */
@Entity
@JvmInline
public value class VaultName private constructor(val value: String) {
  
  companion object {
    const val MINIMUM_LENGTH = 1
    const val MAXIMUM_LENGTH = 300
    
    /**
     * Creates a VaultName with validation
     */
    fun create(string: String): Tried<VaultName, TextualError> {
      if (string.length < MINIMUM_LENGTH || string.length > MAXIMUM_LENGTH) {
        return Tried.failure(
          TextualError.create("Creating VaultName from string")
            .addMessage("String violates length invariants")
            .addNumberAttachment("Minimum length", MINIMUM_LENGTH.toDouble())
            .addNumberAttachment("Maximum length", MAXIMUM_LENGTH.toDouble())
            .addNumberAttachment("Provided string length", string.length.toDouble())
            .addStringAttachment("Provided string", string)
        )
      }
      
      return Tried.success(VaultName(string))
    }

    fun createOrThrow(string: String): VaultName {
      if (string.length < MINIMUM_LENGTH || string.length > MAXIMUM_LENGTH) {
        throw TextualError.create("Creating VaultName from string")
          .addMessage("String violates length invariants")
          .addNumberAttachment("Minimum length", MINIMUM_LENGTH.toDouble())
          .addNumberAttachment("Maximum length", MAXIMUM_LENGTH.toDouble())
          .addNumberAttachment("Provided string length", string.length.toDouble())
          .addStringAttachment("Provided string", string)
      }

      return VaultName(string)
    }
  }
  
  fun isEqualTo(other: VaultName): Boolean {
    return value == other.value
  }
  
  fun length(): Int {
    return value.length
  }

  override fun toString(): String {
    return "VaultName($value)"
  }
}
