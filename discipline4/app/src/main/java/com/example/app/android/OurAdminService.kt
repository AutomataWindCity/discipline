package com.example.app

import android.app.Service
import android.app.admin.DeviceAdminService
import android.content.Intent
import android.os.IBinder
import android.os.Binder
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*
import java.util.concurrent.atomic.AtomicBoolean
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock


public class OurAdminService : Service() {
  private val isServiceRunning = AtomicBoolean(false)

  private val backgroundScope = CoroutineScope(SupervisorJob() + Dispatchers.IO)

  private var synchronizationJob: Job? = null
  private var synchronizationInterval = 30 * 60 * 1000L

  private val state = State.createDefault()
  private val stateMutex = Mutex()

  private val internalStateEvents = MutableSharedFlow<State.Event>()
  val stateEvents: SharedFlow<State.Event> = internalStateEvents.asSharedFlow()

  private val internalStateFlow = MutableStateFlow<State>(State.createDefault())
  val stateFlow = internalStateFlow.asStateFlow()
  
  override fun onCreate() {
    super.onCreate()
    isServiceRunning.set(true)
    startSynchronizationLoop()
  }
  
  override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
    return Service.START_STICKY
  }
  
  private fun startSynchronizationLoop() {
    job?.cancel()
    job = null
    job = backgroundScope.launch {
      while (isActive && isServiceRunning.get()) {
        try {
          synchronize()          
          delay(synchronizationInterval)
        } catch (e: CancellationException) {
          // Job was cancelled - exit gracefully
          break
        } catch (e: Exception) {
          // Handle errors, but continue the loop
          processSynchronizationLoopError(e)
          // Wait a bit before retrying to avoid tight error loops
          delay(10_000)
        }
      }
    }
  }

  private fun stopSynchronizationLoop() {
    job?.cancel()
    job = null
  }

  private fun restartSynchronizationLoop() {
    if (isServiceRunning.get()) {
      startSynchronizationLoop()
    }
  }
  
  /**
   * The actual synchronization function with disk I/O
   */
  private suspend fun synchronize() {
    // Use withContext(Dispatchers.IO) to ensure disk operations happen on IO dispatcher
    withContext(Dispatchers.IO) {
      try {
        
      } catch (e: Exception) {
        throw e
      }
    }
  }
  
  // /**
  //  * Creates a flow that emits a signal at each interval
  //  */
  // private fun syncTickerFlow(intervalMs: Long): Flow<Boolean> = flow {
  //   // Initial delay before first sync
  //   delay(5_000)
    
  //   while (true) {
  //     emit(true)
  //     delay(intervalMs)
  //   }
  // }

  // /**
  //  * Alternative: Using Flow for more control and monitoring
  //  */
  // private fun startPeriodicSyncWithFlow() {
  //   stopSynchronizationLoop()
    
  //   job = scope.launch {
  //     syncTickerFlow(syncIntervalMs)
  //       .catch { e -> 
  //         handleSyncError(e) 
  //       }
  //       .collect { shouldSync ->
  //         if (shouldSync && isServiceRunning.get()) {
  //           synchronize()
  //         }
  //       }
  //   }
  // }
 
  
  /**
   * Handle synchronization errors
   */
  private fun processSynchronizationLoopError(error: Throwable) {
    android.util.Log.e("OurAdminService", "Periodic sync error", error)
    
    // Optional: Notify error to UI or other components
    val intent = Intent("com.example.app.SYNC_ERROR")
    intent.putExtra("error", error.message)
    sendBroadcast(intent)
  }
  
  // /**
  //  * Notify that sync completed
  //  */
  // private fun notifySyncCompleted(success: Boolean, errorMessage: String?) {
  //   val intent = Intent("com.example.app.SYNC_COMPLETED")
  //   intent.putExtra("success", success)
  //   intent.putExtra("timestamp", System.currentTimeMillis())
  //   errorMessage?.let { intent.putExtra("error", it) }
  //   sendBroadcast(intent)
  // }
  
  override fun onDestroy() {
    isServiceRunning.set(false)
    
    // Cancel all coroutines
    backgroundScope.cancel()
    
    super.onDestroy()
  }
  
  // /**
  //  * Optional: Get sync status
  //  */
  // fun getSyncStatus(): SyncStatus {
  //   return SyncStatus(
  //     isRunning = job?.isActive == true,
  //     lastSyncTime = getLastSyncTime(),
  //     nextSyncTime = getNextSyncTime()
  //   )
  // }
  
  // private fun getLastSyncTime(): Long {
  //   // Retrieve from SharedPreferences or database
  //   return getSharedPreferences("sync_prefs", MODE_PRIVATE)
  //     .getLong("last_sync_time", 0)
  // }
  
  // private fun getNextSyncTime(): Long {
  //   if (job?.isActive != true) return 0
  //   return getLastSyncTime() + syncIntervalMs
  // }
  
  // data class SyncStatus(
  //   val isRunning: Boolean,
  //   val lastSyncTime: Long,
  //   val nextSyncTime: Long
  // )

  fun getSomeState() {

  }

  fun addTodo(name: String) {
    backgroundScope.launch {
      val id = System.currentTimeMillis()
      val todo = Todo(name)

      internalStateFlow.update { state -> 
        state.items.set(id, todo)
        state
      }

      internalStateEvents.emit(State.Event.Added(id, todo))
    }
  }

  private val binder = ServiceBinder()

  inner class ServiceBinder : Binder() {
    suspend fun getTodo(id: Long): Todo? {
      return stateMutex.withLock {
        state.items.get(id)
      }
    }
  }

  public override fun onBind(intent: Intent?): IBinder? {
    return binder
  }
}