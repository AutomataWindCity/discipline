import { Integer, TextualError } from "../x.ts";

export const enum ConditionalVariantEnum {
  Countdown,
  CountdownAfterPlea,
}

const COUNTDOWN_AS_NUMBER = Integer.uncheckedFromNumber(0);
const COUNTDOWN_AFTER_PLEA_AS_NUMBER = Integer.uncheckedFromNumber(1);

const fromIntegerOrThrow = (integer: Integer): ConditionalVariantEnum => {
  switch (integer) {
    case COUNTDOWN_AS_NUMBER: {
      return ConditionalVariantEnum.Countdown;
    }
    case COUNTDOWN_AFTER_PLEA_AS_NUMBER: {
      return ConditionalVariantEnum.CountdownAfterPlea;
    }
    default: {
      const textualError = TextualError.create("Creating ConditionalVariant from integer");
      TextualError.addMessage(textualError, `Unrecognized value. Expected ${COUNTDOWN_AS_NUMBER} for Countdown or ${COUNTDOWN_AFTER_PLEA_AS_NUMBER} for CountdownAfterPlea, but found something else`)
      TextualError.addNumberAttachment(textualError, "Integer", integer);
      throw TextualError.toJsError(textualError);
    }
  }
};

const toInteger = (it: ConditionalVariantEnum): Integer => {
  switch (it) {
    case ConditionalVariantEnum.Countdown: {
      return COUNTDOWN_AS_NUMBER;
    }
    case ConditionalVariantEnum.CountdownAfterPlea: {
      return COUNTDOWN_AFTER_PLEA_AS_NUMBER;
    }
  }
};

const match = <R1, R2>(
  it: ConditionalVariantEnum,
  onCountdown: (it: ConditionalVariantEnum.Countdown) => R1,
  onCountdownAfterPlea: (it: ConditionalVariantEnum.CountdownAfterPlea) => R2,
) => {
  switch (it) {
    case ConditionalVariantEnum.Countdown: {
      return onCountdown(it);
    }
    case ConditionalVariantEnum.CountdownAfterPlea: {
      return onCountdownAfterPlea(it);
    }
  }
};

export const ConditionalVariant = {
  fromIntegerOrThrow,
  toInteger,
  match,
};