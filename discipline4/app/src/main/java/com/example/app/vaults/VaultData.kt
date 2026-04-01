package com.example.app

import com.example.app.*
import androidx.room.Entity

/**
 * Represents vault data content with length constraints
 */
@Entity
@JvmInline
public value class VaultData private constructor(val value: String) {
  
  companion object {
    const val MINIMUM_LENGTH = 1
    const val MAXIMUM_LENGTH = 500
    
    /**
     * Creates VaultData with validation
     */
    fun create(string: String): Tried<VaultData, TextualError> {
      if (string.length < MINIMUM_LENGTH || string.length > MAXIMUM_LENGTH) {
        return Tried.failure(
          TextualError.create("Creating VaultData from string")
            .addMessage("String violates length invariants")
            .addNumberAttachment("Minimum length", MINIMUM_LENGTH.toDouble())
            .addNumberAttachment("Maximum length", MAXIMUM_LENGTH.toDouble())
            .addNumberAttachment("Provided string length", string.length.toDouble())
            .addStringAttachment("Provided string", string)
        )
      }
      
      return Tried.success(VaultData(string))
    }
    
    /**
     * Creates VaultData without validation (use with caution)
     */
    fun construct(string: String): VaultData {
      return VaultData(string)
    }
  }
    
  fun isEqualTo(other: VaultData): Boolean {
    return value == other.value
  }
  
  fun length(): Int {
    return value.length
  }
  
  fun isEmpty(): Boolean {
    return value.isEmpty()
  }

  override fun toString(): String {
    return "VaultData($value)"
  }
}