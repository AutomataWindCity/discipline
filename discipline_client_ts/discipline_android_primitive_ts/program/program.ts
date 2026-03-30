import * as Path from "@std/path";
import { 
  Data1Deserialize, Data1Serialize, Data2Deserialize, 
  Data2Serialize, DateTime, Duration, MonotonicClock, 
  Storage, TextualError, UptimeClock, UserRegulation,
  Vault, Server, Conditionals, 
} from "../x.ts";

export class Data1 {
  private constructor(
    readonly vaults: Vault[],
    readonly luny: UserRegulation,
    readonly ruru: UserRegulation,
  ) {}

  static construct(
    luny: UserRegulation,
    ruru: UserRegulation,
    vaults: Vault[],
  ) {
    return new Data1(
      vaults, 
      luny, 
      ruru, 
    );
  }

  static default() {
    return new Data1(
      [], 
      UserRegulation.create(), 
      UserRegulation.create(),
    );
  }

  get conditionalNumber() {
    return this.luny.screen.length + 
      this.luny.internet.length + 
      this.ruru.screen.length + 
      this.ruru.internet.length;
  }
}

export class Data2 {
  private constructor(
    readonly monotonicClock: MonotonicClock,
    readonly uptimeClock: UptimeClock,
  ) {}

  static construct(
    monotonicClock: MonotonicClock,
    uptimeClock: UptimeClock,
  ) {
    return new Data2(monotonicClock, uptimeClock);
  }

  static default() {
    const now = DateTime.now();

    return new Data2(
      MonotonicClock.create(now),
      UptimeClock.create(now),
    );
  }
}

export class Program {
  private constructor(    
    private readonly data1: Data1,
    private readonly data2: Data2,
    private readonly storage1: Storage<Data1>,
    private readonly storage2: Storage<Data2>,
    private readonly clockSynchronizationInterval: Duration,
    private isSynchronizationLoopRunning: boolean,
    private didSynchronizeSinceSystemBoot: boolean,
    private readonly server: Server,
    readonly serverPort: number,
    readonly serverHostname: string,
    readonly serverAddress: string,
    readonly maximumConditionalNumber: number,
    readonly maximumVaultNumber: number,
  ) {}
  
  static async launch({
    clockSynchronizationInterval,
    dataDirectoryPath,
    maximumConditionalNumber,
    maximumVaultNumber,
    serverHostname,
    serverPort,
  }: {
    dataDirectoryPath: string,
    serverPort: number,
    serverHostname: string,
    maximumConditionalNumber: number,
    maximumVaultNumber: number,
    clockSynchronizationInterval: Duration,
  }) {
    const storage1 = Storage.create<Data1>(
      dataDirectoryPath,
      Path.join(dataDirectoryPath, "data_1_temp.json"),
      Path.join(dataDirectoryPath, "data_1.json"),
      Data1.default,
      Data1Serialize,
      Data1Deserialize,
    );

    const storage2 = Storage.create<Data2>(
      dataDirectoryPath,
      Path.join(dataDirectoryPath, "data_2_temp.json"),
      Path.join(dataDirectoryPath, "data_2.json"),
      Data2.default,
      Data2Serialize,
      Data2Deserialize,
    );

    const data1 = await storage1.get();
    if (TextualError.is(data1)) {
      return data1
        .changeContext("Reading 'Data1'")
        .changeContext("Creating 'program'");
    }

    const data2 = await storage2.get();
    if (TextualError.is(data2)) {
      return data2
        .changeContext("Reading 'Data2'")
        .changeContext("Creating 'program'");
    }

    const serverAddress = `http://${serverHostname}:${serverPort}`;
    const server = Server.start(serverPort, serverHostname, () => program);
    if (server instanceof Error) {
      return TextualError
        .create("Starting Discipline Server")
        .addErrorAttachment("Error", server)
        .changeContext("Starting Discipling Program");
    }

    const program = new Program(
      data1,
      data2,
      storage1,
      storage2,
      clockSynchronizationInterval, 
      false,
      false,
      server,
      serverPort,
      serverHostname,
      serverAddress,
      maximumConditionalNumber,
      maximumVaultNumber,
    );

    program.startSynchronizationLoop();
    return program;
  }

  private startSynchronizationLoop() {
    if (this.isSynchronizationLoopRunning) {
      return;
    }
  
    this.isSynchronizationLoopRunning = true;
  
    const timer = () => {
      if (this.isSynchronizationLoopRunning) {
        this.synchronize();
        setTimeout(timer, this.clockSynchronizationInterval.toTotalMilliseconds());
      }
    };
  
    timer();
  }

  get monotonicClock() {
    return this.data2.monotonicClock;
  }
  
  get uptimeClock() {
    return this.data2.uptimeClock;
  }
  
  get luny() {
    return this.data1.luny;
  }

  get ruru() {
    return this.data1.ruru;
  }

  get vaults() {
    return this.data1.vaults;
  }

  get conditionalNumber() {
    return this.data1.conditionalNumber;
  }

  get vaultNumber() {
    return this.data1.vaults.length;
  }

  async synchronize() {
    const now = DateTime.now();
    this.monotonicClock.synchronize(now);
    this.uptimeClock.synchronize(
      now, 
      this.clockSynchronizationInterval,
      this.didSynchronizeSinceSystemBoot,
    );

    const monotonicNow = this.monotonicClock.getNow();
    Conditionals.removeDead(this.luny.screen, monotonicNow);
    Conditionals.removeDead(this.luny.internet, monotonicNow);
    Conditionals.removeDead(this.ruru.screen, monotonicNow);
    Conditionals.removeDead(this.ruru.internet, monotonicNow);

    this.didSynchronizeSinceSystemBoot = true;

    const error = await this.saveData2();
    if (TextualError.is(error)) {
      console.error(
        error
          .changeContext("Saving changes to the filesystem")
          .changeContext("Synchronization program state")
          .print({ color: true })
      );
    }
  }
  
  saveData1() {
    return this.storage1.set(this.data1);
  }
  
  saveData2() {
    return this.storage2.set(this.data2);
  }
}
