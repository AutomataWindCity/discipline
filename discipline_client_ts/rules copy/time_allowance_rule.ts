import { Branded, Duration, Countdown, TextualError, Tried } from "../x.ts";

const brand = Symbol();

export type TimeAllowanceRule = Branded<typeof brand, {
  isEnabled: boolean,
  allowance: Duration,
  lifetime: Countdown,
}>;

export const MINIMUM_ALLOWANCE = Tried.unwrap(Duration.fromHours(1));
export const MAXIMUM_ALLOWANCE = Tried.unwrap(Duration.fromHours(24));
export const MAXIMUM_LIFETIME = Tried.unwrap(Duration.fromHours(24 * 3));

export const construct = (
  isEnabled: boolean,
  allowance: Duration,
  lifetime: Countdown,
): TimeAllowanceRule => {
  return Branded(brand, {
    isEnabled,
    allowance,
    lifetime,
  });
};

export const create = (
  allowance: Duration,
  lifetime: Countdown,
): Tried<TimeAllowanceRule, TextualError> => {
  if (Duration.isShorterThan(allowance, MINIMUM_ALLOWANCE)) {
    const it = TextualError.create("Creating a ScreenTimeAllowanceRule");
    TextualError.addMessage(it, "Allowance is too short");
    TextualError.addStringAttachment(it, "Minimum allowance", Duration.toString2(MINIMUM_ALLOWANCE));
    TextualError.addStringAttachment(it, "Provided allowance", Duration.toString2(allowance));
    return Tried.Failure(it);
  }

  if (Duration.isLongerThan(allowance, MAXIMUM_ALLOWANCE)) {
    const it = TextualError.create("Creating a ScreenTimeAllowanceRule");
    TextualError.addMessage(it, "Allowance is too long");
    TextualError.addStringAttachment(it, "Maximum allowance", Duration.toString2(MAXIMUM_ALLOWANCE));
    TextualError.addStringAttachment(it, "Provided allowance", Duration.toString2(allowance));
    return Tried.Failure(it);
  }

  if (Duration.isLongerThan(Countdown.getTotalDuration(lifetime), MAXIMUM_LIFETIME)) {
    const it = TextualError.create("Creating a ScreenTimeAllowanceRule");
    TextualError.addMessage(it, "Lifetime is too long");
    TextualError.addStringAttachment(it, "Maximum lifetime", Duration.toString2(MAXIMUM_LIFETIME))
    TextualError.addStringAttachment(it, "Provided lifetime", Duration.toString2(Countdown.getTotalDuration(lifetime)));
  }

  return Tried.Success(construct(
    false, 
    allowance, 
    lifetime,
  ));
};

export const getTotalAllowance = (it: TimeAllowanceRule): Duration => {
  return it.allowance;
};

export const getRemainingAllowance = (it: TimeAllowanceRule, dailyUptime: Duration): Duration => {
  return Duration.minusOrZero(it.allowance, dailyUptime);
};

export const isAllowanceUp = (it: TimeAllowanceRule, dailyUptime: Duration): boolean => {
  return Duration.isLongerThan(dailyUptime, it.allowance);
};

export const getLifetime = (it: TimeAllowanceRule): Countdown => {
  return it.lifetime;
};

export const TimeAllowanceRule = {
  MINIMUM_ALLOWANCE,
  MAXIMUM_ALLOWANCE,
  MAXIMUM_LIFETIME,
  create,
  construct,
  getTotalAllowance,
  getRemainingAllowance,
  isAllowanceUp,
  getLifetime,
};