class Box<Value> {
  private constructor(private readonly value: Value) {}

  static create<Value>(value: Value) {
    return new Box(value);
  }

  map<Return>(
    fn: (value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(this.value));
  }

  map1<Arg1, Return>(
    fn: (value: Value, arg1: Arg1) => Return,
    arg1: Arg1,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1));
  }

  map2<Arg1, Arg2, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2) => Return,
    arg1: Arg1,
    arg2: Arg2,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2));
  }

  map3<Arg1, Arg2, Arg3, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3));
  }

  map4<Arg1, Arg2, Arg3, Arg4, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3, arg4));
  }

  map5<Arg1, Arg2, Arg3, Arg4, Arg5, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3, arg4, arg5));
  }

  map6<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3, arg4, arg5, arg6));
  }

  map7<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3, arg4, arg5, arg6, arg7));
  }

  map8<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8));
  }

  map9<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9));
  }

  map10<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10));
  }

  map11<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11));
  }

  map12<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12));
  }

  map13<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12, arg13: Arg13) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
    arg13: Arg13,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13));
  }

  map14<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12, arg13: Arg13, arg14: Arg14) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
    arg13: Arg13,
    arg14: Arg14,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14));
  }

  map15<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12, arg13: Arg13, arg14: Arg14, arg15: Arg15) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
    arg13: Arg13,
    arg14: Arg14,
    arg15: Arg15,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15));
  }

  map16<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15, Arg16, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12, arg13: Arg13, arg14: Arg14, arg15: Arg15, arg16: Arg16) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
    arg13: Arg13,
    arg14: Arg14,
    arg15: Arg15,
    arg16: Arg16,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16));
  }

  map17<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15, Arg16, Arg17, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12, arg13: Arg13, arg14: Arg14, arg15: Arg15, arg16: Arg16, arg17: Arg17) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
    arg13: Arg13,
    arg14: Arg14,
    arg15: Arg15,
    arg16: Arg16,
    arg17: Arg17,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17));
  }

  map18<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15, Arg16, Arg17, Arg18, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12, arg13: Arg13, arg14: Arg14, arg15: Arg15, arg16: Arg16, arg17: Arg17, arg18: Arg18) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
    arg13: Arg13,
    arg14: Arg14,
    arg15: Arg15,
    arg16: Arg16,
    arg17: Arg17,
    arg18: Arg18,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, arg18));
  }

  map19<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15, Arg16, Arg17, Arg18, Arg19, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12, arg13: Arg13, arg14: Arg14, arg15: Arg15, arg16: Arg16, arg17: Arg17, arg18: Arg18, arg19: Arg19) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
    arg13: Arg13,
    arg14: Arg14,
    arg15: Arg15,
    arg16: Arg16,
    arg17: Arg17,
    arg18: Arg18,
    arg19: Arg19,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, arg18, arg19));
  }

  map20<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15, Arg16, Arg17, Arg18, Arg19, Arg20, Return>(
    fn: (value: Value, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12, arg13: Arg13, arg14: Arg14, arg15: Arg15, arg16: Arg16, arg17: Arg17, arg18: Arg18, arg19: Arg19, arg20: Arg20) => Return,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
    arg13: Arg13,
    arg14: Arg14,
    arg15: Arg15,
    arg16: Arg16,
    arg17: Arg17,
    arg18: Arg18,
    arg19: Arg19,
    arg20: Arg20,
  ): Box<Return> {
    return Box.create(fn(this.value, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, arg18, arg19, arg20));
  }

  
  mapR1<Arg1, Return>(
    arg1: Arg1,
    fn: (arg1: Arg1, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, this.value));
  }

  mapR2<Arg1, Arg2, Return>(
    arg1: Arg1,
    arg2: Arg2,
    fn: (arg1: Arg1, arg2: Arg2, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, this.value));
  }

  mapR3<Arg1, Arg2, Arg3, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, this.value));
  }

  mapR4<Arg1, Arg2, Arg3, Arg4, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, arg4, this.value));
  }

  mapR5<Arg1, Arg2, Arg3, Arg4, Arg5, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, arg4, arg5, this.value));
  }

  mapR6<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, arg4, arg5, arg6, this.value));
  }

  mapR7<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, arg4, arg5, arg6, arg7, this.value));
  }

  mapR8<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, this.value));
  }

  mapR9<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, this.value));
  }

  mapR10<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, this.value));
  }

  mapR11<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, this.value));
  }

  mapR12<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, this.value));
  }

  mapR13<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
    arg13: Arg13,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12, arg13: Arg13, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, this.value));
  }

  mapR14<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
    arg13: Arg13,
    arg14: Arg14,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12, arg13: Arg13, arg14: Arg14, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, this.value));
  }

  mapR15<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
    arg13: Arg13,
    arg14: Arg14,
    arg15: Arg15,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12, arg13: Arg13, arg14: Arg14, arg15: Arg15, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, this.value));
  }

  mapR16<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15, Arg16, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
    arg13: Arg13,
    arg14: Arg14,
    arg15: Arg15,
    arg16: Arg16,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12, arg13: Arg13, arg14: Arg14, arg15: Arg15, arg16: Arg16, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, this.value));
  }

  mapR17<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15, Arg16, Arg17, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
    arg13: Arg13,
    arg14: Arg14,
    arg15: Arg15,
    arg16: Arg16,
    arg17: Arg17,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12, arg13: Arg13, arg14: Arg14, arg15: Arg15, arg16: Arg16, arg17: Arg17, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, this.value));
  }

  mapR18<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15, Arg16, Arg17, Arg18, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
    arg13: Arg13,
    arg14: Arg14,
    arg15: Arg15,
    arg16: Arg16,
    arg17: Arg17,
    arg18: Arg18,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12, arg13: Arg13, arg14: Arg14, arg15: Arg15, arg16: Arg16, arg17: Arg17, arg18: Arg18, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, arg18, this.value));
  }

  mapR19<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15, Arg16, Arg17, Arg18, Arg19, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
    arg13: Arg13,
    arg14: Arg14,
    arg15: Arg15,
    arg16: Arg16,
    arg17: Arg17,
    arg18: Arg18,
    arg19: Arg19,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12, arg13: Arg13, arg14: Arg14, arg15: Arg15, arg16: Arg16, arg17: Arg17, arg18: Arg18, arg19: Arg19, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, arg18, arg19, this.value));
  }

  mapR20<Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9, Arg10, Arg11, Arg12, Arg13, Arg14, Arg15, Arg16, Arg17, Arg18, Arg19, Arg20, Return>(
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
    arg6: Arg6,
    arg7: Arg7,
    arg8: Arg8,
    arg9: Arg9,
    arg10: Arg10,
    arg11: Arg11,
    arg12: Arg12,
    arg13: Arg13,
    arg14: Arg14,
    arg15: Arg15,
    arg16: Arg16,
    arg17: Arg17,
    arg18: Arg18,
    arg19: Arg19,
    arg20: Arg20,
    fn: (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10, arg11: Arg11, arg12: Arg12, arg13: Arg13, arg14: Arg14, arg15: Arg15, arg16: Arg16, arg17: Arg17, arg18: Arg18, arg19: Arg19, arg20: Arg20, value: Value) => Return,
  ): Box<Return> {
    return Box.create(fn(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17, arg18, arg19, arg20, this.value));
  }
}

export const box = Box.create;