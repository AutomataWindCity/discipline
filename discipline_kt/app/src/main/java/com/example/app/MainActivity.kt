package com.example.app

import android.os.Bundle
import android.os.IBinder
import androidx.appcompat.app.AppCompatActivity
import android.content.ServiceConnection
import android.content.ComponentName
import android.content.Intent
import android.content.Context
import android.widget.Toast
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.launch

class MainActivity : AppCompatActivity() {

  private lateinit var ourAdminService: OurAdminService
  private var isBound = false
  private val serviceConnection = object : ServiceConnection {
    override fun onServiceConnected(name: ComponentName?, service: IBinder?) {
      ourAdminService = (service as OurAdminService.LocalBinder).getService()
      isBound = true
      observeServiceState()
    }

    override fun onServiceDisconnected(name: ComponentName?) {
      isBound = false
    }
  }


  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    setContentView(R.layout.activity_main)

    // Bind to service
    bindService(
      Intent(this, OurAdminService::class.java),
      serviceConnection,
      Context.BIND_AUTO_CREATE
    )
  }

  private fun observeServiceState() {
    lifecycleScope.launch {
      ourAdminService.stateFlow.collect { state ->
        updateUI(state)
      }
    }

    lifecycleScope.launch {
      ourAdminService.sharedStateEvents.collect { event ->
        when (event) {
          is State.Event.Synced -> {
            showToast("Synced ${event.count} todos")
          }
          is State.Event.Added -> {
            showToast("Added: ${event.todo.name}")
          }
          is State.Event.Updated -> {
            showToast("Todo updated")
          }
          is State.Event.Deleted -> {
            showToast("Todo deleted")
          }
          is State.Event.Error -> {
            showToast("Error: ${event.message}")
          }
        }
      }
    }
  }
  
  fun onAddButtonClick(title: String) {
    if (isBound) {
      ourAdminService.addTodo(title)
    }
  }

  private fun updateUI(state: State) {
    title = "Discipline (${state.items.size})"
  }

  private fun showToast(message: String) {
    Toast.makeText(this, message, Toast.LENGTH_SHORT).show()
  }

  override fun onDestroy() {
    if (isBound) {
      unbindService(serviceConnection)
      isBound = false
    }
    super.onDestroy()
  }
}
