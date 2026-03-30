import { UsageEvent, UsageEvents, UsageStatsManager } from "./android.ts";
import { DateTime, TextualError, Tried } from "./discipline.ts";

export const iterateOverUsageEvents = (
  it: UsageEvents,
  fn: (event: UsageEvent) => void,
): Tried<null, TextualError> => {
  const event = new UsageEvents.Event();
  
  while (it.hasNextEvent()) {
    it.getNextEvent(event);
    fn(event);
  }  

  return Tried.Success(null);
};

export const queryUsageEvents = (
  it: UsageStatsManager,
  from: DateTime,
  till: DateTime,
): Tried<UsageEvents, TextualError> => {
  let usageEvents;

  try {
    usageEvents = it.queryEvents(
      DateTime.toTimestamp(from), 
      DateTime.toTimestamp(till),
    );  
  } catch (error) {
    const it = TextualError.create("");
    return Tried.Failure(it);
  }
// } catch (SecurityException e) {
//     Log.e(TAG, "Permission denied for UsageStatsManager. User may have revoked usage access.", e);
// } catch (Exception e) {
//     Log.e(TAG, "Error querying usage events", e);
// }

  
  if (usageEvents === null) {
    // Log.w(AppMonitoringService.TAG, "No events returned from query");
    return Tried.Failure(TextualError.create(""));
  }

  return Tried.Success(usageEvents);
};