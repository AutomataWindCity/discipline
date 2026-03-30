import { ComponentName, Context, DevicePolicyManager, Settings, UserManager } from "./android.ts";
import { Nullable, TextualError, Tried } from "./discipline.ts";
import { DisciplineDeviceAdminReceiver } from "./discipline_admin_receiver.ts"

export class DisciplinePolicyManager {
  private static readonly TAG = "DisciplinePolicyManager";

  /** This refers to the 'DisciplineDeviceAdminReceiver' class */
  private admin: ComponentName;
  /** We get this from the "on*" methods of "DisciplineDeviceAdminReceiver" */
  private context: Context;
  private devicePolicyManager: DevicePolicyManager;

  private constructor(
    name: ComponentName,
    context: Context,
    devicePolicyManager: DevicePolicyManager,
  ) {
    this.admin = name;
    this.context = context;
    this.devicePolicyManager = devicePolicyManager;
  }

  static create(context: Context): Tried<DisciplinePolicyManager, TextualError> {
    let admin;
    try {
      admin = new ComponentName(context, DisciplineDeviceAdminReceiver.class);
    } catch (exception) {
      const error = TextualError.create("Creating 'DisciplinePolicyManager' from an Android 'Context'");
      TextualError.addMessage(error, "Android threw an exception when creating a 'ComponentName' using 'Context' and 'DisciplineDeviceAdmineReceiver'");
      TextualError.addUnknownAttachment(error, "Exception", exception);
      return Tried.Failure(error);
    }

    let devicePolicyManager;
    try {
      devicePolicyManager = context.getSystemService(Context.DEVICE_POLICY_SERVICE) as DevicePolicyManager;
    } catch (exception) {
      const error = TextualError.create("Creating 'DisciplinePolicyManager' from an Android 'Context'");
      TextualError.addMessage(error, "Android threw an exception when retreiving the 'DevicePolicyManager' service using 'context.getSystemService(Context.DEVICE_POLICY_SERVICE)'");
      TextualError.addUnknownAttachment(error, "Exception", exception);
      return Tried.Failure(error);
    }

    return Tried.Success(new DisciplinePolicyManager(
      admin, 
      context,
      devicePolicyManager,
    ));
  }

  enableFactoryReset(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.clearUserRestriction(
        this.admin,
        UserManager.DISALLOW_FACTORY_RESET,
      );

      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager enabling factory reset")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.clearUserRestriction' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  disableFactoryReset(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.addUserRestriction(
        this.admin, 
        UserManager.DISALLOW_FACTORY_RESET,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager disabling factory reset")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.addUserRestriction' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  enableAppsControl(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.clearUserRestriction(
        this.admin,
        UserManager.DISALLOW_APPS_CONTROL,
      );

      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager enabling app control")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.clearUserRestriction' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  disableAppsControl(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.addUserRestriction(
        this.admin,
        UserManager.DISALLOW_APPS_CONTROL,
      );

      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager disabling app control")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.addUserRestriction' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  enableUsbFileTransfer(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.clearUserRestriction(
        this.admin,
        UserManager.DISALLOW_USB_FILE_TRANSFER,
      );

      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager enabling USB file transfer")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.clearUserRestriction' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }
  
  disableUsbFileTransfer(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.addUserRestriction(
        this.admin,
        UserManager.DISALLOW_USB_FILE_TRANSFER,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager disabling USB file transfer")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.addUserRestriction' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  enableContentCapture(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.clearUserRestriction(
        this.admin,
        UserManager.DISALLOW_CONTENT_CAPTURE,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager enabling content capture")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.clearUserRestriction' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }
  
  disableContentCapture(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.addUserRestriction(
        this.admin,
        UserManager.DISALLOW_CONTENT_CAPTURE,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager disabling content capture")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.addUserRestriction' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }
  
  enableDebuggingFeatures(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.clearUserRestriction(
        this.admin,
        UserManager.DISALLOW_DEBUGGING_FEATURES,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager enabling debugging features")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.clearUserRestriction' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }
  
  disableDebuggingFeatures(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.addUserRestriction(
        this.admin,
        UserManager.DISALLOW_DEBUGGING_FEATURES,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager disabling debugging features")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.addUserRestriction' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  enableApplication(appPackageName: string): Nullable<TextualError> {
    try {
      this.devicePolicyManager.setApplicationHidden(
        this.admin,
        appPackageName,
        false,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager enabling application")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.setApplicationHidden' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  disableApplication(appPackageName: string): Nullable<TextualError> {
    try {
      this.devicePolicyManager.setApplicationHidden(
        this.admin,
        appPackageName,
        true,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager disabling application")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.setApplicationHidden' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  enableApplicationUninstallation(appPackageName: string): Nullable<TextualError> {
    try {
      this.devicePolicyManager.setUninstallBlocked(
        this.admin,
        appPackageName, 
        false,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager enabling application uninstallation")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.setUninstallBlocked' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  disableApplicationUninstallation(appPackageName: string): Nullable<TextualError> {
    try {
      this.devicePolicyManager.setUninstallBlocked(
        this.admin,
        appPackageName, 
        true,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager disabling application uninstallation")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.setUninstallBlocked' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  isApplicationUninstallDisabled(appPackageName: string): Nullable<boolean> {
    try {
      const result = this.devicePolicyManager.isUninstallBlocked(
        this.admin,
        appPackageName,
      );
      return Nullable.Some(result);
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager checking if application uninstall is disabled")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.isUninstallBlocked' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(false); // Default to false on error
    }
  }

  enableInstallApplications(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.clearUserRestriction(
        this.admin,
        UserManager.DISALLOW_INSTALL_APPS,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager enabling install applications")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.clearUserRestriction' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  disableInstallApplications(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.addUserRestriction(
        this.admin,
        UserManager.DISALLOW_INSTALL_APPS,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager disabling install applications")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.addUserRestriction' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  enableSafeBoot(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.clearUserRestriction(
        this.admin,
        UserManager.DISALLOW_SAFE_BOOT,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager enabling safe boot")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.clearUserRestriction' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  disableSafeBoot(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.addUserRestriction(
        this.admin,
        UserManager.DISALLOW_SAFE_BOOT,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager disabling safe boot")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.addUserRestriction' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  turnWiFiOn(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.setSystemSetting(
        this.admin,
        Settings.Global.WIFI_ON,
        "1",
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager turning WiFi on")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.setSystemSetting' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  turnWiFiOff(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.setSystemSetting(
        this.admin,
        Settings.Global.WIFI_ON,
        "0",
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager turning WiFi off")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.setSystemSetting' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  enableDevelopmentSettings(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.setSystemSetting(
        this.admin,
        Settings.Global.DEVELOPMENT_SETTINGS_ENABLED,
        "1",
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager enabling development settings")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.setSystemSetting' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  disableDevelopmentSettings(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.setSystemSetting(
        this.admin,
        Settings.Global.DEVELOPMENT_SETTINGS_ENABLED,
        "0",
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager disabling development settings")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.setSystemSetting' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  enableStatusBar(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.setStatusBarDisabled(
        this.admin,
        false,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager enabling status bar")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.setStatusBarDisabled' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }
  
  disableStatusBar(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.setStatusBarDisabled(
        this.admin,
        true,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager disabling status bar")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.setStatusBarDisabled' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  enableUsbDebugging(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.setSystemSetting(
        this.admin,
        Settings.Global.ADB_ENABLED,
        "1",
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager enabling USB debugging")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.setSystemSetting' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  disableUsbDebugging(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.setSystemSetting(
        this.admin,
        Settings.Global.ADB_ENABLED,
        "0",
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager disabling USB debugging")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.setSystemSetting' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  clearApplicationUserData(
    appPackageName: string,
    executer: java.util.concurrent.Executor,
    callback: (success: boolean) => void,
  ): Nullable<TextualError> {
    try {
      this.devicePolicyManager.clearApplicationUserData(
        this.admin,
        appPackageName,
        executer,
        new android.app.admin.DevicePolicyManager.OnClearApplicationUserDataListener({
          onApplicationUserDataCleared(_, success) {
            callback(success);
          },
        }),
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager clearing application user data")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.clearApplicationUserData' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  enableSystemApplication(appPackageName: string): Nullable<TextualError> {
    try {
      this.devicePolicyManager.enableSystemApp(
        this.admin,
        appPackageName,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager enabling system application")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.enableSystemApp' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  suspendPackages(packageNames: string[]): Nullable<TextualError> {
    try {
      this.devicePolicyManager.setPackagesSuspended(
        this.admin,
        packageNames,
        true,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager suspending packages")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.setPackagesSuspended' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }
  
  unsuspendPackages(packageNames: string[]): Nullable<TextualError> {
    try {
      this.devicePolicyManager.setPackagesSuspended(
        this.admin,
        packageNames,
        false,
      );
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager unsuspending packages")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.setPackagesSuspended' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }

  /**
   * Required permissions: "LOCK_DEVICE".
   */
  lock(): Nullable<TextualError> {
    try {
      this.devicePolicyManager.lockNow();
      return Nullable.None();
    } catch (exception) {
      const it = TextualError.create("DisciplinePolicyManager locking device")
      TextualError.addMessage(it, "Android's 'DevicePolicyManager.lockNow' threw an exception");
      TextualError.addUnknownAttachment(it, "Exception", exception);
      return Nullable.Some(it);
    }
  }
}