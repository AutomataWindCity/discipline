import { Service, Handler, PackageManager, UsageStatsManager, Intent, UsageEvent, Looper, Log, Context, IBinder } from "./android.ts"
import { iterateOverUsageEvents, queryUsageEvents } from "./android_utilities.ts";
import { Instant, DateTime, Duration, MonotonicClock, TextualError, Tried, getDisciplineInstance, App } from "./discipline.ts"
import { Runnable } from "./java.ts"

@NativeClass()
@JavaProxy("com.discipline.DisciplineMonitoringService")
export class DisciplineMonitoringService extends Service {
  private static readonly TAG = "DisciplineMonitoringService";
  private static readonly CHANNEL_ID = "detox_monitoring_channel";
  private static readonly NOTIFICATION_ID = 1001;
  
  private state: null | {
    discipline: App,
    handler: Handler,
    runnable: Runnable,
    packageManager: PackageManager,
    usageStatsManager: UsageStatsManager,
    didCreateNotificationChannel: boolean,
  } = null;

  override onCreate(): void {
    super.onCreate();
    Log.d(DisciplineMonitoringService.TAG, "Service created");
  }

  override onStartCommand(intent: Intent, flags: number, startId: number): number {
    Log.d(DisciplineMonitoringService.TAG, "Service started");
    
    const discipline = getDisciplineInstance();
    if (Tried.isFailure(discipline)) {
      return Service.START_STICKY;
    }

    const handler = new Handler(Looper.getMainLooper());
    // const runnable = Runnable.create(() => {
    //   this.synhcronize();
    //   handler.postDelayed(runnable, DisciplineMonitoringService.POLL_INTERVAL_MS);
    // });

    const packageManager: PackageManager = this.getPackageManager();
    if (packageManager === null) {
      Log.e(DisciplineMonitoringService.TAG, "PackageManager is null");
      return;
    }

    const usageStatsManager: UsageStatsManager = this.getSystemService(Context.USAGE_STATS_SERVICE);
    if (usageStatsManager === null) {
      Log.e(DisciplineMonitoringService.TAG, "UsageStatsManager is null");
      return Service.START_STICKY;
    }

    this.state = {
      handler,
      runnable,
      lastEventTimestamp: 0,
      usageStatsManager,
      packageManager,
      didCreateNotificationChannel: false,

    };
    // // Create notification channel for foreground service
    // this.createNotificationChannel();
    
    // // Start as foreground service
    // const notification = this.createNotification();
    // this.startForeground(AppMonitoringService.NOTIFICATION_ID, notification);
    
    // Start polling
    if (this.state === null) {
      Log.e(DisciplineMonitoringService.TAG, "In 'onStartCommand': Field 'state' is not initialized: Method 'onCreate' is responsible for initializing it");
      return Service.START_STICKY; // Restart if system kills the service
    }

    this.state.handler.post(this.state.runnable);
    return Service.START_STICKY;
  }

  logErr(error: TextualError) {
    // TODO
    // logError(DisciplineMonitoringService.TAG, TextualError.prettyPrint(error));
  }
  
  /**
   * Query the UsageStatsManager for events since the last poll
   */
  private synhcronize() {
    let it;

    if (this.state === null) {
      it = TextualError.create("AppMonitoringService");
      TextualError.addMessage(it, "Field 'state' is not initialized. This is due to this method being called before 'onCreate'")
      return Tried.Failure(it);
    }

    const nowAsInstant = MonotonicClock.getNow(
      this.state.app.state.monotonicClock,
    );

    const interval = Instant.tillOrZero(
      this.state.previousSynchronizationInstant,
      nowAsInstant,
    );

    // Detect 'Handler' error. This is virtually impossible becuase it means
    // there is an error in the Android OS.
    if (Duration.isShorterThan(interval, this.state.synchronizationInterval)) {
      it = TextualError.create("AppMonitoringService");
      TextualError.addMessage(it, "Time elapsed since 'prevoiusSynchronizationTimeAsInstant' is less than 'synchronizationInterval'")
      return Tried.Failure(it);
    }

    const nowAsDateTime = DateTime.now();
    if (DateTime.isEarilerThan(nowAsDateTime, this.state.previousSynchronizationDateTime)) {
      it = TextualError.create("AppMonitoringService");
      TextualError.addMessage(it, "System clock jumped backwards")
      return Tried.Failure(it);
    }

    it = queryUsageEvents(
      this.state.usageStatsManager,
      this.state.previousSynchronizationDateTime,
      nowAsDateTime,
    );
    if (Tried.isFailure(it)) {
      return;
    }

    const state = this.state;
    const usageEvents = Tried.value(it);

    it = iterateOverUsageEvents(
      usageEvents,
      event => {
        let it = DateTime.fromTimestamp(event.getTimeStamp());
        if (Tried.isFailure(it)) {
          return;
        }

        const time = Tried.value(it);     
        const packageName = event.getPackageName() as string | null;

        switch (event.getEventType()) {
          case UsageEvent.ACTIVITY_PAUSED: {
            // App.onActivityResumed();
            return;
          }
          case UsageEvent.ACTIVITY_RESUMED: {
            return;
          }
          case UsageEvent.ACTIVITY_STOPPED: {
            return;
          }
          case UsageEvent.DEVICE_STARTUP: {
            return;
          }
          case UsageEvent.DEVICE_SHUTDOWN: {
            return;
          }
          case UsageEvent.KEYGUARD_HIDDEN: {
            return;
          }
          case UsageEvent.KEYGUARD_SHOWN: {
            return;
          }
          default: {
            // We don't care about other event types, for now [Smug Face Emoji].
            return;
          }
        }
      },
    );

    if (Tried.isFailure(it)) {
      it = Tried.error(it);
      TextualError.changeContext(it, "");
      return Tried.Failure(it);
    }

    return Tried.Success(null);
  }

  // private createNotificationChannel(): void {
  //   if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
  //     const channel = new NotificationChannel(
  //       DisciplineMonitoringService.CHANNEL_ID,
  //       "Digital Detox Monitoring",
  //       NotificationManager.IMPORTANCE_LOW
  //     );

  //     channel.setDescription("Shows that digital detox is actively monitoring app usage");
      
  //     const notificationManager: NotificationManager = this.getSystemService(Service.NOTIFICATION_SERVICE);
  //     if (notificationManager != null) {
  //       notificationManager.createNotificationChannel(channel);
  //     }
  //   }
  // }

  /**
   * Create notification for foreground service
   */
  // private createNotification(): Notification {
  //   const builder = new NotificationCompat.Builder(this, DisciplineMonitoringService.CHANNEL_ID)
  //     .setContentTitle("Digital Detox Active")
  //     .setContentText("Monitoring app usage to help you stay focused")
  //     .setSmallIcon(android.R.drawable.ic_menu_info_details)
  //     .setPriority(NotificationCompat.PRIORITY_LOW)
  //     .setOngoing(true);

  //   return builder.build();
  // }

  override onDestroy(): void {
    super.onDestroy();
    Log.d(DisciplineMonitoringService.TAG, "Service destroyed");
    
    if (this.state !== null) {
      this.state.handler.removeCallbacks(this.state.runnable);
      return;
    }
  }

  // We are required to implement this even if we don't use it.
  //
  // Implementors that don't use this method are allowed to return 'null'.
  override onBind(_intent: Intent): IBinder {
    // Not using binding
    return null as unknown as IBinder;
  }
}
