package com.example.app.database

import com.example.app.*
import android.database.Cursor
import android.database.sqlite.SQLiteDatabase

@JvmInline
value class DatabaseConnection(
  val connection: SQLiteDatabase,
) {
  fun queryOrThrow(string: String): Cursor {
    return connection.rawQuery(string, emptyArray())
  }

  fun insertOrThrow(string: String): Long {
    val id = connection.insertOrThrow(string, null, null)
    if (id == -1L) {
      throw TextualError
        .create("DatabaseConnection inserting a row")
        .addMessage("android.database.sqlite.SQLiteDatabase.insertOrThrow retuned a -1 indicating an error")
    }

    return id
  }

  fun execSqlOrThrow(sql: String) {
    connection.execSQL(sql)
  }
}
