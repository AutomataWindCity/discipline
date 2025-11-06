export type TUnique<Id extends string, T> = T & {
  readonly ___________TYPE_________: Id
}

export const create = <Id extends string, T>(value: Omit<T, "___________TYPE_________">): TUnique<Id, T> => {
  return value as TUnique<Id, T>;
}

export type Obscure<Id, T> = {
  readonly ____________________________VIRTUAL_TYPE_ID______________________: Id, 
};

export const obscure = <Id, T>(value: T): Obscure<Id, T> => {
  return value as Obscure<Id, T>;
};

export const unobscure = <Id, T>(value: Obscure<Id, T>): T => {
  return value as T;
};