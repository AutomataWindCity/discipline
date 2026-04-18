package com.example.app.database

import com.example.app.*

object ApplicationRegulationsTable {
  const val TABLE = "ApplicationRegulations"

  const val ID = "id"
  const val APPLICATION_NAME = "application_name"
  
  fun writeCreateTable(buffer: Buffer) {
    buffer.code("""
      CREATE TABLE IF NOT EXISTS $TABLE (
        $ID INTEGER PRIMARY KEY,
        $APPLICATION_NAME TEXT NOT NULL
      ) STRICT;
    """)
  }

  fun writeInsert(
    buffer: Buffer,
    applicationName: ApplicationName,
  ) {
    buffer.apply {
      code("INSERT INTO $TABLE VALUES (NULL, ")
      applicationName(applicationName)
      code(");")
    }
  }

  fun insertOrThrow(
    database: DatabaseConnection,
    applicationName: ApplicationName,
  ): ApplicationRegulationId {
    val buffer = Buffer()
    writeInsert(buffer, applicationName)
    return ApplicationRegulationId(database.insertOrThrow(buffer.string()))
  }

  fun writeDelete(
    buffer: Buffer,
    applicationName: ApplicationName,
  ) {
    buffer.apply {
      code("DELETE FROM $TABLE WHERE $APPLICATION_NAME = ")
      applicationName(applicationName)
      code(";")
    }
  }

  fun deleteOrThrow(
    database: DatabaseConnection,
    applicationName: ApplicationName,
  ) {
    val buffer = Buffer()
    writeDelete(buffer, applicationName)
    database.execSqlOrThrow(buffer.string())
  }


  fun writeDeleteById(
    buffer: Buffer,
    regulationId: ApplicationRegulationId,
  ) {
    buffer.apply {
      code("DELETE FROM $TABLE WHERE $APPLICATION_NAME = ")
      applicationRegulationId(regulationId)
      code(";")
    }
  }

  fun deleteByIdOrThrow(
    database: DatabaseConnection,
    regulationId: ApplicationRegulationId,
  ) {
    val buffer = Buffer()
    writeDeleteById(buffer, regulationId)
    database.execSqlOrThrow(buffer.string())
  }
}