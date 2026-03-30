import { App } from "../discipline.ts"

let app: null | App;



onmessage = (event) => {

};

onerror = () => {

};


export const synchronize = () => {
  let it;
  
  let me;
  me = get();
  if (Tried.isFailure(me)) {
    return;
  } else {
    me = Tried.value(me);
  }

  const { 
    androidMonitoringServiceState: {
      appUsageStatsQueryInterval,
      previousAppUsageStatsQueryDateTime,
      previousAppUsageStatsQueryInstant,
    },
    monotonicClock,
  } = me.discipline.state;

  // if (this.state === null) {
  //   it = TextualError.create("AppMonitoringService");
  //   TextualError.addMessage(it, "Field 'state' is not initialized. This is due to this method being called before 'onCreate'")
  //   return Tried.Failure(it);
  // }

  const nowAsInstant = MonotonicClock.getNow(monotonicClock);

  const interval = Instant.tillOrZero(
    previousAppUsageStatsQueryInstant,
    nowAsInstant,
  );

  // Detect 'Handler' error. This is virtually impossible becuase it means
  // there is an error in the Android OS.
  if (Duration.isShorterThan(interval, appUsageStatsQueryInterval)) {
    it = TextualError.create("AppMonitoringService");
    TextualError.addMessage(it, "Time elapsed since 'prevoiusSynchronizationTimeAsInstant' is less than 'synchronizationInterval'")
    return Tried.Failure(it);
  }

  const nowAsDateTime = DateTime.now();
  if (DateTime.isEarilerThan(nowAsDateTime, previousAppUsageStatsQueryDateTime)) {
    it = TextualError.create("AppMonitoringService");
    TextualError.addMessage(it, "System clock jumped backwards")
    return Tried.Failure(it);
  }

  it = queryUsageEvents(
    me.usageStatsManager,
    previousAppUsageStatsQueryDateTime,
    nowAsDateTime,
  );
  if (Tried.isFailure(it)) {
    return;
  }

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
          // BackgroundThread.onActivityResumed();
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
};