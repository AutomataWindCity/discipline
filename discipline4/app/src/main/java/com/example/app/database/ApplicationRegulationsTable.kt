package com.example.app.database

import com.example.app.*

object ApplicationRegulationsTable {
  const val TABLE = "ApplicationRegulations"

  const val ID = "id"
  const val APPLICATION_NAME = "application_name"
  
  fun writeCreateTable(buffer: Buffer) {
    buffer.code("""
      CREATE TABLE IF NOT EXISTS $TABLE (
        $ID PRIMARY KEY AUTOINCREMENT,
        $APPLICATION_NAME TEXT NOT NULL
      ) STRICT, WITHOUT ROWID;
    """)
  }

  fun writeInsert(
    buffer: Buffer,
    id: ApplicationRegulationId,
    applicationName: ApplicationName,
  ) {
    buffer.apply {
      code("INSERT INTO $TABLE VALUES (")
      applicationRegulationId(id)
      comma()
      applicationName(applicationName)
      code(");")
    }
  }

  fun insertOrThrow(
    database: DatabaseConnection,
  )
}