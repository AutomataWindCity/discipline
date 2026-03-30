import { App, Countdown, CountdownRule, Duration, MonotonicClock } from "../x.ts";

const createScreenCountdownRule = (
  app: App,
  duration: Duration,
) => {
  const countdown = Countdown.create(
    MonotonicClock.getNow(app.state.monotonicClock),
    duration,
  );

  const rule = CountdownRule.create(countdown);
};

const deleteScreenCountdownRule = () => {};
const createScreenTimeRangeRule = () => {};
const deleteScreenTimeRangeRule = () => {};
const createScreenTimeAllowanceRule = () => {};
const deleteScreenTimeAllowanceRule = () => {};

const createApplicationRule = () => {};
const deleteApplicationRule = () => {};

const createApplicationCountdownRule = () => {};
const deleteApplicationCountdownRule = () => {};
const createApplicationTimeRangeRule = () => {};
const deleteApplicationTimeRangeRule = () => {};
const createApplicationTimeAllowanceRule = () => {};
const deleteApplicationTimeAllowanceRule = () => {};