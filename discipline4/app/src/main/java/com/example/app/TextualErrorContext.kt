package com.example.app

import com.example.app.TextualErrorAttachment;

/** Represents a context for error tracking with action and messages */
public data class TextualErrorContext
private constructor(
  val action: String,
  val errorMessages: MutableList<String> = mutableListOf(),
  val infoMessages: MutableList<String> = mutableListOf(),
  val attachments: MutableList<TextualErrorAttachment> = mutableListOf()
) {
  companion object {
    fun create(action: String): TextualErrorContext {
      return TextualErrorContext(action)
    }
  }

  // Convenience methods for adding messages
  fun addErrorMessage(message: String) {
    errorMessages.add(message)
  }

  fun addInfoMessage(message: String) {
    infoMessages.add(message)
  }

  fun addAttachment(attachment: TextualErrorAttachment) {
    attachments.add(attachment)
  }

  // Check if context has any content
  fun isEmpty(): Boolean {
    return errorMessages.isEmpty() && infoMessages.isEmpty() && attachments.isEmpty()
  }
}
