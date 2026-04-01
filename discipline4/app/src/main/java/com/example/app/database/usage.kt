package com.example.app

import android.content.Context
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import com.example.app.*

// In your Activity or ViewModel
class UserRepository(private val context: Context) {
  private val db = AppDatabase.getInstance(context)
  private val userDao = db.userDao()

    suspend fun addUser(user: CountdownRule) {
      userDao.insertUser(user)
    }

    suspend fun getUsers(): List<CountdownRule> {
        return userDao.getAllUsers()
    }
}

// // Example usage (ideally from a ViewModel using coroutines)
// fun example() {
//     val repository = UserRepository(context)
//     val newUser = User(userName = "JohnDoe", email = "john@example.com")

//     // Using a coroutine (e.g., with viewModelScope)
//     runBlocking {
//         launch(Dispatchers.IO) {
//             repository.addUser(newUser)
//             val users = repository.getUsers()
//             println(users)
//         }
//     }
// }

class CountdownRulesTable(
  val name: String,
) {

}