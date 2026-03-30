import { Duration, Time, Branded, Tried, TextualError } from "../x.ts";

const BRAND = Symbol();

export type TimeRange = Branded<typeof BRAND, {
  from: number,
  till: number,
}>;

export const construct = (from: number, till: number): TimeRange => {
  return Branded(BRAND, { from, till });
};

export const MINIMUM_FROM_TIMESTAMP = 0;
export const MAXIMUM_FROM_TIMESTAMP = 1000 * 60 * 60 * 24 - 1;

export const MINIMUM_TILL_TIMESTAMP = 0;
export const MAXIMUM_TILL_TIMESTAMP = 1000 * 60 * 60 * 24 * 2 - 1;


export const fromTimestamps = (from: number, till: number): Tried<TimeRange, TextualError> => {
  if (!Number.isInteger(from)) {
    const it = TextualError.create("Creating a TimeRange from timestamps");
    TextualError.addMessage(it, "Argument 'from' is not an integer");
    TextualError.addNumberAttachment(it, "Argument 'from'", from);
    return Tried.Failure(it);
  }
  if (from < MINIMUM_FROM_TIMESTAMP) {
    const it = TextualError.create("Creating a TimeRange from timestamps");
    TextualError.addMessage(it, "Argument 'from' is less than the minimum valid value");
    TextualError.addNumberAttachment(it, "Argument 'from'", from);
    TextualError.addNumberAttachment(it, "Minimum valid value", MINIMUM_FROM_TIMESTAMP);
    return Tried.Failure(it);
  }
  if (from > MAXIMUM_FROM_TIMESTAMP) {
    const it = TextualError.create("Creating a TimeRange from timestamps");
    TextualError.addMessage(it, "Argument 'from' is less than the maximum valid value");
    TextualError.addNumberAttachment(it, "Argument 'from'", from);
    TextualError.addNumberAttachment(it, "Maximum valid value", MAXIMUM_FROM_TIMESTAMP);
    return Tried.Failure(it);
  }

  if (!Number.isInteger(till)) {
    const it = TextualError.create("Creating a TimeRange till timestamps");
    TextualError.addMessage(it, "Argument 'till' is not an integer");
    TextualError.addNumberAttachment(it, "Argument 'till'", till);
    return Tried.Failure(it);
  }
  if (till < MINIMUM_TILL_TIMESTAMP) {
    const it = TextualError.create("Creating a TimeRange till timestamps");
    TextualError.addMessage(it, "Argument 'till' is less than the minimum valid value");
    TextualError.addNumberAttachment(it, "Argument 'till'", till);
    TextualError.addNumberAttachment(it, "Minimum valid value", MINIMUM_TILL_TIMESTAMP);
    return Tried.Failure(it);
  }
  if (till > MAXIMUM_TILL_TIMESTAMP) {
    const it = TextualError.create("Creating a TimeRange till timestamps");
    TextualError.addMessage(it, "Argument 'till' is less than the maximum valid value");
    TextualError.addNumberAttachment(it, "Argument 'till'", till);
    TextualError.addNumberAttachment(it, "Maximum valid value", MAXIMUM_TILL_TIMESTAMP);
    return Tried.Failure(it);
  }

  if (from > till) {
    const it = TextualError.create("Creating a TimeRange till timestamps");
    TextualError.addMessage(it, "Argument 'from' is greater than 'till', thereby refering to a later time.");
    TextualError.addNumberAttachment(it, "Argument 'from'", till);
    TextualError.addNumberAttachment(it, "Argument 'till'", till);
    return Tried.Failure(it); 
  }

  if (till - from >= Duration.MILLISECONDS_PER_DAY) {
    const it = TextualError.create("Creating a TimeRange till timestamps");
    TextualError.addMessage(it, "Arguments 'from' and 'till' specift a time range that is longer than 24 hours");
    TextualError.addNumberAttachment(it, "Argument 'from'", from);
    TextualError.addNumberAttachment(it, "Argument 'till'", till);
    return Tried.Failure(it);  
  }

  return Tried.Success(construct(from, till));
};

export const fromTimes = (from: Time, till: Time): TimeRange => {
  const fromTimestamp = Time.getTimestamp(from);
  const tillTimestamp = Time.getTimestamp(till);

  if (fromTimestamp <= tillTimestamp) {
    return construct(fromTimestamp, tillTimestamp);
  } else {
    return construct(fromTimestamp, tillTimestamp + Duration.MILLISECONDS_PER_DAY);
  }
};

export const contains = (it: TimeRange, time: Time): boolean => {
  return (
    Time.getTimestamp(time) >= it.from
    &&
    Time.getTimestamp(time) <= it.till
  );
};

export const getFrom = (it: TimeRange): Time => {
  return Tried.unwrap(Time.fromTimestamp(it.from));
};

export const getTill = (it: TimeRange): Time => {
  return Tried.unwrap(
    Time.fromTimestamp(
      it.till <= MAXIMUM_FROM_TIMESTAMP 
        ? it.till
        : it.till - MAXIMUM_FROM_TIMESTAMP
    ),
  );
};

export const getFromTimestamp = (it: TimeRange): number => {
  return it.from;
};

export const getTillTimestamp = (it: TimeRange): number => {
  return it.till;
};

export const toString = (it: TimeRange): string => {
  const from = Time.toString(getFrom(it));
  const till = Time.toString(getTill(it));
  return `${from} .. ${till}`;
};

export const TimeRange = {
  MINIMUM_FROM_TIMESTAMP,
  MAXIMUM_FROM_TIMESTAMP,
  MINIMUM_TILL_TIMESTAMP,
  MAXIMUM_TILL_TIMESTAMP,
  fromHour12AndMinuteOrNone: fromTimestamps,
  fromTimes,
  contains,
  getFrom,
  getTill,
  getFromTimestamp,
  getTillTimestamp,
  toString,
};