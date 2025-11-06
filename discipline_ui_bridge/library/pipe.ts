type Fn2<A, B> = (value: A) => B;

export interface TPipeFn {
  <Value1>(value: Value1): Value1,
  <Value1, Value2>(value: Value1, fn1: Fn2<Value1, Value2>): Value2,
  <Value1, Value2, Value3>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>): Value3,
  <Value1, Value2, Value3, Value4>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>, fn3: Fn2<Value3, Value4>): Value4,
  <Value1, Value2, Value3, Value4, Value5>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>, fn3: Fn2<Value3, Value4>, fn4: Fn2<Value4, Value5>): Value5,
  <Value1, Value2, Value3, Value4, Value5, Value6>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>, fn3: Fn2<Value3, Value4>, fn4: Fn2<Value4, Value5>, fn5: Fn2<Value5, Value6>): Value6,
  <Value1, Value2, Value3, Value4, Value5, Value6, Value7>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>, fn3: Fn2<Value3, Value4>, fn4: Fn2<Value4, Value5>, fn5: Fn2<Value5, Value6>, fn6: Fn2<Value6, Value7>): Value7,
  <Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>, fn3: Fn2<Value3, Value4>, fn4: Fn2<Value4, Value5>, fn5: Fn2<Value5, Value6>, fn6: Fn2<Value6, Value7>, fn7: Fn2<Value7, Value8>): Value8,
  <Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>, fn3: Fn2<Value3, Value4>, fn4: Fn2<Value4, Value5>, fn5: Fn2<Value5, Value6>, fn6: Fn2<Value6, Value7>, fn7: Fn2<Value7, Value8>, fn8: Fn2<Value8, Value9>): Value9,
  <Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>, fn3: Fn2<Value3, Value4>, fn4: Fn2<Value4, Value5>, fn5: Fn2<Value5, Value6>, fn6: Fn2<Value6, Value7>, fn7: Fn2<Value7, Value8>, fn8: Fn2<Value8, Value9>, fn9: Fn2<Value9, Value10>): Value10,
  <Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>, fn3: Fn2<Value3, Value4>, fn4: Fn2<Value4, Value5>, fn5: Fn2<Value5, Value6>, fn6: Fn2<Value6, Value7>, fn7: Fn2<Value7, Value8>, fn8: Fn2<Value8, Value9>, fn9: Fn2<Value9, Value10>, fn10: Fn2<Value10, Value11>): Value11,
  <Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11, Value12>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>, fn3: Fn2<Value3, Value4>, fn4: Fn2<Value4, Value5>, fn5: Fn2<Value5, Value6>, fn6: Fn2<Value6, Value7>, fn7: Fn2<Value7, Value8>, fn8: Fn2<Value8, Value9>, fn9: Fn2<Value9, Value10>, fn10: Fn2<Value10, Value11>, fn11: Fn2<Value11, Value12>): Value12,
  <Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11, Value12, Value13>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>, fn3: Fn2<Value3, Value4>, fn4: Fn2<Value4, Value5>, fn5: Fn2<Value5, Value6>, fn6: Fn2<Value6, Value7>, fn7: Fn2<Value7, Value8>, fn8: Fn2<Value8, Value9>, fn9: Fn2<Value9, Value10>, fn10: Fn2<Value10, Value11>, fn11: Fn2<Value11, Value12>, fn12: Fn2<Value12, Value13>): Value13,
  <Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11, Value12, Value13, Value14>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>, fn3: Fn2<Value3, Value4>, fn4: Fn2<Value4, Value5>, fn5: Fn2<Value5, Value6>, fn6: Fn2<Value6, Value7>, fn7: Fn2<Value7, Value8>, fn8: Fn2<Value8, Value9>, fn9: Fn2<Value9, Value10>, fn10: Fn2<Value10, Value11>, fn11: Fn2<Value11, Value12>, fn12: Fn2<Value12, Value13>, fn13: Fn2<Value13, Value14>): Value14,
  <Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11, Value12, Value13, Value14, Value15>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>, fn3: Fn2<Value3, Value4>, fn4: Fn2<Value4, Value5>, fn5: Fn2<Value5, Value6>, fn6: Fn2<Value6, Value7>, fn7: Fn2<Value7, Value8>, fn8: Fn2<Value8, Value9>, fn9: Fn2<Value9, Value10>, fn10: Fn2<Value10, Value11>, fn11: Fn2<Value11, Value12>, fn12: Fn2<Value12, Value13>, fn13: Fn2<Value13, Value14>, fn14: Fn2<Value14, Value15>): Value15,
  <Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11, Value12, Value13, Value14, Value15, Value16>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>, fn3: Fn2<Value3, Value4>, fn4: Fn2<Value4, Value5>, fn5: Fn2<Value5, Value6>, fn6: Fn2<Value6, Value7>, fn7: Fn2<Value7, Value8>, fn8: Fn2<Value8, Value9>, fn9: Fn2<Value9, Value10>, fn10: Fn2<Value10, Value11>, fn11: Fn2<Value11, Value12>, fn12: Fn2<Value12, Value13>, fn13: Fn2<Value13, Value14>, fn14: Fn2<Value14, Value15>, fn15: Fn2<Value15, Value16>): Value16,
  <Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11, Value12, Value13, Value14, Value15, Value16, Value17>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>, fn3: Fn2<Value3, Value4>, fn4: Fn2<Value4, Value5>, fn5: Fn2<Value5, Value6>, fn6: Fn2<Value6, Value7>, fn7: Fn2<Value7, Value8>, fn8: Fn2<Value8, Value9>, fn9: Fn2<Value9, Value10>, fn10: Fn2<Value10, Value11>, fn11: Fn2<Value11, Value12>, fn12: Fn2<Value12, Value13>, fn13: Fn2<Value13, Value14>, fn14: Fn2<Value14, Value15>, fn15: Fn2<Value15, Value16>, fn16: Fn2<Value16, Value17>): Value17,
  <Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11, Value12, Value13, Value14, Value15, Value16, Value17, Value18>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>, fn3: Fn2<Value3, Value4>, fn4: Fn2<Value4, Value5>, fn5: Fn2<Value5, Value6>, fn6: Fn2<Value6, Value7>, fn7: Fn2<Value7, Value8>, fn8: Fn2<Value8, Value9>, fn9: Fn2<Value9, Value10>, fn10: Fn2<Value10, Value11>, fn11: Fn2<Value11, Value12>, fn12: Fn2<Value12, Value13>, fn13: Fn2<Value13, Value14>, fn14: Fn2<Value14, Value15>, fn15: Fn2<Value15, Value16>, fn16: Fn2<Value16, Value17>, fn17: Fn2<Value17, Value18>): Value18,
  <Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11, Value12, Value13, Value14, Value15, Value16, Value17, Value18, Value19>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>, fn3: Fn2<Value3, Value4>, fn4: Fn2<Value4, Value5>, fn5: Fn2<Value5, Value6>, fn6: Fn2<Value6, Value7>, fn7: Fn2<Value7, Value8>, fn8: Fn2<Value8, Value9>, fn9: Fn2<Value9, Value10>, fn10: Fn2<Value10, Value11>, fn11: Fn2<Value11, Value12>, fn12: Fn2<Value12, Value13>, fn13: Fn2<Value13, Value14>, fn14: Fn2<Value14, Value15>, fn15: Fn2<Value15, Value16>, fn16: Fn2<Value16, Value17>, fn17: Fn2<Value17, Value18>, fn18: Fn2<Value18, Value19>): Value19,
  <Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11, Value12, Value13, Value14, Value15, Value16, Value17, Value18, Value19, Value20>(value: Value1, fn1: Fn2<Value1, Value2>, fn2: Fn2<Value2, Value3>, fn3: Fn2<Value3, Value4>, fn4: Fn2<Value4, Value5>, fn5: Fn2<Value5, Value6>, fn6: Fn2<Value6, Value7>, fn7: Fn2<Value7, Value8>, fn8: Fn2<Value8, Value9>, fn9: Fn2<Value9, Value10>, fn10: Fn2<Value10, Value11>, fn11: Fn2<Value11, Value12>, fn12: Fn2<Value12, Value13>, fn13: Fn2<Value13, Value14>, fn14: Fn2<Value14, Value15>, fn15: Fn2<Value15, Value16>, fn16: Fn2<Value16, Value17>, fn17: Fn2<Value17, Value18>, fn18: Fn2<Value18, Value19>, fn19: Fn2<Value19, Value20>): Value20,

}

export const pipe = ((value: unknown, ...fns: ((value: unknown) => unknown)[]): unknown => {
  for (const fn of fns) {
    value = fn(value);
  }
  return value;
}) as TPipeFn;

type Currayable1<Parameter1 = any, Return = any> = (
  parameter1: Parameter1,
  ...paramaters: any[]
) => any;

type Parameter1<T> = 
  T extends (paramter1: infer Parameter1, ...parameters: any[]) => any
    ? Parameter1
    : never;

type Parameters<T> = 
  T extends (paramter1: any, ...parameters: infer Parameters) => any
    ? Parameters
    : T;
  
type Return<T> = 
  T extends (...parameters: any[]) => infer Return
    ? Return
    : never;

export class Pipe<T> {
  private constructor(private readonly inner: T) {}

  static withValue<T>(value: T) {
    return new Pipe(value);
  }

  curry1<Fn extends Currayable1<T>>(
    fn: Fn, 
    ...paramaters: Parameters<Fn>
  ): Pipe<ReturnType<Fn>> {
    return new Pipe(fn(this.inner, ...paramaters));
  }

  value() {
    return this.inner
  }
}