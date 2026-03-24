import { ComponentName, Context, DevicePolicyManager, Settings, UserManager } from "./android.ts";
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

  static create(context: Context) {
    return new DisciplinePolicyManager(
      new ComponentName(context, DisciplineDeviceAdminReceiver.class),
      context,
      context.getSystemService(Context.DEVICE_POLICY_SERVICE) as DevicePolicyManager,
    );
  }

  applyInitialPolicies() {
    
  }

  allowFactoryReset() {
    this.devicePolicyManager.clearUserRestriction(
      this.admin,
      UserManager.DISALLOW_FACTORY_RESET,
    );
  }

  disallowFactoryReset() {
    this.devicePolicyManager.addUserRestriction(
      this.admin, 
      UserManager.DISALLOW_FACTORY_RESET,
    );
  }

  allowAppsControl() {
    this.devicePolicyManager.clearUserRestriction(
      this.admin,
      UserManager.DISALLOW_APPS_CONTROL,
    );
  }

  disallowAppsControl() {
    this.devicePolicyManager.addUserRestriction(
      this.admin,
      UserManager.DISALLOW_APPS_CONTROL,
    );
  }

  allowUsbFileTransfer() {
    this.devicePolicyManager.clearUserRestriction(
      this.admin,
      UserManager.DISALLOW_USB_FILE_TRANSFER,
    );
  }
  
  disallowUsbFileTransfer() {
    this.devicePolicyManager.addUserRestriction(
      this.admin,
      UserManager.DISALLOW_USB_FILE_TRANSFER,
    );
  }

  allowContentCapture() {
    this.devicePolicyManager.clearUserRestriction(
      this.admin,
      UserManager.DISALLOW_CONTENT_CAPTURE,
    )
  }
  
  disallowContentCapture() {
    this.devicePolicyManager.addUserRestriction(
      this.admin,
      UserManager.DISALLOW_CONTENT_CAPTURE,
    )
  }
  
  allowDebuggingFeatures() {
    this.devicePolicyManager.clearUserRestriction(
      this.admin,
      UserManager.DISALLOW_DEBUGGING_FEATURES,
    )
  }
  
  disallowDebuggingFeatures() {
    this.devicePolicyManager.addUserRestriction(
      this.admin,
      UserManager.DISALLOW_DEBUGGING_FEATURES,
    )
  }

  allowApplication(appPackageName: string) {
    this.devicePolicyManager.setApplicationHidden(
      this.admin,
      appPackageName,
      false,
    );
  }

  disallowApplication(appPackageName: string) {
    this.devicePolicyManager.setApplicationHidden(
      this.admin,
      appPackageName,
      true,
    );
  }

  allowApplicationUnistallation(appPackageName: string) {
    this.devicePolicyManager.setUninstallBlocked(
      this.admin,
      appPackageName, 
      false,
    );
  }

  disallowApplicationUnistallation(appPackageName: string) {
    this.devicePolicyManager.setUninstallBlocked(
      this.admin,
      appPackageName, 
      true,
    );
  }

  isApplicationUninstallDisallowed(appPackageName: string) {
    return this.devicePolicyManager.isUninstallBlocked(
      this.admin,
      appPackageName,
    );
  }

  allowInstallApplications() {
    this.devicePolicyManager.clearUserRestriction(
      this.admin,
      UserManager.DISALLOW_INSTALL_APPS,
    );
  }

  disallowInstallApplications() {
    this.devicePolicyManager.addUserRestriction(
      this.admin,
      UserManager.DISALLOW_INSTALL_APPS,
    );
  }

  allowSafeBoot() {
    this.devicePolicyManager.clearUserRestriction(
      this.admin,
      UserManager.DISALLOW_SAFE_BOOT,
    );
  }

  disallowSafeBoot() {
    this.devicePolicyManager.addUserRestriction(
      this.admin,
      UserManager.DISALLOW_SAFE_BOOT,
    );
  }

  turnWiFiOn() {
    this.devicePolicyManager.setSystemSetting(
      this.admin,
      Settings.Global.WIFI_ON,
      "1",
    );
  }

  turnWiFiOff() {
    this.devicePolicyManager.setSystemSetting(
      this.admin,
      Settings.Global.WIFI_ON,
      "0",
    );
  }

  enableDevelopmentSettings() {
    this.devicePolicyManager.setSystemSetting(
      this.admin,
      Settings.Global.DEVELOPMENT_SETTINGS_ENABLED,
      "1",
    );
  }

  disableDevelopmentSettings() {
    this.devicePolicyManager.setSystemSetting(
      this.admin,
      Settings.Global.DEVELOPMENT_SETTINGS_ENABLED,
      "0",
    );
  }

  enableStatusBar() {
    this.devicePolicyManager.setStatusBarDisabled(
      this.admin,
      false,
    );
  }
  
  disableStatusBar() {
    this.devicePolicyManager.setStatusBarDisabled(
      this.admin,
      true,
    );
  }

  // isStatusBarEnabled() {
  //   return this.devicePolicyManager.
  // }

  allowUsbDebugging() {
    this.devicePolicyManager.setSystemSetting(
      this.admin,
      Settings.Global.ADB_ENABLED,
      "1",
    );
  }

  disallowUsbDebugging() {
    this.devicePolicyManager.setSystemSetting(
      this.admin,
      Settings.Global.ADB_ENABLED,
      "0",
    );
  }

  clearApplicationUserData(appPackageName: string) {
    // this.devicePolicyManager.clearApplicationUserData(
    //   this.admin,
    //   appPackageName,

    // )
  }

  enableSystemApplication(appPackageName: string) {
    this.devicePolicyManager.enableSystemApp(
      this.admin,
      appPackageName,
    );
  }

  // disablePowerMenu() {
  //   this.devicePolicyManager.addUserRestriction(
  //     this.admin,
  //     UserManager.
  //   )
  // }

  suspendPackages(packageNames: string[]) {
    this.devicePolicyManager.setPackagesSuspended(
      this.admin,
      packageNames,
      true,
    );
  }
  
  unsuspendPackages(packageNames: string[]) {
    this.devicePolicyManager.setPackagesSuspended(
      this.admin,
      packageNames,
      false,
    );
  }
  
  lock() {
    this.devicePolicyManager.lockNow();
  }

  it() {
    this.devicePolicyManager.logoutUser(
      this.admin,
    );
    
    
  }
}