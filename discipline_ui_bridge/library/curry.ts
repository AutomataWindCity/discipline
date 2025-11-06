type Fn = (...paramaters: any[]) => any;

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

type Curried<T> = (it: Parameter1<T>) => Return<T>;

export const curry1 = <T extends Fn>(fn: T, ...paramaters: Parameters<T>): Curried<T> => {
  return (it) => {
    return fn(it, ...paramaters);
  };
};
