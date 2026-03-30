// import { ComponentName, Context, DevicePolicyManager, Settings, UserManager } from "./android.ts";
// import { Nullable, TextualError } from "./discipline.ts";
// import { DisciplineDeviceAdminReceiver } from "./discipline_admin_receiver.ts"

// export class DisciplinePolicyManager {
//   private static readonly TAG = "DisciplinePolicyManager";

//   /** This refers to the 'DisciplineDeviceAdminReceiver' class */
//   private admin: ComponentName;
//   /** We get this from the "on*" methods of "DisciplineDeviceAdminReceiver" */
//   private context: Context;
//   private devicePolicyManager: DevicePolicyManager;

//   private constructor(
//     name: ComponentName,
//     context: Context,
//     devicePolicyManager: DevicePolicyManager,
//   ) {
//     this.admin = name;
//     this.context = context;
//     this.devicePolicyManager = devicePolicyManager;
//   }

//   static create(context: Context) {
//     return new DisciplinePolicyManager(
//       new ComponentName(context, DisciplineDeviceAdminReceiver.class),
//       context,
//       context.getSystemService(Context.DEVICE_POLICY_SERVICE) as DevicePolicyManager,
//     );
//   }

//   enableFactoryReset() {
//     try {
//       this.devicePolicyManager.clearUserRestriction(
//         this.admin,
//         UserManager.DISALLOW_FACTORY_RESET,
//       );

//       return Nullable.None();
//     } catch (exception) {
//       const it = TextualError.create("DisciplinePolicyManager enabling factory reset")
//       TextualError.addMessage(it, "Android's 'DevicePolicyManager.clearUserRestriction' threw an exception");
//       TextualError.addUnknownAttachment(it, "Exception", exception);
//       return Nullable.Some(it);
//     }
//   }

//   disableFactoryReset(): Nullable<TextualError> {
//     try {
//       this.devicePolicyManager.addUserRestriction(
//         this.admin, 
//         UserManager.DISALLOW_FACTORY_RESET,
//       );
//     } catch (exception) {
//       const it = TextualError.create("DisciplinePolicyManager disabling factory reset")
//       TextualError.addMessage(it, "Android's 'DevicePolicyManager.addUserRestriction' threw an exception");
//       TextualError.addUnknownAttachment(it, "Exception", exception);
//       return Nullable.Some(it);
//     }

//     return Nullable.None();
//   }

//   enableAppsControl() {
//     try {
//       this.devicePolicyManager.clearUserRestriction(
//         this.admin,
//         UserManager.DISALLOW_APPS_CONTROL,
//       );

//       return Nullable.None();
//     } catch (exception) {
//       const it = TextualError.create("DisciplinePolicyManager enabling app control")
//       TextualError.addMessage(it, "Android's 'DevicePolicyManager.clearUserRestriction' threw an exception");
//       TextualError.addUnknownAttachment(it, "Exception", exception);
//       return Nullable.Some(it);
//     }
//   }

//   disableAppsControl() {
//     try {
//       this.devicePolicyManager.addUserRestriction(
//         this.admin,
//         UserManager.DISALLOW_APPS_CONTROL,
//       );

//       return Nullable.None();
//     } catch (exception) {
//       const it = TextualError.create("DisciplinePolicyManager disabling app control")
//       TextualError.addMessage(it, "Android's 'DevicePolicyManager.addUserRestriction' threw an exception");
//       TextualError.addUnknownAttachment(it, "Exception", exception);
//       return Nullable.Some(it);
//     }
//   }

//   enableUsbFileTransfer() {
//     try {
//       this.devicePolicyManager.clearUserRestriction(
//         this.admin,
//         UserManager.DISALLOW_USB_FILE_TRANSFER,
//       );

//       return Nullable.None();
//     } catch (exception) {
//       const it = TextualError.create("DisciplinePolicyManager enabling USB file transfer")
//       TextualError.addMessage(it, "Android's 'DevicePolicyManager.clearUserRestriction' threw an exception");
//       TextualError.addUnknownAttachment(it, "Exception", exception);
//       return Nullable.Some(it);
//     }
//   }
  
//   disableUsbFileTransfer() {
//     this.devicePolicyManager.addUserRestriction(
//       this.admin,
//       UserManager.DISALLOW_USB_FILE_TRANSFER,
//     );
//   }

//   enableContentCapture() {
//     this.devicePolicyManager.clearUserRestriction(
//       this.admin,
//       UserManager.DISALLOW_CONTENT_CAPTURE,
//     )
//   }
  
//   disableContentCapture() {
//     this.devicePolicyManager.addUserRestriction(
//       this.admin,
//       UserManager.DISALLOW_CONTENT_CAPTURE,
//     )
//   }
  
//   enableDebuggingFeatures() {
//     this.devicePolicyManager.clearUserRestriction(
//       this.admin,
//       UserManager.DISALLOW_DEBUGGING_FEATURES,
//     )
//   }
  
//   disableDebuggingFeatures() {
//     this.devicePolicyManager.addUserRestriction(
//       this.admin,
//       UserManager.DISALLOW_DEBUGGING_FEATURES,
//     )
//   }

//   enableApplication(appPackageName: string) {
//     this.devicePolicyManager.setApplicationHidden(
//       this.admin,
//       appPackageName,
//       false,
//     );
//   }

//   disableApplication(appPackageName: string) {
//     this.devicePolicyManager.setApplicationHidden(
//       this.admin,
//       appPackageName,
//       true,
//     );
//   }

//   enableApplicationUnistallation(appPackageName: string) {
//     this.devicePolicyManager.setUninstallBlocked(
//       this.admin,
//       appPackageName, 
//       false,
//     );
//   }

//   disableApplicationUnistallation(appPackageName: string) {
//     this.devicePolicyManager.setUninstallBlocked(
//       this.admin,
//       appPackageName, 
//       true,
//     );
//   }

//   isApplicationUninstallDisenableed(appPackageName: string) {
//     return this.devicePolicyManager.isUninstallBlocked(
//       this.admin,
//       appPackageName,
//     );
//   }

//   enableInstallApplications() {
//     this.devicePolicyManager.clearUserRestriction(
//       this.admin,
//       UserManager.DISALLOW_INSTALL_APPS,
//     );
//   }

//   disableInstallApplications() {
//     this.devicePolicyManager.addUserRestriction(
//       this.admin,
//       UserManager.DISALLOW_INSTALL_APPS,
//     );
//   }

//   enableSafeBoot() {
//     this.devicePolicyManager.clearUserRestriction(
//       this.admin,
//       UserManager.DISALLOW_SAFE_BOOT,
//     );
//   }

//   disableSafeBoot() {
//     this.devicePolicyManager.addUserRestriction(
//       this.admin,
//       UserManager.DISALLOW_SAFE_BOOT,
//     );
//   }

//   turnWiFiOn() {
//     this.devicePolicyManager.setSystemSetting(
//       this.admin,
//       Settings.Global.WIFI_ON,
//       "1",
//     );
//   }

//   turnWiFiOff() {
//     this.devicePolicyManager.setSystemSetting(
//       this.admin,
//       Settings.Global.WIFI_ON,
//       "0",
//     );
//   }

//   enableDevelopmentSettings() {
//     this.devicePolicyManager.setSystemSetting(
//       this.admin,
//       Settings.Global.DEVELOPMENT_SETTINGS_ENABLED,
//       "1",
//     );
//   }

//   disableDevelopmentSettings() {
//     this.devicePolicyManager.setSystemSetting(
//       this.admin,
//       Settings.Global.DEVELOPMENT_SETTINGS_ENABLED,
//       "0",
//     );
//   }

//   enableStatusBar() {
//     this.devicePolicyManager.setStatusBarDisabled(
//       this.admin,
//       false,
//     );
//   }
  
//   disableStatusBar() {
//     this.devicePolicyManager.setStatusBarDisabled(
//       this.admin,
//       true,
//     );
//   }

//   // isStatusBarEnabled() {
//   //   return this.devicePolicyManager.
//   // }

//   enableUsbDebugging() {
//     this.devicePolicyManager.setSystemSetting(
//       this.admin,
//       Settings.Global.ADB_ENABLED,
//       "1",
//     );
//   }

//   disableUsbDebugging() {
//     this.devicePolicyManager.setSystemSetting(
//       this.admin,
//       Settings.Global.ADB_ENABLED,
//       "0",
//     );
//   }

//   clearApplicationUserData(appPackageName: string) {
//     this.devicePolicyManager.clearApplicationUserData(
//       this.admin,
//       appPackageName,

//     )
//   }

//   enableSystemApplication(appPackageName: string) {
//     this.devicePolicyManager.enableSystemApp(
//       this.admin,
//       appPackageName,
//     );
//   }

//   // disablePowerMenu() {
//   //   this.devicePolicyManager.addUserRestriction(
//   //     this.admin,
//   //     UserManager.
//   //   )
//   // }

//   suspendPackages(packageNames: string[]) {
//     this.devicePolicyManager.setPackagesSuspended(
//       this.admin,
//       packageNames,
//       true,
//     );
//   }
  
//   unsuspendPackages(packageNames: string[]) {
//     this.devicePolicyManager.setPackagesSuspended(
//       this.admin,
//       packageNames,
//       false,
//     );
//   }

//   /**
//    * Required permissions: "LOCK_DEVIVE".
//    */
//   lock() {
//     try {
//       this.devicePolicyManager.lockNow();
//     } catch (exception) {
//       return 
//     }
//   }
// }

