package com.example.app

import android.app.admin.DevicePolicyManager
import android.app.admin.DeviceAdminReceiver
import android.content.ComponentName
import android.content.Context
import android.os.UserManager
import android.provider.Settings
import arrow.core.Either
import arrow.core.raise.either
import arrow.core.raise.ensure
import java.util.concurrent.Executor
import com.example.app.DisciplineDeviceAdminReceiver
import com.example.app.TextualError

class DisciplinePolicyManager private constructor(
    /** This refers to the 'DisciplineDeviceAdminReceiver' class */
    private val admin: ComponentName,
    /** We get this from the "on*" methods of "DisciplineDeviceAdminReceiver" */
    private val context: Context,
    private val devicePolicyManager: DevicePolicyManager
) {
    companion object {
        private const val TAG = "DisciplinePolicyManager"

        fun create(context: Context): Either<TextualError, DisciplinePolicyManager> = either {
            val admin = Either.catch {
                ComponentName(context, DisciplineDeviceAdminReceiver::class.java)
            }.mapLeft { exception ->
                TextualError.create("Creating 'DisciplinePolicyManager' from an Android 'Context'")
                    .addMessage("Android threw an exception when creating a 'ComponentName' using 'Context' and 'DisciplineDeviceAdminReceiver'")
                    .addUnknownAttachment("Exception", exception)
            }.bind()

            val devicePolicyManager = Either.catch {
                context.getSystemService(Context.DEVICE_POLICY_SERVICE) as DevicePolicyManager
            }.mapLeft { exception ->
                TextualError.create("Creating 'DisciplinePolicyManager' from an Android 'Context'")
                    .addMessage("Android threw an exception when retrieving the 'DevicePolicyManager' service using 'context.getSystemService(Context.DEVICE_POLICY_SERVICE)'")
                    .addUnknownAttachment("Exception", exception)
            }.bind()

            DisciplinePolicyManager(admin, context, devicePolicyManager)
        }
    }

    // User Restriction Methods
    fun enableFactoryReset(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.clearUserRestriction(admin, UserManager.DISALLOW_FACTORY_RESET)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager enabling factory reset")
                .addMessage("Android's 'DevicePolicyManager.clearUserRestriction' threw an exception")
                .addStringAttachment("Restriction", UserManager.DISALLOW_FACTORY_RESET)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun disableFactoryReset(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.addUserRestriction(admin, UserManager.DISALLOW_FACTORY_RESET)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager disabling factory reset")
                .addMessage("Android's 'DevicePolicyManager.addUserRestriction' threw an exception")
                .addStringAttachment("Restriction", UserManager.DISALLOW_FACTORY_RESET)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun enableAppsControl(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.clearUserRestriction(admin, UserManager.DISALLOW_APPS_CONTROL)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager enabling app control")
                .addMessage("Android's 'DevicePolicyManager.clearUserRestriction' threw an exception")
                .addStringAttachment("Restriction", UserManager.DISALLOW_APPS_CONTROL)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun disableAppsControl(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.addUserRestriction(admin, UserManager.DISALLOW_APPS_CONTROL)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager disabling app control")
                .addMessage("Android's 'DevicePolicyManager.addUserRestriction' threw an exception")
                .addStringAttachment("Restriction", UserManager.DISALLOW_APPS_CONTROL)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun enableUsbFileTransfer(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.clearUserRestriction(admin, UserManager.DISALLOW_USB_FILE_TRANSFER)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager enabling USB file transfer")
                .addMessage("Android's 'DevicePolicyManager.clearUserRestriction' threw an exception")
                .addStringAttachment("Restriction", UserManager.DISALLOW_USB_FILE_TRANSFER)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun disableUsbFileTransfer(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.addUserRestriction(admin, UserManager.DISALLOW_USB_FILE_TRANSFER)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager disabling USB file transfer")
                .addMessage("Android's 'DevicePolicyManager.addUserRestriction' threw an exception")
                .addStringAttachment("Restriction", UserManager.DISALLOW_USB_FILE_TRANSFER)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun enableContentCapture(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.clearUserRestriction(admin, UserManager.DISALLOW_CONTENT_CAPTURE)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager enabling content capture")
                .addMessage("Android's 'DevicePolicyManager.clearUserRestriction' threw an exception")
                .addStringAttachment("Restriction", UserManager.DISALLOW_CONTENT_CAPTURE)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun disableContentCapture(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.addUserRestriction(admin, UserManager.DISALLOW_CONTENT_CAPTURE)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager disabling content capture")
                .addMessage("Android's 'DevicePolicyManager.addUserRestriction' threw an exception")
                .addStringAttachment("Restriction", UserManager.DISALLOW_CONTENT_CAPTURE)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun enableDebuggingFeatures(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.clearUserRestriction(admin, UserManager.DISALLOW_DEBUGGING_FEATURES)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager enabling debugging features")
                .addMessage("Android's 'DevicePolicyManager.clearUserRestriction' threw an exception")
                .addStringAttachment("Restriction", UserManager.DISALLOW_DEBUGGING_FEATURES)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun disableDebuggingFeatures(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.addUserRestriction(admin, UserManager.DISALLOW_DEBUGGING_FEATURES)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager disabling debugging features")
                .addMessage("Android's 'DevicePolicyManager.addUserRestriction' threw an exception")
                .addStringAttachment("Restriction", UserManager.DISALLOW_DEBUGGING_FEATURES)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    // Application Management Methods
    fun enableApplication(appPackageName: String): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.setApplicationHidden(admin, appPackageName, false)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager enabling application")
                .addMessage("Android's 'DevicePolicyManager.setApplicationHidden' threw an exception")
                .addStringAttachment("PackageName", appPackageName)
                .addBooleanAttachment("Hidden", false)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun disableApplication(appPackageName: String): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.setApplicationHidden(admin, appPackageName, true)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager disabling application")
                .addMessage("Android's 'DevicePolicyManager.setApplicationHidden' threw an exception")
                .addStringAttachment("PackageName", appPackageName)
                .addBooleanAttachment("Hidden", true)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun enableApplicationUninstallation(appPackageName: String): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.setUninstallBlocked(admin, appPackageName, false)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager enabling application uninstallation")
                .addMessage("Android's 'DevicePolicyManager.setUninstallBlocked' threw an exception")
                .addStringAttachment("PackageName", appPackageName)
                .addBooleanAttachment("Blocked", false)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun disableApplicationUninstallation(appPackageName: String): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.setUninstallBlocked(admin, appPackageName, true)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager disabling application uninstallation")
                .addMessage("Android's 'DevicePolicyManager.setUninstallBlocked' threw an exception")
                .addStringAttachment("PackageName", appPackageName)
                .addBooleanAttachment("Blocked", true)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun isApplicationUninstallDisabled(appPackageName: String): Either<TextualError, Boolean> = either {
        Either.catch {
            devicePolicyManager.isUninstallBlocked(admin, appPackageName)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager checking if application uninstall is disabled")
                .addMessage("Android's 'DevicePolicyManager.isUninstallBlocked' threw an exception")
                .addStringAttachment("PackageName", appPackageName)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun enableInstallApplications(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.clearUserRestriction(admin, UserManager.DISALLOW_INSTALL_APPS)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager enabling install applications")
                .addMessage("Android's 'DevicePolicyManager.clearUserRestriction' threw an exception")
                .addStringAttachment("Restriction", UserManager.DISALLOW_INSTALL_APPS)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun disableInstallApplications(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.addUserRestriction(admin, UserManager.DISALLOW_INSTALL_APPS)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager disabling install applications")
                .addMessage("Android's 'DevicePolicyManager.addUserRestriction' threw an exception")
                .addStringAttachment("Restriction", UserManager.DISALLOW_INSTALL_APPS)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun enableSafeBoot(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.clearUserRestriction(admin, UserManager.DISALLOW_SAFE_BOOT)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager enabling safe boot")
                .addMessage("Android's 'DevicePolicyManager.clearUserRestriction' threw an exception")
                .addStringAttachment("Restriction", UserManager.DISALLOW_SAFE_BOOT)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun disableSafeBoot(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.addUserRestriction(admin, UserManager.DISALLOW_SAFE_BOOT)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager disabling safe boot")
                .addMessage("Android's 'DevicePolicyManager.addUserRestriction' threw an exception")
                .addStringAttachment("Restriction", UserManager.DISALLOW_SAFE_BOOT)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    // System Settings Methods
    fun turnWiFiOn(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.setSystemSetting(admin, Settings.Global.WIFI_ON, "1")
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager turning WiFi on")
                .addMessage("Android's 'DevicePolicyManager.setSystemSetting' threw an exception")
                .addStringAttachment("Setting", Settings.Global.WIFI_ON)
                .addStringAttachment("Value", "1")
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun turnWiFiOff(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.setSystemSetting(admin, Settings.Global.WIFI_ON, "0")
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager turning WiFi off")
                .addMessage("Android's 'DevicePolicyManager.setSystemSetting' threw an exception")
                .addStringAttachment("Setting", Settings.Global.WIFI_ON)
                .addStringAttachment("Value", "0")
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun enableDevelopmentSettings(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.setSystemSetting(admin, Settings.Global.DEVELOPMENT_SETTINGS_ENABLED, "1")
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager enabling development settings")
                .addMessage("Android's 'DevicePolicyManager.setSystemSetting' threw an exception")
                .addStringAttachment("Setting", Settings.Global.DEVELOPMENT_SETTINGS_ENABLED)
                .addStringAttachment("Value", "1")
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun disableDevelopmentSettings(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.setSystemSetting(admin, Settings.Global.DEVELOPMENT_SETTINGS_ENABLED, "0")
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager disabling development settings")
                .addMessage("Android's 'DevicePolicyManager.setSystemSetting' threw an exception")
                .addStringAttachment("Setting", Settings.Global.DEVELOPMENT_SETTINGS_ENABLED)
                .addStringAttachment("Value", "0")
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun enableStatusBar(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.setStatusBarDisabled(admin, false)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager enabling status bar")
                .addMessage("Android's 'DevicePolicyManager.setStatusBarDisabled' threw an exception")
                .addBooleanAttachment("Disabled", false)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun disableStatusBar(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.setStatusBarDisabled(admin, true)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager disabling status bar")
                .addMessage("Android's 'DevicePolicyManager.setStatusBarDisabled' threw an exception")
                .addBooleanAttachment("Disabled", true)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun enableUsbDebugging(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.setSystemSetting(admin, Settings.Global.ADB_ENABLED, "1")
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager enabling USB debugging")
                .addMessage("Android's 'DevicePolicyManager.setSystemSetting' threw an exception")
                .addStringAttachment("Setting", Settings.Global.ADB_ENABLED)
                .addStringAttachment("Value", "1")
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun disableUsbDebugging(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.setSystemSetting(admin, Settings.Global.ADB_ENABLED, "0")
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager disabling USB debugging")
                .addMessage("Android's 'DevicePolicyManager.setSystemSetting' threw an exception")
                .addStringAttachment("Setting", Settings.Global.ADB_ENABLED)
                .addStringAttachment("Value", "0")
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    // Advanced Methods
    fun clearApplicationUserData(
        appPackageName: String,
        executor: Executor,
        callback: (Boolean) -> Unit
    ): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.clearApplicationUserData(
                admin,
                appPackageName,
                executor,
                DevicePolicyManager.OnClearApplicationUserDataListener { _, success ->
                    callback(success)
                }
            )
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager clearing application user data")
                .addMessage("Android's 'DevicePolicyManager.clearApplicationUserData' threw an exception")
                .addStringAttachment("PackageName", appPackageName)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun enableSystemApplication(appPackageName: String): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.enableSystemApp(admin, appPackageName)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager enabling system application")
                .addMessage("Android's 'DevicePolicyManager.enableSystemApp' threw an exception")
                .addStringAttachment("PackageName", appPackageName)
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun suspendPackages(packageNames: Array<String>): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.setPackagesSuspended(admin, packageNames, true)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager suspending packages")
                .addMessage("Android's 'DevicePolicyManager.setPackagesSuspended' threw an exception")
                .addStringAttachment("PackageCount", packageNames.size.toString())
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    fun unsuspendPackages(packageNames: Array<String>): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.setPackagesSuspended(admin, packageNames, false)
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager unsuspending packages")
                .addMessage("Android's 'DevicePolicyManager.setPackagesSuspended' threw an exception")
                .addStringAttachment("PackageCount", packageNames.size.toString())
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }

    /**
     * Required permissions: "LOCK_DEVICE".
     */
    fun lock(): Either<TextualError, Unit> = either {
        Either.catch {
            devicePolicyManager.lockNow()
        }.mapLeft { exception ->
            TextualError.create("DisciplinePolicyManager locking device")
                .addMessage("Android's 'DevicePolicyManager.lockNow' threw an exception")
                .addUnknownAttachment("Exception", exception)
        }.bind()
    }
}