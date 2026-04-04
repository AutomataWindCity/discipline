package com.example.app.database

import android.database.Cursor
import android.database.sqlite.SQLiteDatabase
import com.example.app.*
import com.example.app.database.Buffer

fun Int.writeSql(buffer: Buffer) {
  buffer.write(toString())
}

fun Long.writeSql(buffer: Buffer) {
  buffer.write(toString())
}

fun Boolean.writeSql(buffer: Buffer) {
  buffer.write(if (this) {
    "TRUE"
  } else {
    "FALSE"
  })
}

fun String.writeSql(buffer: Buffer) {
  // TODO
  buffer.write(this)
}

fun Duration.writeSql(buffer: Buffer) {
  toTotalMilliseconds().writeSql(buffer)
}

fun Instant.writeSql(buffer: Buffer) {
  toElapsedTime().writeSql(buffer)
}

fun Time.writeSql(buffer: Buffer) {
  toTimestamp().writeSql(buffer)
}

fun VaultName.writeSql(buffer: Buffer) {
  toString().writeSql(buffer)
}

fun VaultData.writeSql(buffer: Buffer) {
  toString().writeSql(buffer)
}

fun Int.toSql(): StringBuilder {
  val buffer = Buffer()
  writeSql(buffer)
  return buffer.builder()
}

fun Long.toSql(): StringBuilder {
  val buffer = Buffer()
  writeSql(buffer)
  return buffer.builder()
}

fun Boolean.toSql(): StringBuilder {
  val buffer = Buffer()
  writeSql(buffer)
  return buffer.builder()
}

fun String.toSql(): StringBuilder {
  val buffer = Buffer()
  writeSql(buffer)
  return buffer.builder()
}

fun Duration.toSql(): StringBuilder {
  val buffer = Buffer()
  writeSql(buffer)
  return buffer.builder()
}

fun Instant.toSql(): StringBuilder {
  val buffer = Buffer()
  writeSql(buffer)
  return buffer.builder()
}

fun Time.toSql(): StringBuilder {
  val buffer = Buffer()
  writeSql(buffer)
  return buffer.builder()
}

fun VaultName.toSql(): StringBuilder {
  val buffer = Buffer()
  writeSql(buffer)
  return buffer.builder()
}

fun VaultData.toSql(): StringBuilder {
  val buffer = Buffer()
  writeSql(buffer)
  return buffer.builder()
}

fun UuidV4.toSql(): StringBuilder {
  return StringBuilder()
}

fun SQLiteDatabase.ourQuery(string: String): Cursor {
  try {
    return rawQuery(string, arrayOf())
  } catch (exception: Throwable) {
    throw TextualError
      .create("Running a SQLite query")
      .addErrorAttachment("Exception", exception)
  }
}

fun Cursor.ourGetStringOrThrow(index: Int): String {
  return ""
}
fun Cursor.ourGetIntOrThrow(index: Int): Int {
  return 0
}
fun Cursor.ourGetLongOrThrow(index: Int): Long {
  return 0
}
fun Cursor.ourGetBooleanOrThrow(index: Int): Boolean {
  return true
}
fun Cursor.ourMoveToNext(): Boolean {
  try {
    return moveToNext()
  } catch (e: Throwable) {
    throw TextualError
      .create("Moving a SQLite cursor to the next row")
      .addErrorAttachment("Exception", e)
  }
}

fun createInstantFromSqlOrThrow(value: Long): Instant {
  throw TextualError.create("action")
}
fun createDurationFromSqlOrThrow(value: Long): Duration {
  throw TextualError.create("action")
}
fun createUuidFromSqlOrThrow(value: String): UuidV4 {
  throw TextualError.create("action")
}
fun createBooleanFromSqlOrThrow(value: Int): Boolean {
  throw TextualError.create("")
}
