package com.example.app

import android.app.admin.DevicePolicyManager
import android.app.admin.DeviceAdminReceiver
import android.content.ComponentName
import android.content.Context
import android.os.UserManager
import android.provider.Settings
import java.util.concurrent.Executor
import com.example.app.*

class OurPolicyManager private constructor(
  /** This refers to the 'DisciplineDeviceAdminReceiver' class */
  private val admin: ComponentName,
  /** We get this from the "on*" methods of "DisciplineDeviceAdminReceiver" */
  private val context: Context,
  private val devicePolicyManager: DevicePolicyManager
) {
  companion object {
    private const val TAG = "OurPolicyManager"

    fun create(context: Context): Tried<OurPolicyManager, TextualError> {
      val admin = try {
        ComponentName(context, OurDeviceAdminReceiver::class.java)
      } catch (exception: Exception) {
        return Tried.failure(
          TextualError.create("Creating 'OurPolicyManager' from an Android 'Context'")
            .addMessage("Android threw an exception when creating a 'ComponentName' using 'Context' and 'DisciplineDeviceAdminReceiver'")
            .addUnknownAttachment("Exception", exception)
        )
      }

      val devicePolicyManager = try {
        // TODO: Check whether this is of type
        context.getSystemService(Context.DEVICE_POLICY_SERVICE) as DevicePolicyManager
      } catch (exception: Exception) {
        return Tried.failure(
          TextualError.create("Creating 'OurPolicyManager' from an Android 'Context'")
            .addMessage("Android threw an exception when retrieving the 'DevicePolicyManager' service using 'context.getSystemService(Context.DEVICE_POLICY_SERVICE)'")
            .addUnknownAttachment("Exception", exception)
        )
      }

      return Tried.success(OurPolicyManager(admin, context, devicePolicyManager))
    }

    fun createOrThrow(context: Context): OurPolicyManager {
      val admin = try {
        ComponentName(context, OurDeviceAdminReceiver::class.java)
      } catch (exception: Exception) {
        throw TextualError.create("Creating 'OurPolicyManager' from an Android 'Context'")
          .addMessage("Android threw an exception when creating a 'ComponentName' using 'Context' and 'DisciplineDeviceAdminReceiver'")
          .addUnknownAttachment("Exception", exception)
      }

      val devicePolicyManager = try {
        context.getSystemService(Context.DEVICE_POLICY_SERVICE) as DevicePolicyManager
      } catch (exception: Exception) {
        throw TextualError.create("Creating 'OurPolicyManager' from an Android 'Context'")
          .addMessage("Android threw an exception when retrieving the 'DevicePolicyManager' service using 'context.getSystemService(Context.DEVICE_POLICY_SERVICE)'")
          .addUnknownAttachment("Exception", exception)
      }

      return OurPolicyManager(admin, context, devicePolicyManager)
    }
  }

  // User Restriction Methods
  fun enableFactoryReset(): TextualError? {
    try {
      devicePolicyManager.clearUserRestriction(admin, UserManager.DISALLOW_FACTORY_RESET)
      return null
    } catch (exception: Exception) {
      return TextualError.create("OurPolicyManager disabling factory reset")
        .addMessage("Android's 'DevicePolicyManager.clearUserRestriction' threw an exception")
        .addStringAttachment("Restriction", UserManager.DISALLOW_FACTORY_RESET)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun disableFactoryReset(): TextualError? {
    try {
      devicePolicyManager.addUserRestriction(admin, UserManager.DISALLOW_FACTORY_RESET)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager disabling factory reset")
        .addMessage("Android's 'DevicePolicyManager.addUserRestriction' threw an exception")
        .addStringAttachment("Restriction", UserManager.DISALLOW_FACTORY_RESET)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun enableAppsControl(): TextualError? {
    try {
      devicePolicyManager.clearUserRestriction(admin, UserManager.DISALLOW_APPS_CONTROL)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager enabling app control")
        .addMessage("Android's 'DevicePolicyManager.clearUserRestriction' threw an exception")
        .addStringAttachment("Restriction", UserManager.DISALLOW_APPS_CONTROL)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun disableAppsControl(): TextualError? {
    try {
      devicePolicyManager.addUserRestriction(admin, UserManager.DISALLOW_APPS_CONTROL)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager disabling app control")
        .addMessage("Android's 'DevicePolicyManager.addUserRestriction' threw an exception")
        .addStringAttachment("Restriction", UserManager.DISALLOW_APPS_CONTROL)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun enableUsbFileTransfer(): TextualError? {
    try {
      devicePolicyManager.clearUserRestriction(admin, UserManager.DISALLOW_USB_FILE_TRANSFER)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager enabling USB file transfer")
        .addMessage("Android's 'DevicePolicyManager.clearUserRestriction' threw an exception")
        .addStringAttachment("Restriction", UserManager.DISALLOW_USB_FILE_TRANSFER)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun disableUsbFileTransfer(): TextualError? {
    try {
      devicePolicyManager.addUserRestriction(admin, UserManager.DISALLOW_USB_FILE_TRANSFER)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager disabling USB file transfer")
        .addMessage("Android's 'DevicePolicyManager.addUserRestriction' threw an exception")
        .addStringAttachment("Restriction", UserManager.DISALLOW_USB_FILE_TRANSFER)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun enableContentCapture(): TextualError? {
    try {
      devicePolicyManager.clearUserRestriction(admin, UserManager.DISALLOW_CONTENT_CAPTURE)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager enabling content capture")
        .addMessage("Android's 'DevicePolicyManager.clearUserRestriction' threw an exception")
        .addStringAttachment("Restriction", UserManager.DISALLOW_CONTENT_CAPTURE)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun disableContentCapture(): TextualError? {
    try {
      devicePolicyManager.addUserRestriction(admin, UserManager.DISALLOW_CONTENT_CAPTURE)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager disabling content capture")
        .addMessage("Android's 'DevicePolicyManager.addUserRestriction' threw an exception")
        .addStringAttachment("Restriction", UserManager.DISALLOW_CONTENT_CAPTURE)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun enableDebuggingFeatures(): TextualError? {
    try {
      devicePolicyManager.clearUserRestriction(admin, UserManager.DISALLOW_DEBUGGING_FEATURES)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager enabling debugging features")
        .addMessage("Android's 'DevicePolicyManager.clearUserRestriction' threw an exception")
        .addStringAttachment("Restriction", UserManager.DISALLOW_DEBUGGING_FEATURES)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun disableDebuggingFeatures(): TextualError? {
    try {
      devicePolicyManager.addUserRestriction(admin, UserManager.DISALLOW_DEBUGGING_FEATURES)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager disabling debugging features")
        .addMessage("Android's 'DevicePolicyManager.addUserRestriction' threw an exception")
        .addStringAttachment("Restriction", UserManager.DISALLOW_DEBUGGING_FEATURES)
        .addUnknownAttachment("Exception", exception)
    }
  }

  // Application Management Methods
  fun enableApplication(appPackageName: String): TextualError? {
    try {
      devicePolicyManager.setApplicationHidden(admin, appPackageName, false)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager enabling application")
        .addMessage("Android's 'DevicePolicyManager.setApplicationHidden' threw an exception")
        .addStringAttachment("PackageName", appPackageName)
        .addBooleanAttachment("Hidden", false)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun disableApplication(appPackageName: String): TextualError? {
    try {
      devicePolicyManager.setApplicationHidden(admin, appPackageName, true)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager disabling application")
        .addMessage("Android's 'DevicePolicyManager.setApplicationHidden' threw an exception")
        .addStringAttachment("PackageName", appPackageName)
        .addBooleanAttachment("Hidden", true)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun enableApplicationUninstallation(appPackageName: String): TextualError? {
    try {
      devicePolicyManager.setUninstallBlocked(admin, appPackageName, false)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager enabling application uninstallation")
        .addMessage("Android's 'DevicePolicyManager.setUninstallBlocked' threw an exception")
        .addStringAttachment("PackageName", appPackageName)
        .addBooleanAttachment("Blocked", false)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun disableApplicationUninstallation(appPackageName: String): TextualError? {
    try {
      devicePolicyManager.setUninstallBlocked(admin, appPackageName, true)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager disabling application uninstallation")
        .addMessage("Android's 'DevicePolicyManager.setUninstallBlocked' threw an exception")
        .addStringAttachment("PackageName", appPackageName)
        .addBooleanAttachment("Blocked", true)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun isApplicationUninstallDisabled(appPackageName: String): Tried<Boolean, TextualError> {
    try {
      return Tried.success(
        devicePolicyManager.isUninstallBlocked(admin, appPackageName)
      )
    } catch (exception: Exception) {
      return Tried.failure(
        TextualError
          .create("OurPolicyManager checking if application uninstall is disabled")
          .addMessage("Android's 'DevicePolicyManager.isUninstallBlocked' threw an exception")
          .addStringAttachment("PackageName", appPackageName)
          .addUnknownAttachment("Exception", exception)
      )
    }
  }

  fun isApplicationUninstallDisabledOrThrow(appPackageName: String): Boolean {
    try {
      return devicePolicyManager.isUninstallBlocked(admin, appPackageName)
    } catch (exception: Exception) {
      throw TextualError
        .create("OurPolicyManager checking if application uninstall is disabled")
        .addMessage("Android's 'DevicePolicyManager.isUninstallBlocked' threw an exception")
        .addStringAttachment("PackageName", appPackageName)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun enableInstallApplications(): TextualError? {
    try {
      devicePolicyManager.clearUserRestriction(admin, UserManager.DISALLOW_INSTALL_APPS)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager enabling install applications")
        .addMessage("Android's 'DevicePolicyManager.clearUserRestriction' threw an exception")
        .addStringAttachment("Restriction", UserManager.DISALLOW_INSTALL_APPS)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun disableInstallApplications(): TextualError? {
    try {
      devicePolicyManager.addUserRestriction(admin, UserManager.DISALLOW_INSTALL_APPS)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager disabling install applications")
        .addMessage("Android's 'DevicePolicyManager.addUserRestriction' threw an exception")
        .addStringAttachment("Restriction", UserManager.DISALLOW_INSTALL_APPS)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun enableSafeBoot(): TextualError? {
    try {
      devicePolicyManager.clearUserRestriction(admin, UserManager.DISALLOW_SAFE_BOOT)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager enabling safe boot")
        .addMessage("Android's 'DevicePolicyManager.clearUserRestriction' threw an exception")
        .addStringAttachment("Restriction", UserManager.DISALLOW_SAFE_BOOT)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun disableSafeBoot(): TextualError? {
    try {
      devicePolicyManager.addUserRestriction(admin, UserManager.DISALLOW_SAFE_BOOT)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager disabling safe boot")
        .addMessage("Android's 'DevicePolicyManager.addUserRestriction' threw an exception")
        .addStringAttachment("Restriction", UserManager.DISALLOW_SAFE_BOOT)
        .addUnknownAttachment("Exception", exception)
    }
  }

  // System Settings Methods
  fun turnWiFiOn(): TextualError? {
    try {
      devicePolicyManager.setSystemSetting(admin, Settings.Global.WIFI_ON, "1")
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager turning WiFi on")
        .addMessage("Android's 'DevicePolicyManager.setSystemSetting' threw an exception")
        .addStringAttachment("Setting", Settings.Global.WIFI_ON)
        .addStringAttachment("Value", "1")
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun turnWiFiOff(): TextualError? {
    try {
      devicePolicyManager.setSystemSetting(admin, Settings.Global.WIFI_ON, "0")
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager turning WiFi off")
        .addMessage("Android's 'DevicePolicyManager.setSystemSetting' threw an exception")
        .addStringAttachment("Setting", Settings.Global.WIFI_ON)
        .addStringAttachment("Value", "0")
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun enableDevelopmentSettings(): TextualError? {
    try {
      devicePolicyManager.setSystemSetting(admin, Settings.Global.DEVELOPMENT_SETTINGS_ENABLED, "1")
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager enabling development settings")
        .addMessage("Android's 'DevicePolicyManager.setSystemSetting' threw an exception")
        .addStringAttachment("Setting", Settings.Global.DEVELOPMENT_SETTINGS_ENABLED)
        .addStringAttachment("Value", "1")
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun disableDevelopmentSettings(): TextualError? {
    try {
      devicePolicyManager.setSystemSetting(admin, Settings.Global.DEVELOPMENT_SETTINGS_ENABLED, "0")
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager disabling development settings")
        .addMessage("Android's 'DevicePolicyManager.setSystemSetting' threw an exception")
        .addStringAttachment("Setting", Settings.Global.DEVELOPMENT_SETTINGS_ENABLED)
        .addStringAttachment("Value", "0")
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun enableStatusBar(): TextualError? {
    try {
      devicePolicyManager.setStatusBarDisabled(admin, false)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager enabling status bar")
        .addMessage("Android's 'DevicePolicyManager.setStatusBarDisabled' threw an exception")
        .addBooleanAttachment("Disabled", false)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun disableStatusBar(): TextualError? {
    try {
      devicePolicyManager.setStatusBarDisabled(admin, true)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager disabling status bar")
        .addMessage("Android's 'DevicePolicyManager.setStatusBarDisabled' threw an exception")
        .addBooleanAttachment("Disabled", true)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun enableUsbDebugging(): TextualError? {
    try {
      devicePolicyManager.setSystemSetting(admin, Settings.Global.ADB_ENABLED, "1")
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager enabling USB debugging")
        .addMessage("Android's 'DevicePolicyManager.setSystemSetting' threw an exception")
        .addStringAttachment("Setting", Settings.Global.ADB_ENABLED)
        .addStringAttachment("Value", "1")
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun disableUsbDebugging(): TextualError? {
    try {
      devicePolicyManager.setSystemSetting(admin, Settings.Global.ADB_ENABLED, "0")
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager disabling USB debugging")
        .addMessage("Android's 'DevicePolicyManager.setSystemSetting' threw an exception")
        .addStringAttachment("Setting", Settings.Global.ADB_ENABLED)
        .addStringAttachment("Value", "0")
        .addUnknownAttachment("Exception", exception)
    }
  }

  // Advanced Methods
  fun clearApplicationUserData(
    appPackageName: String,
    executor: Executor,
    callback: (Boolean) -> Unit
  ): TextualError? {
    try {
      devicePolicyManager.clearApplicationUserData(
        admin,
        appPackageName,
        executor,
        DevicePolicyManager.OnClearApplicationUserDataListener { _, success ->
          callback(success)
        }
      )
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager clearing application user data")
        .addMessage("Android's 'DevicePolicyManager.clearApplicationUserData' threw an exception")
        .addStringAttachment("PackageName", appPackageName)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun enableSystemApplication(appPackageName: String): TextualError? {
    try {
      devicePolicyManager.enableSystemApp(admin, appPackageName)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager enabling system application")
        .addMessage("Android's 'DevicePolicyManager.enableSystemApp' threw an exception")
        .addStringAttachment("PackageName", appPackageName)
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun suspendPackages(packageNames: Array<String>): TextualError? {
    try {
      devicePolicyManager.setPackagesSuspended(admin, packageNames, true)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager suspending packages")
        .addMessage("Android's 'DevicePolicyManager.setPackagesSuspended' threw an exception")
        .addStringAttachment("PackageCount", packageNames.size.toString())
        .addUnknownAttachment("Exception", exception)
    }
  }

  fun unsuspendPackages(packageNames: Array<String>): TextualError? {
    try {
      devicePolicyManager.setPackagesSuspended(admin, packageNames, false)
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager unsuspending packages")
        .addMessage("Android's 'DevicePolicyManager.setPackagesSuspended' threw an exception")
        .addStringAttachment("PackageCount", packageNames.size.toString())
        .addUnknownAttachment("Exception", exception)
    }
  }

  /**
   * Required permissions: "LOCK_DEVICE".
   */
  fun lock(): TextualError? {
    try {
      devicePolicyManager.lockNow()
      return null
    } catch (exception: Exception) {
      return TextualError
        .create("OurPolicyManager locking device")
        .addMessage("Android's 'DevicePolicyManager.lockNow' threw an exception")
        .addUnknownAttachment("Exception", exception)
    }
  }
}
