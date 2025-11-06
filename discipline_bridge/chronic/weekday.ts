import { TypeId } from "../mod.ts";

export interface WeekdayMatchCases<MonReturn, TueReturn, WedReturn, ThuReturn, FriReturn, SatReturn, SunReturn> {
  Mon: () => MonReturn,
  Tue: () => TueReturn,
  Wed: () => WedReturn,
  Thu: () => ThuReturn,
  Fri: () => FriReturn,
  Sat: () => SatReturn,
  Sun: () => SunReturn,
}

export class Weekday {
  readonly typeId = TypeId.Weekday;

  private constructor(private readonly inner: number) {}

  private static readonly MON_AS_NUMBER = 0;
  private static readonly TUE_AS_NUMBER = 1;
  private static readonly WED_AS_NUMBER = 2;
  private static readonly THU_AS_NUMBER = 3;
  private static readonly FRI_AS_NUMBER = 4;
  private static readonly SAT_AS_NUMBER = 5;
  private static readonly SUN_AS_NUMBER = 6;

  private static readonly MON = new Weekday(Weekday.MON_AS_NUMBER);
  private static readonly TUE = new Weekday(Weekday.TUE_AS_NUMBER);
  private static readonly WED = new Weekday(Weekday.WED_AS_NUMBER);
  private static readonly THU = new Weekday(Weekday.THU_AS_NUMBER);
  private static readonly FRI = new Weekday(Weekday.FRI_AS_NUMBER);
  private static readonly SAT = new Weekday(Weekday.SAT_AS_NUMBER);
  private static readonly SUN = new Weekday(Weekday.SUN_AS_NUMBER);

  static Mon(): Weekday {
    return Weekday.MON;
  }
  static Tue(): Weekday {
    return Weekday.TUE;
  }
  static Wed(): Weekday {
    return Weekday.WED;
  }
  static Thu(): Weekday {
    return Weekday.THU;
  }
  static Fri(): Weekday {
    return Weekday.FRI;
  }
  static Sat(): Weekday {
    return Weekday.SAT;
  }
  static Sun(): Weekday {
    return Weekday.SUN;
  }

  match<MonReturn, TueReturn, WedReturn, ThuReturn, FriReturn, SatReturn, SunReturn>(cases: WeekdayMatchCases<MonReturn, TueReturn, WedReturn, ThuReturn, FriReturn, SatReturn, SunReturn>) {
    switch (this.inner) {
      case Weekday.MON_AS_NUMBER: return cases.Mon();
      case Weekday.TUE_AS_NUMBER: return cases.Tue();
      case Weekday.WED_AS_NUMBER: return cases.Wed();
      case Weekday.THU_AS_NUMBER: return cases.Thu();
      case Weekday.FRI_AS_NUMBER: return cases.Fri();
      case Weekday.SAT_AS_NUMBER: return cases.Sat();
      case Weekday.SUN_AS_NUMBER: return cases.Sun();
      default: throw new Error("Matching against weekday number: Unreachable")
    } 
  }
}

