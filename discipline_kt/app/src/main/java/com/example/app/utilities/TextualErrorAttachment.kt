package com.example.app

/** Represents a named attachment with a string value for error context */
public data class TextualErrorAttachment private constructor(val name: String, val value: String) {
  companion object {
    fun create(name: String, value: String): TextualErrorAttachment {
      return TextualErrorAttachment(name, value)
    }
  }
}
