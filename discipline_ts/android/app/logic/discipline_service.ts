import { Intent, IBinder, DeviceAdminService, Service, Context, Notification, NotificationCompat, Build, NotificationManager, NotificationChannel, Log, PackageManager, UsageStatsManager } from "./android.ts"
import { Nullable, TextualError, Tried } from "./discipline.ts";

@NativeClass()
@JavaProxy("awc.autonytro.discipline.Service")
export class DisciplineService extends DeviceAdminService {
  override onCreate(): void {
    onCreate(this);
  }

  override onStartCommand(intent: Intent, flags: number, startId: number): number {
    onStartCommand(this);
    return Service.START_STICKY;
  }

  override onDestroy(): void {
    onDestroy(this);
  }

  // This is an abstract method that we must implement even
  // though it's not applicable to our use case. Android
  // allows us to return null.
  override onBind(intent: Intent): IBinder {
    return null as unknown as IBinder;
  }
}

const LOG_TAG = "DisciplineService";
const NOTIFICATION_ID = 1;
const NOTIFICATION_CHANNEL_ID = "main-channel";
const NOTIFICATION_CHANNEL_NAME = "Discipline Status";

// // Big text style for long messages
// private void showBigTextNotification() {
//     String longText = "This is a very long message that will be truncated in the normal view. " +
//     "When expanded, the user will see the full content. This is useful for emails, " +
//     "chat messages, or any content that requires more space.";

//   NotificationCompat.BigTextStyle bigTextStyle = new NotificationCompat.BigTextStyle()
//     .bigText(longText)
//     .setBigContentTitle("Long Message")
//     .setSummaryText("New message");

//   NotificationCompat.Builder builder = new NotificationCompat.Builder(this, "regular_channel")
//     .setSmallIcon(R.drawable.ic_notification)
//     .setContentTitle("New Message")
//     .setContentText("You have a new message") // Preview text
//     .setStyle(bigTextStyle)
//     .setPriority(NotificationCompat.PRIORITY_DEFAULT);

//   NotificationManagerCompat.from(this).notify(3, builder.build());
// }

// // Inbox style for multiple items
// private void showInboxStyleNotification() {
//   NotificationCompat.InboxStyle inboxStyle = new NotificationCompat.InboxStyle();
//   inboxStyle.setBigContentTitle("5 New Messages");
//   inboxStyle.addLine("John: Hey, are you coming to the meeting?");
//   inboxStyle.addLine("Sarah: Can you review my pull request?");
//   inboxStyle.addLine("Mike: Happy birthday!");
//   inboxStyle.addLine("Team: Project deadline extended");
//   inboxStyle.addLine("System: Your update is ready");
//   inboxStyle.setSummaryText("+2 more messages");

//   NotificationCompat.Builder builder = new NotificationCompat.Builder(this, "important_channel")
//     .setSmallIcon(R.drawable.ic_notification)
//     .setContentTitle("New Messages")
//     .setContentText("You have 5 new messages")
//     .setStyle(inboxStyle)
//     .setNumber(5) // Show badge count
//     .setPriority(NotificationCompat.PRIORITY_HIGH);

//   NotificationManagerCompat.from(this).notify(4, builder.build());
// }

const createForegroundNotification = (context: Context): Tried<Notification, TextualError> => {
  try {
    const builder = new NotificationCompat.Builder(context, NOTIFICATION_CHANNEL_ID)
      .setContentTitle("Discipline Digital Detox Active...")
      .setContentText("Monitoring system and appling policies...")
      .setSmallIcon(android.R.drawable.ic_menu_info_details)
      .setPriority(NotificationCompat.PRIORITY_LOW)
      .setOngoing(true);
  
    return Tried.Success(builder.build());
  } catch (exception) {
    const error = TextualError.create("Creating Discipline's main notification");
    TextualError.addMessage(error, "An exception was thrown by Android's 'NotificationCompat.Builder'");
    TextualError.addUnknownAttachment(error, "Exception", exception);
    return Tried.Failure(error);
  }
};

const ensureNotificationChannelExists = (notificationManager: NotificationManager): Nullable<TextualError> => {
  if (Build.VERSION.PREVIEW_SDK_INT < Build.VERSION_CODES.O) {
    return Nullable.None();
  }

  try {
    if (notificationManager.getNotificationChannel(NOTIFICATION_CHANNEL_ID) !== null) {
      return Nullable.None();
    }
  } catch (exception) {
    const error = TextualError.create("Ensuring Discipline's main notification channel exists");
    TextualError.addMessage(error, "Android's 'NotificationManager.getNotificationChannel' threw an exception");
    TextualError.addUnknownAttachment(error, "Exception", exception);
    return Nullable.Some(error);
  }

  let channel;
  try {
    channel = new NotificationChannel(
      NOTIFICATION_CHANNEL_ID,
      NOTIFICATION_CHANNEL_NAME,
      NotificationManager.IMPORTANCE_HIGH,
    );
  } catch (exception) {
    const error = TextualError.create("Ensuring Discipline's main notification channel exists");
    TextualError.addMessage(error, "Android's 'NotificationChannel' constructor threw an exception");
    TextualError.addUnknownAttachment(error, "Exception", exception);
    return Nullable.Some(error);
  }

  try {  
    channel.setDescription("Shows that digital detox is actively monitoring app usage");
  } catch (exception) {
    const error = TextualError.create("Ensuring Discipline's main notification channel exists");
    TextualError.addMessage(error, "Android's 'NotificationChannel.setDescription' threw an exception");
    TextualError.addUnknownAttachment(error, "Exception", exception);
    return Nullable.Some(error);
  }

  try {
    notificationManager.createNotificationChannel(channel);
  } catch (exception) {
    const error = TextualError.create("Ensuring Discipline's main notification channel exists");
    TextualError.addMessage(error, "Android's 'NotificationManager.createNotificationChannel' threw an exception");
    TextualError.addUnknownAttachment(error, "Exception", exception);
    return Nullable.Some(error);
  }

  return Nullable.None();
};

type State = {
  readonly packageManager: PackageManager,
  readonly usageStatsManager: UsageStatsManager,
  readonly notificationManager: NotificationManager,
};

const create = (service: Service): Tried<State, TextualError> => {
  let packageManager: PackageManager | null;
  try {
    packageManager = service.getPackageManager();
  } catch (exception) {
    const error = TextualError.create("Creating 'DisciplineServiceState'");
    TextualError.addMessage(error, "Android's 'Service.getPackageManager' threw an exception");
    TextualError.addUnknownAttachment(error, "Exception", exception);
    return Tried.Failure(error);
  }
  if (packageManager === null) {
    const error = TextualError.create("Creating 'DisciplineServiceState'");
    TextualError.addMessage(error, "Android's 'Service.getPackageManager' returned 'null'");
    return Tried.Failure(error);
  }

  let usageStatsManager: UsageStatsManager | null;
  try {
    usageStatsManager = service.getSystemService(Context.USAGE_STATS_SERVICE); 
  } catch (exception) {
    const error = TextualError.create("Creating 'DisciplineServiceState'");
    TextualError.addMessage(error, "Android's 'Service.getSystemService(Service.USAGE_STATS_SERVICE)' threw an exception");
    TextualError.addUnknownAttachment(error, "Exception", exception);
    return Tried.Failure(error);   
  }
  if (usageStatsManager === null) {
    const error = TextualError.create("Creating 'DisciplineServiceState'");
    TextualError.addMessage(error, "Android's 'Service.getSystemService(Service.USAGE_STATS_SERVICE)' returned 'null'");
    return Tried.Failure(error);  
  }

  let notificationManager: NotificationManager | null;
  try {
    notificationManager = service.getSystemService(Service.NOTIFICATION_SERVICE);
  } catch (exception) {
    const error = TextualError.create("Creating 'DisciplineServiceState'");
    TextualError.addMessage(error, "Android's 'Service.getSystemService(Service.NOTIFICATION_SERVICE)' threw an exception");
    TextualError.addUnknownAttachment(error, "Exception", exception);
    return Tried.Failure(error);    
  }
  if (notificationManager === null) {
    const error = TextualError.create("Creating 'DisciplineServiceState'");
    TextualError.addMessage(error, "Android's 'Service.getSystemService(Service.NOTIFICATION_SERVICE)' returned 'null'");
    return Tried.Failure(error);
  }

  return Tried.Success({
    packageManager,
    usageStatsManager,
    notificationManager,
  });
};

let state: Nullable<State>;

const getOrInitialize = (service: Service): Tried<State, TextualError> => {
  if (state !== null) {
    return Tried.Success(state);
  }
  
  const stateOrError = create(service);
  if (Tried.isFailure(stateOrError)) {
    return stateOrError;
  }

  state = Tried.value(stateOrError);
  return Tried.Success(state);
};

const onCreate = (service: Service) => {
  let it;
  
  // it = State.staticOnDisciplineServiceCreated();
  // if (it !== null) {
  //   TextualError.changeContext(it, "Calling 'onCreate' on 'DisciplineService'");
  //   Log.e(LOG_TAG, TextualError.prettyPrint(it));
  //   return;
  // }

  it = getOrInitialize(service);
  if (Tried.isSuccess(it)) {
    it = Tried.value(it);
  } else {
    it = Tried.error(it);
    TextualError.changeContext(it, "Calling 'onCreate' on 'DisciplineService'");
    Log.e(LOG_TAG, TextualError.prettyPrint(it));
    return;
  }
  
  it = ensureNotificationChannelExists(it.notificationManager);
  if (it !== null) {
    TextualError.changeContext(it, "Calling 'onCreate' on 'DisciplineService'");
    Log.e(LOG_TAG, TextualError.prettyPrint(it));
    return;
  }

  Log.i(LOG_TAG, "Method 'onCreate' was called on 'DisciplineService' and returned successfully");
};

const onStartCommand = (service: Service) => {
  let it;

  // it = State.staticOnDisciplineServiceStartCommand();
  // if (it !== null) {
  //   TextualError.changeContext(it, "Calling 'onStartCommand' on 'DisciplineService'");
  //   Log.e(LOG_TAG, TextualError.prettyPrint(it));
  //   return;
  // }

  it = getOrInitialize(service);
  if (Tried.isSuccess(it)) {
    it = Tried.value(it);
  } else {
    it = Tried.error(it);
    TextualError.changeContext(it, "Calling 'onStartCommand' on 'DisciplineService'");
    Log.e(LOG_TAG, TextualError.prettyPrint(it));
    return;
  }

  it = ensureNotificationChannelExists(it.notificationManager);
  if (it !== null) {
    TextualError.changeContext(it, "Calling 'onStartCommand' on 'DisciplineService'");
    Log.e(LOG_TAG, TextualError.prettyPrint(it));
    return;
  }

  it = createForegroundNotification(service);
  if (Tried.isSuccess(it)) {
    it = Tried.value(it);
  } else {
    it = Tried.error(it);
    TextualError.changeContext(it, "Calling 'onStartCommand' on 'DisciplineService'");
    Log.e(LOG_TAG, TextualError.prettyPrint(it));
    return;
  }

  try {
    service.startForeground(NOTIFICATION_ID, it);
  } catch (exception) {
    it = TextualError.create(LOG_TAG);
    TextualError.changeContext(it, "Calling 'onStartCommand' on 'DisciplineService'");
    TextualError.addMessage(it, "'Service.startForeground' threw an exception");
    TextualError.addUnknownAttachment(it, "Exception", exception);
    Log.e(LOG_TAG, TextualError.prettyPrint(it));
    return;
  }

  Log.i(LOG_TAG, "Method 'onStartCommand' was called on 'DisciplineService' and returned successfully");
};

const onDestroy = (service: Service) => {
  let it;
  
  // it = State.staticOnDisciplineServiceDestroy(service);
  // if (it !== null) {
  //   TextualError.changeContext(it, "Calling 'onDestroy' on 'DisciplineService'");
  //   Log.e(LOG_TAG, TextualError.prettyPrint(it));
  //   return;
  // }
  
  state = null;
  Log.i(LOG_TAG, "Method 'onDestroy' was called on 'DisciplineService' and returned successfully");
};