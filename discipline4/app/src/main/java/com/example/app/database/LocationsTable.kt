package com.example.app.database

import com.example.app.*

// TODO: Complete implementing this table using the pattern establish by other tables in this directory:
//   - add a writeCreateTable method
//   - add a writeInsertLocation() method
//   - add a writeInsertLocation method

object AlwaysRulesLocationsTable {
  const val TABLE = "AlwaysRulesLocations"
  
  const val ID = "id"
  const val VARIANT = "variant"
  const val DATA_1 = "data_1"

  const val ID_INDEX = 0
  const val VARIANT_INDEX = 0
  const val DATA_1_INDEX = 0

  fun writeCreateTable(buffer: Buffer) {
    buffer.code("""
      CREATE TABLE IF NOT EXISTS $TABLE (
        $ID INTEGER PRIMARY KEY,
        $VARIANT INTEGER NOT NULL,
        $DATA_1
      ) STRICT, WITHOUT ROWID;
    """)
  }

  fun 
}


class LocationId(val value: Long) {}
