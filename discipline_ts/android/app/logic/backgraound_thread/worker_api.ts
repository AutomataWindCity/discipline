import { Nullable, TextualError, Tried } from "../discipline.ts"
import { UserHandle, UsageStatsManager, Context } from "../android.ts"
import { Utils } from "@nativescript/core";

export type WorkerApi = {
  readonly worker: Worker,
  readonly usageStatsManager: UsageStatsManager,
};

const create = (): Tried<WorkerApi, TextualError> => {
  let worker;
  
  try {
    // TODO: 
    worker = new Worker("./worker.ts");
  } catch (exception) {
    const error = TextualError.create("Creating Discipline's 'BackgraoundThread'");
    TextualError.addMessage(error, "NativeScript's 'Worker' constructor threw an exception");
    TextualError.addUnknownAttachment(error, "Exception", exception);
    return Tried.Failure(error);
  }

  let context;
  try {
    context = Utils.android.getApplicationContext();
  } catch (exception) {
    const error = TextualError.create("Creating Discipline's 'BackgraoundThread'");
    TextualError.addMessage(error, "An exception was thrown when retreiving the Android Application Context");
    TextualError.addUnknownAttachment(error, "Exception", exception);
    return Tried.Failure(error);
  }

  // let packageManager: PackageManager | null;
  // try {
  //   packageManager = context.getPackageManager();
  // } catch (exception) {
  //   const error = TextualError.create("");
  //   return Tried.Failure(error);
  // }
  // if (packageManager === null) {
  //   const error = TextualError.create("");
  //   return Tried.Failure(error); 
  // }

  let usageStatsManager: UsageStatsManager | null;
  try {
    usageStatsManager = context.getSystemService(Context.USAGE_STATS_SERVICE); 
  } catch (exception) {
    const error = TextualError.create("Creating Discipline's 'BackgraoundThread'");
    TextualError.addMessage(error, "Android's 'Context.getSystemService(Context.USAGE_STATS_SERVICE)' threw an exception");
    TextualError.addUnknownAttachment(error, "Exception", exception);
    return Tried.Failure(error);
  }
  if (usageStatsManager === null) {
    const error = TextualError.create("Creating Discipline's 'BackgraoundThread'");
    TextualError.addMessage(error, "Android's 'Context.getSystemService(Context.USAGE_STATS_SERVICE)' returned 'null'");
    return Tried.Failure(error);
  }

  // let notificationManager: NotificationManager | null;
  // try {
  //   notificationManager = context.getSystemService(Service.NOTIFICATION_SERVICE);
  // } catch (exception) {
  //   const error = TextualError.create("");
  //   return Tried.Failure(error);    
  // }
  // if (notificationManager === null) {
  //   const error = TextualError.create("");
  //   return Tried.Failure(error);
  // }

  return Tried.Success({
    // packageManager,
    worker,
    usageStatsManager,
    // notificationManager,
  });
};

const onProfileProvisioningComplete = (it: WorkerApi): Nullable<TextualError> => {
  return null;
};

const onDeviceAdminEnabled = (it: WorkerApi): Nullable<TextualError> => {
  return null;
};

const onDeviceAdminDisabled = (it: WorkerApi): Nullable<TextualError> => {
  return null;
};

const onUserStarted = (it: WorkerApi, user: UserHandle): Nullable<TextualError> => {
  return null;
};

const onUserStopped = (it: WorkerApi, user: UserHandle): Nullable<TextualError> => {
  return null;
};

const onUserRemoved = (it: WorkerApi, user: UserHandle): Nullable<TextualError> => {
  return null;
};

const onUserSwitched = (it: WorkerApi, user: UserHandle): Nullable<TextualError> => {
  return null;
};

const onServiceCreated = (it: WorkerApi): Nullable<TextualError> => {
  return null;
};

const onServiceStartCommand = (it: WorkerApi): Nullable<TextualError> => {
  return null;
};

const onServiceDestroy = (it: WorkerApi): Nullable<TextualError> => {
  return null;
};

let instance: Nullable<WorkerApi>;

const getOrInitialize = (): Tried<WorkerApi, TextualError> => {
  if (instance !== null) {
    return Tried.Success(instance);
  }

  const instanceOrError = create();
  if (Tried.isFailure(instanceOrError)) {
    return instanceOrError;
  }

  instance = Tried.value(instanceOrError);
  return instanceOrError;
};

const staticOnProfileProvisioningComplete = () => {
  return Tried.match(
    getOrInitialize(), {
      Failure: error =>
        TextualError.changeContext(error, "Calling 'staticOnProfileProvisioningComplete' on 'WorkerApi'"),

      Success: it =>
        onProfileProvisioningComplete(it),
    },
  );
};

const staticOnDeviceAdminEnabled = () => {
  return Tried.match(
    getOrInitialize(), {
      Failure: error =>
        TextualError.changeContext(error, "Calling 'staticOnDeviceAdminEnabled' on 'WorkerApi'"),

      Success: it =>
        onDeviceAdminEnabled(it),
    },
  );
};

const staticOnDeviceAdminDisabled = () => {
  return Tried.match(
    getOrInitialize(), {
      Failure: error =>
        TextualError.changeContext(error, "Calling 'staticOnDeviceAdminDisabled' on 'WorkerApi'"),

      Success: it =>
        onDeviceAdminDisabled(it),
    },
  );
};

const staticOnUserStarted = (user: UserHandle) => {
  return Tried.match(
    getOrInitialize(), {
      Failure: error =>
        TextualError.changeContext(error, "Calling 'staticOnUserStarted' on 'WorkerApi'"),

      Success: it =>
        onUserStarted(it, user),
    },
  );
};

const staticOnUserStopped = (user: UserHandle) => {
  return Tried.match(
    getOrInitialize(), {
      Failure: error =>
        TextualError.changeContext(error, "Calling 'staticOnUserStopped' on 'WorkerApi'"),

      Success: it =>
        onUserStopped(it, user),
    },
  );
};

const staticOnUserSwitched = (user: UserHandle) => {
  return Tried.match(
    getOrInitialize(), {
      Failure: error =>
        TextualError.changeContext(error, "Calling 'staticOnUserWsitched' on 'WorkerApi'"),

      Success: it =>
        onUserSwitched(it, user),
    },
  );
};

const staticOnUserRemoved = (user: UserHandle) => {
  return Tried.match(
    getOrInitialize(), {
      Failure: error =>
        TextualError.changeContext(error, "Calling 'staticOnUserRemoved' on 'WorkerApi'"),

      Success: it =>
        onUserRemoved(it, user),
    },
  );
};

const staticOnDisciplineServiceCreated = (): Nullable<TextualError> => {
  return Nullable.None();
};

const staticOnDisciplineServiceStartCommand = (): Nullable<TextualError> => {
  return Tried.getErrorAsNullable(getOrInitialize());
};

const staticOnDisciplineServiceDestroy = (): Nullable<TextualError> => {
  if (instance === null) {
    // TODO: 
    // Log.i("", "")
    return null;
  }

  try {
    instance.worker.terminate();
  } catch (exception) {
    const error = TextualError.create("Closing Discipline's 'Worker'")
    TextualError.addMessage(error, "NativeScript's 'Worker.terminate' threw an exception");
    TextualError.addUnknownAttachment(error, "Exception", exception);
    return error;
  }

  return null;
};

export const WorkerApi = {
  staticOnProfileProvisioningComplete,
  staticOnDeviceAdminEnabled,
  staticOnDeviceAdminDisabled,
  staticOnUserStarted,
  staticOnUserStopped,
  staticOnUserSwitched,
  staticOnUserRemoved,
  staticOnDisciplineServiceCreated,
  staticOnDisciplineServiceStartCommand,
  staticOnDisciplineServiceDestroy,
};