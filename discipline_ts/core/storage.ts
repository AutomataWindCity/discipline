import { Deserialize, Serialize, TextualError } from "./x.ts";

const isNotFound = (value: unknown): value is Deno.errors.NotFound => {
  return value instanceof Deno.errors.NotFound;
};

const writeOrCreate = async (
  directoryPath: string,
  filePath: string,
  data: string,
) => {
  try {
    await Deno.writeTextFile(filePath, data, { create: true,  });
    return;
  } catch (cause) {
    if (!isNotFound(cause)) {
      return TextualError
        .create("A fatal io error occured")
        .addStringAttachment("Error", String(cause))
        .changeContext("Writing data to a file")
        .addStringAttachment("File path", filePath)
        .addStringAttachment("Directory path", directoryPath)
        .addStringAttachment("Data", data);
    }

    try {
      await Deno.mkdir(directoryPath);
    } catch (cause) {
      return TextualError
        .create("Writing data to a file")
        .addMessage("The file and its parent directory didn't exist. Attempting to create the directory resulted in a fatal io error")
        .addStringAttachment("File path", filePath)
        .addStringAttachment("Directory path", directoryPath)
        .addStringAttachment("Data", data)
        .addStringAttachment("Error", String(cause));
    }

    try {
      await Deno.writeTextFile(filePath, data, { create: true });
    } catch (cause) {
      return TextualError
        .create("Writing data to file")
        .addMessage("The directory didn't exist previously, but we just created it. Attempting to create the file and write to it resulted in a fatal io error")
        .addStringAttachment("File path", filePath)
        .addStringAttachment("Directory path", directoryPath)
        .addStringAttachment("Data", data)
        .addStringAttachment("Error", String(cause));
    }
  }
};

const atomicWrite = async <Value>(
  directoryPath: string, 
  tempFilePath: string, 
  actualFilePath: string, 
  data: Value,
  serialize: Serialize<Value>,
) => {
  const dataAsString = serialize.serialize(data);
  if (TextualError.is(dataAsString)) {
    return dataAsString
      .changeContext("Serializing data")
      .changeContext("Atomically writing data to a file");
  }

  const error = await writeOrCreate(directoryPath, tempFilePath, dataAsString);
  if (TextualError.is(error)) {
    return error.changeContext("Atomically writing data to a file");
  }

  try {
    await Deno.rename(tempFilePath, actualFilePath);
    return;
  } catch (cause) {
    return TextualError
      .create("Committing the changes from the temp file to the actual file")
      .addMessage("A fatal io error occured")
      .addStringAttachment("Temp file path", tempFilePath)
      .addStringAttachment("Actual file path", actualFilePath)
      .addStringAttachment("Data", dataAsString)
      .addStringAttachment("Error", String(cause));
  }    
};

const atomicRead = async <Value>(
  directoryPath: string,
  tempFilePath: string,
  actualFilePath: string,
  serialize: Serialize<Value>,
  deserialize: Deserialize<Value>,
  createDefault: () => Value,
) => {
  try {
    const dataAsString = await Deno.readTextFile(actualFilePath);
    const data = deserialize.deserialize(dataAsString);
    if (TextualError.is(data)) {
      return data
        .changeContext("Deserializing file content")
        .changeContext("Reading a text file")
        .addStringAttachment("File path", actualFilePath);
    }

    return data;
  } catch (cause) {
    if (!isNotFound(cause)) {
      return TextualError 
        .create("Reading a text file")
        .addMessage("A fatal io error occured")
        .addStringAttachment("File path", actualFilePath)
        .addStringAttachment("Error", String(cause));
    }
    
    // const dataAsStringOrError = serialize(defaultData);
    // if (dataAsStringOrError instanceof Error) {
    //   return new Error(`Reading a text file: File doesn't exist: Attempting to initialize file: Created default file data but couldn't serialize it. File path: ${actualFilePath}`, {
    //     // cause: dataAsStringOrError,
    //   });
    // }

    const defaultData = createDefault();
    const writeError = await atomicWrite(
      directoryPath, 
      tempFilePath, 
      actualFilePath, 
      defaultData, 
      serialize,
    );
    if (TextualError.is(writeError)) {
      return writeError
        .changeContext("Initializing text file")
        .changeContext("Reading a text file... File doesn't exist");
    }

    return defaultData;
  }
}

export class Storage<Data> {
  private directoryPath: string;
  private tempFilePath: string;
  private actualFilePath: string;
  private createDefault: () => Data;
  private serialize: Serialize<Data>;
  private deserialize: Deserialize<Data>;

  private constructor(
    directoryPath: string,
    tempFilePath: string,
    actualFilePath: string,
    createDefault: () => Data,
    serialize: Serialize<Data>,
    deserialize: Deserialize<Data>,
  ) {
    this.directoryPath = directoryPath;
    this.tempFilePath = tempFilePath;
    this.actualFilePath = actualFilePath;
    this.createDefault = createDefault;
    this.serialize = serialize;
    this.deserialize = deserialize;
  }

  static create<Data>(
    directoryPath: string,
    tempFilePath: string,
    actualFilePath: string,
    createDefault: () => Data,
    serialize: Serialize<Data>,
    deserialize: Deserialize<Data>,
  ) {
    return new Storage(
      directoryPath,
      tempFilePath,
      actualFilePath,
      createDefault,
      serialize,
      deserialize,
    );
  }

  get() {
    return atomicRead(
      this.directoryPath,
      this.tempFilePath,
      this.actualFilePath,
      this.serialize,
      this.deserialize,
      this.createDefault,
    );
  }

  set(newData: Data) {
    return atomicWrite(
      this.directoryPath,
      this.tempFilePath,
      this.actualFilePath,
      newData,
      this.serialize,
    );
  }
}