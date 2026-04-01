package com.example.app

import androidx.room.Database
import androidx.room.Room
import androidx.room.RoomDatabase
import android.content.Context
import com.example.app.*

interface Scalar<T> {
  fun read(): Unit
  fun write(): Unit
}

object IntScalar : Scalar<Int> {
  override fun read(): Unit {

  }

  override fun write(): Unit {

  }
}

object IntScalar : Scalar<Int> {
  override fun read(): Unit {

  }

  override fun write(): Unit {

  }
}

// @Database(entities = [CountdownRule::class], version = 1)
// public abstract class AppDatabase : RoomDatabase() {
//     abstract fun userDao(): CountdownRuleDao

//     companion object {
//         @Volatile
//         private var INSTANCE: AppDatabase? = null

//         fun getInstance(context: Context): AppDatabase {
//             return INSTANCE ?: synchronized(this) {
//                 val instance = Room.databaseBuilder(
//                     context.applicationContext,
//                     AppDatabase::class.java,
//                     "app_database"
//                 ).build()
//                 INSTANCE = instance
//                 instance
//             }
//         }
//     }
// }