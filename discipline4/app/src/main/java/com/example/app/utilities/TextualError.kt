package com.example.app

import java.io.StringWriter
import java.io.PrintWriter
import com.example.app.TextualErrorContext

/**
 * Main error type with context stacking capability
 */
public class TextualError private constructor(
    var context: TextualErrorContext,
    val earlierContexts: MutableList<TextualErrorContext> = mutableListOf()
) : Throwable() {
    companion object {
        fun create(action: String): TextualError {
            return TextualError(TextualErrorContext.create(action))
        }
        
        fun createEmpty(): TextualError {
           return TextualError(TextualErrorContext.create(""))
        }

        fun fromContext(context: TextualErrorContext): TextualError {
            return TextualError(context)
        }
    }
    
    // Context management
    fun changeContext(newContextAction: String): TextualError {
        earlierContexts.add(context)
        context = TextualErrorContext.create(newContextAction)
        return this
    }
    
    fun changeContextFrom(otherError: TextualError): TextualError {
        earlierContexts.add(context)
        context = otherError.context
        earlierContexts.addAll(otherError.earlierContexts)
        return this
    }
    
    // Message addition
    fun addMessage(message: String): TextualError {
        context.errorMessages.add(message)
        return this
    }
    
    fun addInfo(message: String): TextualError {
        context.infoMessages.add(message)
        return this
    }
    
    // Attachment methods
    fun addStringAttachment(name: String, value: String): TextualError {
        val escapedValue = value
            .replace("\"", "\\\"")
            .replace("\n", "\\n")
            .replace("\r", "\\r")
            .replace("\t", "\\t")
        context.attachments.add(
            TextualErrorAttachment.create(name, "\"$escapedValue\"")
        )
        return this
    }
    
    fun addErrorAttachment(name: String, value: Throwable): TextualError {
        val sw = StringWriter()
        val pw = PrintWriter(sw)
        value.printStackTrace(pw)
        context.attachments.add(
            TextualErrorAttachment.create(name, sw.toString())
        )

    //     return buildString {
    //     appendLine(throwable::class.java.name)
    //     appendLine(throwable.message ?: "No message")
    //     appendLine(throwable.stackTrace.joinToString("\n") { it.toString() })
    // }
        return this
    }
    
    fun addNumberAttachment(name: String, value: Number): TextualError {
        context.attachments.add(
            TextualErrorAttachment.create(name, value.toString())
        )
        return this
    }
    fun addLongAttachment(name: String, value: Long): TextualError {
        context.attachments.add(
            TextualErrorAttachment.create(name, value.toString())
        )
        return this
    }
    fun addIntAttachment(name: String, value: Int): TextualError {
        context.attachments.add(
            TextualErrorAttachment.create(name, value.toString())
        )
        return this
    }
    fun addUIntAttachment(name: String, value: UInt): TextualError {
        context.attachments.add(
            TextualErrorAttachment.create(name, value.toString())
        )
        return this
    }
    
    fun addBooleanAttachment(name: String, value: Boolean): TextualError {
        context.attachments.add(
            TextualErrorAttachment.create(name, if (value) "true" else "false")
        )
        return this
    }
    
    fun addNullAttachment(name: String): TextualError {
        context.attachments.add(
            TextualErrorAttachment.create(name, "null")
        )
        return this
    }
    
    fun addPrimitiveAttachment(
        name: String,
        value: Any?
    ): TextualError {
        return when (value) {
            null -> addNullAttachment(name)
            is String -> addStringAttachment(name, value)
            is Number -> addNumberAttachment(name, value)
            is Boolean -> addBooleanAttachment(name, value)
            else -> addStringAttachment(name, value.toString())
        }
    }
    
    fun addUnknownAttachment(name: String, value: Any?): TextualError {
        return when (value) {
            null -> addNullAttachment(name)
            is Throwable -> addErrorAttachment(name, value)
            is String -> addStringAttachment(name, value)
            is Number -> addNumberAttachment(name, value)
            is Boolean -> addBooleanAttachment(name, value)
            else -> addStringAttachment(name, value.toString())
        }
    }
    
    // Printing/rendering methods
    fun print(): String {
        return toString()
    }
    
    fun prettyPrint(): String {
        return buildString {
            appendLine("═══════════════════════════════════════")
            appendLine("ERROR DETAILS")
            appendLine("═══════════════════════════════════════")
            
            // Current context
            appendLine()
            appendLine("Current Context: ${context.action}")
            
            if (context.errorMessages.isNotEmpty()) {
                appendLine()
                appendLine("Error Messages:")
                context.errorMessages.forEachIndexed { index, msg ->
                    appendLine("  ${index + 1}. $msg")
                }
            }
            
            if (context.infoMessages.isNotEmpty()) {
                appendLine()
                appendLine("Info Messages:")
                context.infoMessages.forEachIndexed { index, msg ->
                    appendLine("  ${index + 1}. $msg")
                }
            }
            
            if (context.attachments.isNotEmpty()) {
                appendLine()
                appendLine("Attachments:")
                context.attachments.forEach { attachment ->
                    appendLine("  ${attachment.name}: ${attachment.value}")
                }
            }
            
            // Earlier contexts
            if (earlierContexts.isNotEmpty()) {
                appendLine()
                appendLine("───────────────────────────────────────────")
                appendLine("Earlier Contexts:")
                earlierContexts.forEachIndexed { index, ctx ->
                    appendLine()
                    appendLine("Context ${index + 1}: ${ctx.action}")
                    
                    if (ctx.errorMessages.isNotEmpty()) {
                        ctx.errorMessages.forEach { msg ->
                            appendLine("  ERROR: $msg")
                        }
                    }
                    
                    if (ctx.infoMessages.isNotEmpty()) {
                        ctx.infoMessages.forEach { msg ->
                            appendLine("  INFO: $msg")
                        }
                    }
                    
                    if (ctx.attachments.isNotEmpty()) {
                        ctx.attachments.forEach { attachment ->
                            appendLine("  ${attachment.name}: ${attachment.value}")
                        }
                    }
                }
            }
            
            appendLine()
            appendLine("═══════════════════════════════════════")
        }
    }
    
    override fun toString(): String {
        return buildString {
            append("TextualError[${context.action}]")
            
            val allMessages = context.errorMessages + context.infoMessages
            if (allMessages.isNotEmpty()) {
                append(": ${allMessages.first()}")
                if (allMessages.size > 1) {
                    append(" (+${allMessages.size - 1} more)")
                }
            }
            
            if (context.attachments.isNotEmpty()) {
                append(" (${context.attachments.size} attachments)")
            }
            
            if (earlierContexts.isNotEmpty()) {
                append(" [${earlierContexts.size} earlier contexts]")
            }
        }
    }
    
    // Utility methods
    fun hasErrors(): Boolean = context.errorMessages.isNotEmpty()
    
    fun getFirstErrorMessage(): String? = context.errorMessages.firstOrNull()
    
    fun getAllErrorMessages(): List<String> = context.errorMessages.toList()
    
    fun getAllContexts(): List<TextualErrorContext> {
        val contexts = mutableListOf<TextualErrorContext>()
        contexts.addAll(earlierContexts)
        contexts.add(context)
        return contexts
    }
    
    fun clear(): TextualError {
        context.errorMessages.clear()
        context.infoMessages.clear()
        context.attachments.clear()
        return this
    }
    
    // Builder pattern for fluent API
    class Builder(private val action: String) {
        private val messages = mutableListOf<String>()
        private val infos = mutableListOf<String>()
        private val attachments = mutableListOf<TextualErrorAttachment>()
        
        fun message(msg: String) = apply { messages.add(msg) }
        fun info(msg: String) = apply { infos.add(msg) }
        fun attachment(name: String, value: String) = apply {
            attachments.add(TextualErrorAttachment.create(name, value))
        }
        
        fun build(): TextualError {
            return TextualError.create(action).apply {
                messages.forEach { addMessage(it) }
                infos.forEach { addInfo(it) }
                attachments.forEach { context.addAttachment(it) }
            }
        }
    }
}

// Extension functions for easier use
fun TextualError.withMessage(message: String): TextualError = addMessage(message)
fun TextualError.withInfo(message: String): TextualError = addInfo(message)
fun TextualError.withAttachment(name: String, value: Any?): TextualError = 
    addUnknownAttachment(name, value)
fun TextualError.withContext(action: String): TextualError = changeContext(action)
