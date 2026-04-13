package com.example.app

import android.app.admin.DeviceAdminReceiver
import android.content.Context
import android.content.Intent
import android.os.UserHandle
import com.example.app.OurAdminService

public class OurDeviceAdminReceiver : DeviceAdminReceiver() {
  override fun onProfileProvisioningComplete(context: Context, intent: Intent): Unit {
    super.onProfileProvisioningComplete(context, intent)
  }

  override fun onEnabled(context: Context, intent: Intent): Unit {
    super.onEnabled(context, intent)

    startServices(context)
  }

  override fun onDisabled(context: Context, intent: Intent): Unit {
    super.onDisabled(context, intent)

    stopServices(context)
  }

  override fun onUserStarted(context: Context, intent: Intent, user: UserHandle): Unit {
    super.onUserStarted(context, intent, user)
  }

  override fun onUserRemoved(context: Context, intent: Intent, user: UserHandle): Unit {
    super.onUserRemoved(context, intent, user)
  }

  override fun onUserStopped(context: Context, intent: Intent, user: UserHandle): Unit {
    super.onUserStopped(context, intent, user)
  }

  override fun onUserSwitched(context: Context, intent: Intent, user: UserHandle): Unit {
    super.onUserSwitched(context, intent, user)
  }

  private fun startServices(context: Context) {
    val intent = Intent(context, OurAdminService::class.java)
    context.startService(intent)
  }

  private fun stopServices(context: Context) {
    val intent = Intent(context, OurAdminService::class.java)
    context.stopService(intent)
  }
}