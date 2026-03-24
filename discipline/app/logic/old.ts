
@NativeClass()
@JavaProxy("automata-wind-city.auto-nytro.discipline.ScreenTimeService")
class ScreenTimeService extends Service {
  private static readonly TAG = "ScreenTimeService";
  private static readonly CHANNEL_ID = "screen_time_channel";
  private static readonly NOTIFICATION_ID = 1001;

  private timer: Timer | null;
  private policyManager: PolicyManager;
  private appUsageMap: Map<string, number>;
  private currentForegroundApp: string | null = null;
  private currentAppStartTime: number = 0;

  public override onCreate() {
    super.onCreate();

    const policyManager = PolicyManager.fromContext(this);
    this.ourStartForegroundService();
    this.startMonitoring();

    Log.d(TAG, "ScreenTimeService created");
  }

  private ourStartForegroundService() {
    this.createNotificationChannel();

    const notification = new NotificationCompat.Builder(this, ScreenTimeService.CHANNEL_ID)
      .setContentTitle("Screen Time Monitor")
      .setContentText("Monitoring app usage and enforcing limits")
      .setSmallIcon(android.R.drawable.ic_dialog_info)
      .setPriority(NotificationCompat.PRIORITY_LOW)
      .build();

    return this.startForeground(ScreenTimeService.NOTIFICATION_ID, notification);
  }

  private createNotificationChannel() {
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
      const channel = new NotificationChannel(
        ScreenTimeService.CHANNEL_ID,
        "Screen Time Service",
        NotificationManager.IMPORTANCE_LOW
      );

      const manager: NotificationManager = this.getSystemService(NotificationManager.class);
      manager.createNotificationChannel(channel);
    }
  }

  private startMonitoring() {
    const me = this;

    @NativeClass()
    @JavaProxy("automata-wind-city.auto-nytro.discipline.MyTimerTask")
    class MyTimerTask extends TimerTask {
      public run() {
        me.monitorForegroundApp();
      }
    }

    this.timer = new Timer(true);
    // Check every 5 seconds
    this.timer.scheduleAtFixedRate(new MyTimerTask(), 0, 5000);
  }

  private monitorForegroundApp() {
    const am: ActivityManager = this.getSystemService(Context.ACTIVITY_SERVICE);

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP) {
      const appProcesses = am.getRunningAppProcesses();

      if (appProcesses != null) {
        const iterator = appProcesses.iterator();
        while (iterator.hasNext()) {
          const processInfo: RunningAppProcessInfo = iterator.next();

          if (processInfo.importance == ActivityManager.RunningAppProcessInfo.IMPORTANCE_FOREGROUND) {
            const packages = processInfo.pkgList;
            if (packages.length > 0) {
              const foregroundPackage = packages[0];

              // Check if foreground app changed
              if (
                this.currentForegroundApp === null
                ||
                this.currentForegroundApp !== foregroundPackage
              ) {
                this.recordPreviousAppUsage();

                // Start tracking new app
                this.currentForegroundApp = foregroundPackage;
                this.currentAppStartTime = Date.now();
                // this.currentAppStartTime = System.currentTimeMillis();

                // Check if this app should be blocked
                if (this.policyManager.isAppBlocked(foregroundPackage)) {
                  // Force stop blocked app
                  new Handler(Looper.getMainLooper()).post(() => {
                    this.policyManager.forceStopApp(foregroundPackage);
                  });
                }

                Log.d(TAG, "Foreground app changed to: " + foregroundPackage);
              }
            }
            break;
          }
        }
      }
    }
  }

  private recordPreviousAppUsage() {
    if (
      this.currentForegroundApp !== null
      &&
      this.currentAppStartTime > 0
    ) {
      // const duration = System.currentTimeMillis() - this.currentAppStartTime;
      const duration = Date.now() - this.currentAppStartTime;
      this.policyManager.recordAppUsage(this.currentForegroundApp, duration);
      Log.d(TAG, "Recorded " + duration + "ms for app: " + currentForegroundApp);
    }
  }

  public override onStartCommand(intent: Intent, flags: number, startId: number): number {
    return ScreenTimeService.START_STICKY;
  }

  public override onDestroy() {
    super.onDestroy();

    if (this.timer != null) {
      this.timer.cancel();
      this.timer = null;
    }

    this.recordPreviousAppUsage();

    Log.d(TAG, "ScreenTimeService destroyed");
  }

  public override onBind(intent: Intent): IBinder | null {
    return null;
  }
}

@NativeClass()
@JavaProxy("automata-wind-city.auto-nytro.discipline.AppBlockerService")
class AppBlockerService extends AccessibilityService {
  private policyManager: PolicyManager;

  public override onCreate(): void {
    super.onCreate();
    this.policyManager = PolicyManager.fromContext(this);
  }

  public override onAccessibilityEvent(event: AccessibilityEvent): void {
    if (event.getEventType() == AccessibilityEvent.TYPE_WINDOW_STATE_CHANGED) {
      const packageName = event.getPackageName() != null 
        ? event.getPackageName().toString() 
        : "";

      // Check if app should be blocked
      if (this.policyManager.isAppBlocked(packageName)) {
        // Block the app by going back to home screen
       this. performGlobalAction(AppBlockerService.GLOBAL_ACTION_HOME);
      }

      // Check screen time limit
      const remainingTime = this.policyManager.getRemainingScreenTime(packageName);
      if (remainingTime <= 0) {
        this.performGlobalAction(AppBlockerService.GLOBAL_ACTION_HOME);
      }
    }
  }

  public override onInterrupt(): void {
    // Service interrupted
  }

  protected override onServiceConnected(): void {
    super.onServiceConnected();
      
    const info = new AccessibilityServiceInfo();
    info.eventTypes = AccessibilityEvent.TYPE_WINDOW_STATE_CHANGED;
    info.feedbackType = AccessibilityServiceInfo.FEEDBACK_GENERIC;
    info.notificationTimeout = 100;
    this.setServiceInfo(info);
  }
}

class PolicyManager {
  static readonly TAG = "PolicyManager";

  private static readonly BLOCKED_APPS = [
    "com.facebook.katana",      // Facebook
    "com.instagram.android",    // Instagram
    "com.twitter.android",      // Twitter
    "com.snapchat.android",     // Snapchat
    "com.tiktok.android",       // TikTok
    "com.whatsapp",             // WhatsApp
    "com.youtube.android",      // YouTube
    "com.netflix.mediaclient",  // Netflix
    "com.amazon.avod",          // Amazon Prime
    "com.spotify.music"         // Spotify
  ];

  // Daily screen time limits in minutes (per app)
  private static readonly DEFAULT_DAILY_LIMIT_MINUTES = 120; // 2 hours
  private static readonly WARNING_THRESHOLD_MINUTES = 15;     // Warning when 15 mins left

  private context: Context;
  private dpm: DevicePolicyManager;
  private adminComponent: ComponentName;

  private constructor(
    context: Context | Service,
    dpm: DevicePolicyManager,
    adminComponent: ComponentName,
  ) {
    this.context = context;
    this.dpm = dpm;
    this.adminComponent = adminComponent;
  }

  static fromContext(context: Context) {
    return new PolicyManager(
      context,
      context.getSystemService(Context.DEVICE_POLICY_SERVICE) as DevicePolicyManager,
      new ComponentName(context, DeviceOwnerReceiverImpl.class)
    );
  }

  applyInitialPolicies() {
    if (!this.isDeviceOwner()) {
      // Log.e(TAG, "Not a device owner, cannot apply policies");
      return;
    }

    try {
      // Lock screen policies
      this.setLockScreenPolicies();

      // Security policies
      this.setSecurityPolicies();

      // User restrictions
      this.setUserRestrictions();

      // App management
      this.blockApps();

      // System settings
      this.setSystemSettings();

      Log.d(TAG, "All initial policies applied successfully");

    } catch (error) {
      // Log.e(TAG, "Failed to apply policies: " + e.getMessage());
    }
  }

  setLockScreenPolicies() {
    // Set password quality
    this.dpm.setPasswordQuality(this.adminComponent, DevicePolicyManager.PASSWORD_QUALITY_COMPLEX);

    // Minimum password length
    this.dpm.setPasswordMinimumLength(this.adminComponent, 6);

    // Password expiration (90 days)
    this.dpm.setPasswordExpirationTimeout(this.adminComponent, 90 * 24 * 60 * 60 * 1000L);

    // Password history (prevent last 3 passwords)
    this.dpm.setPasswordHistoryLength(this.adminComponent, 3);

    // Maximum failed attempts before wipe (optional)
    this.dpm.setMaximumFailedPasswordsForWipe(this.adminComponent, 10);

    // Lock screen timeout (2 minutes)
    this.dpm.setMaximumTimeToLock(this.adminComponent, 2 * 60 * 1000);

    Log.d(TAG, "Lock screen policies applied");
  }

  private setSecurityPolicies() {
    // Require storage encryption
    if (this.dpm.getStorageEncryptionStatus() != DevicePolicyManager.ENCRYPTION_STATUS_ACTIVATING) {
      this.dpm.setStorageEncryption(this.adminComponent, true);
    }

    // Disable camera
    this.dpm.setCameraDisabled(this.adminComponent, true);

    Log.d(TAG, "Security policies applied");
  }

  private setUserRestrictions() {
    // Disable factory reset
    this.dpm.addUserRestriction(this.adminComponent, UserManager.DISALLOW_FACTORY_RESET);

    // Disable uninstalling apps
    this.dpm.addUserRestriction(this.adminComponent, UserManager.DISALLOW_APPS_CONTROL);

    // Disable USB file transfer
    this.dpm.addUserRestriction(this.adminComponent, UserManager.DISALLOW_USB_FILE_TRANSFER);

    // Disable screen capture
    this.dpm.addUserRestriction(this.adminComponent, UserManager.DISALLOW_SCREEN_CAPTURE);

    // Disable sharing location
    this.dpm.addUserRestriction(this.adminComponent, UserManager.DISALLOW_SHARE_LOCATION);

    // Disable modifying accounts
    this.dpm.addUserRestriction(this.adminComponent, UserManager.DISALLOW_MODIFY_ACCOUNTS);

    // Disable installing unknown apps
    this.dpm.addUserRestriction(this.adminComponent, UserManager.DISALLOW_INSTALL_UNKNOWN_SOURCES);

    // Disable debugging features
    this.dpm.addUserRestriction(this.adminComponent, UserManager.DISALLOW_DEBUGGING_FEATURES);

    Log.d(TAG, "User restrictions applied");
  }

  blockApps() {
    if (!this.isDeviceOwner()) {
      // Log.e(TAG, "Not device owner, cannot block apps");
      return;
    }

    const pm = this.context.getPackageManager();

    for (const packageName of PolicyManager.BLOCKED_APPS) {
      try {
        // Check if app is installed
        pm.getPackageInfo(packageName, 0);

        // Disable the app
        this.dpm.setApplicationHidden(this.adminComponent, packageName, true);
        Log.d(TAG, "Blocked app: " + packageName);

        // } catch (PackageManager.NameNotFoundException e) {
      } catch (error) {
        Log.d(TAG, "App not installed: " + packageName);
      }
    }
  }

  public unblockApp(packageName: string) {
    if (!this.isDeviceOwner()) {
      // Log.e(TAG, "Not device owner, cannot unblock app");
      return;
    }

    this.dpm.setApplicationHidden(this.adminComponent, packageName, false);
    Log.d(TAG, "Unblocked app: " + packageName);
  }

  public isAppBlocked(packageName: string): boolean {
    return this.dpm.isApplicationHidden(this.adminComponent, packageName);
  }

  public getBlockedAppsList() {
    return PolicyManager.BLOCKED_APPS;
  }

  private setSystemSettings() {
    // Disable status bar expansion
    this.dpm.addUserRestriction(this.adminComponent, UserManager.DISALLOW_STATUS_BAR);

    // Disable keyguard features (trust agents, fingerprint)
    this.dpm.setKeyguardDisabledFeatures(this.adminComponent,
      DevicePolicyManager.KEYGUARD_DISABLE_FINGERPRINT |
      DevicePolicyManager.KEYGUARD_DISABLE_TRUST_AGENTS |
      DevicePolicyManager.KEYGUARD_DISABLE_FACE |
      DevicePolicyManager.KEYGUARD_DISABLE_IRIS
    );

    Log.d(TAG, "System settings applied");
  }

  public setScreenTimeLimit(packageName: string, minutes: number) {
    // This would typically store limits in SharedPreferences
    // The actual enforcement happens in ScreenTimeService
    this
      .context
      .getSharedPreferences("screen_time", Context.MODE_PRIVATE)
      .edit()
      .putLong(packageName + "_limit", minutes * 60 * 1000)
      .apply();
  }

  getRemainingScreenTime(packageName: string): number {
    const limit = this
      .context
      .getSharedPreferences(
        "screen_time",
        Context.MODE_PRIVATE,
      )
      .getLong(
        packageName + "_limit",
        PolicyManager.DEFAULT_DAILY_LIMIT_MINUTES * 60 * 1000,
      );

    const used = this
      .context
      .getSharedPreferences(
        "screen_time",
        Context.MODE_PRIVATE,
      )
      .getLong(
        packageName + "_used_" + this.getCurrentDate(),
        0,
      );

    return Math.max(0, limit - used);
  }

  private getCurrentDate(): string {
    return "not implemented"
    // return String.valueOf(System.currentTimeMillis() / (24 * 60 * 60 * 1000));
  }

  public recordAppUsage(packageName: string, durationMs: number) {
    const key = packageName + "_used_" + this.getCurrentDate();
    const currentUsed = this
      .context
      .getSharedPreferences("screen_time", Context.MODE_PRIVATE)
      .getLong(key, 0);

    this
      .context
      .getSharedPreferences("screen_time", Context.MODE_PRIVATE)
      .edit()
      .putLong(key, currentUsed + durationMs)
      .apply();

    // Check if limit exceeded
    const limit = this
      .context
      .getSharedPreferences(
        "screen_time",
        Context.MODE_PRIVATE,
      )
      .getLong(
        packageName + "_limit",
        PolicyManager.DEFAULT_DAILY_LIMIT_MINUTES * 60 * 1000,
      );

    if (currentUsed + durationMs >= limit - (PolicyManager.WARNING_THRESHOLD_MINUTES * 60 * 1000)) {
      // Send warning notification
      this.sendScreenTimeWarning(
        packageName,
        (limit - (currentUsed + durationMs)) / (60 * 1000),
      );
    }

    if (currentUsed + durationMs >= limit) {
      // Force stop the app if limit exceeded
      this.forceStopApp(packageName);
    }
  }

  private sendScreenTimeWarning(packageName: string, minutesLeft: number) {
    const nm: NotificationManager = this
      .context
      .getSystemService(Context.NOTIFICATION_SERVICE);

    // Build and show notification
    const builder = new android.app.Notification.Builder(this.context)
      .setContentTitle("Screen Time Limit")
      .setContentText(minutesLeft + " minutes remaining for " + packageName)
      .setSmallIcon(android.R.drawable.ic_dialog_info);

    if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.O) {
      builder.setChannelId("screen_time");
    }

    nm.notify(packageName.hashCode(), builder.build());
  }

  private forceStopApp(packageName: string) {
    try {
      const am: ActivityManager = this
        .context
        .getSystemService(Context.ACTIVITY_SERVICE);

      am.
      // am.forceStopPackage(packageName);

      Log.d(TAG, "Force stopped app: " + packageName + " due to screen time limit");
    } catch (e) {
      // } catch (SecurityException e) {
      // Log.e(TAG, "Failed to force stop app: " + e.getMessage());
    }
  }

  public setKioskMode(enabled: boolean, allowedPackages: string[]) {
    if (enabled) {
      // Lock to specific apps
      this.dpm.setLockTaskPackages(this.adminComponent, allowedPackages);

      // Start lock task mode
      if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.M) {
        // Requires the app to call startLockTask() from an activity
      }
    } else {
      this.dpm.setLockTaskPackages(this.adminComponent, []);
    }
  }

  public wipeDevice() {
    this.dpm.wipeData(DevicePolicyManager.WIPE_EXTERNAL_STORAGE);
  }

  public lockDevice() {
    this.dpm.lockNow();
  }

  private isDeviceOwner(): boolean {
    return this.dpm.isDeviceOwnerApp(this.context.getPackageName());
  }
}


@NativeClass()
@JavaProxy("automata-wind-city.auto-nytro.discipline.DeviceOwnerReceiver")
class DeviceOwnerReceiverImpl extends DeviceAdminReceiver {
  static TAG = "DeviceOwnerReceiver";

  override onEnabled(context: Context, intent: Intent) {
    super.onEnabled(context, intent);

    Log.d(DeviceOwnerReceiverImpl.TAG, "Device Admin enabled");
    Toast.makeText(context, "Device Admin Enabled", Toast.LENGTH_SHORT).show();

    // Start management services
    this.startManagementServices(context);
  }

  override onDisabled(context: Context, intent: Intent) {
    super.onDisabled(context, intent);
    Log.d(TAG, "Device Admin disabled");
    Toast.makeText(context, "Device Admin Disabled", Toast.LENGTH_SHORT).show();

    // Stop services
    this.stopManagementServices(context);
  }

  onProfileProvisioningComplete(context: Context, intent: Intent) {
    super.onProfileProvisioningComplete(context, intent);
    Log.d(TAG, "Profile provisioning complete");

    // Set up policies after provisioning
    const policyManager = PolicyManager.fromContext(context);
    policyManager.applyInitialPolicies();

    this.startManagementServices(context);
  }

  startManagementServices(context: Context) {
    // Start screen time monitoring service
    const screenTimeIntent = new Intent(context, ScreenTimeService.class);
    context.startForegroundService(screenTimeIntent);

    // Start app blocking service
    const appBlockerIntent = new Intent(context, AppBlockerService.class);
    context.startService(appBlockerIntent);
  }

  stopManagementServices(context: Context) {
    context.stopService(new Intent(context, ScreenTimeService.class));
    context.stopService(new Intent(context, AppBlockerService.class));
  }
}