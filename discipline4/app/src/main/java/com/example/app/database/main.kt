package com.example.app

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.example.app.*

@Dao
public interface CountdownRuleDao {
  @Insert(onConflict = OnConflictStrategy.REPLACE)
  suspend fun insertUser(rule: CountdownRule)

  @Query("SELECT * FROM MainUserCountdownRules")
  suspend fun getAllUsers(): List<CountdownRule>

  @Query("SELECT * FROM MainUserCountdownRules WHERE userName LIKE :userName")
  suspend fun findUserByName(userName: String): CountdownRule?
}
