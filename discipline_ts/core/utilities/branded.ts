const TYPE_BRAND = Symbol();

export type Branded<Brand extends symbol, T> = (
  & T 
  & { [TYPE_BRAND]: Brand }
  & { [Key in Brand]: Brand }
);

export const Branded = <Brand extends symbol, T>(
  brand: Brand,
  value: Omit<T, Brand | typeof TYPE_BRAND>,
) => {
  return value as Branded<Brand, T>;
};

const NOMINAL_BRAND = Symbol();

export type Nominal<Brand, Type> = {
  virtualType: Type,
  virtualTypeBrand: Brand,
  virtualOwnBrand: typeof NOMINAL_BRAND,
};

export const create = <Brand, Type>(brand: Brand, type: Type): Nominal<Brand, Type> => {
  return type as any as Nominal<Brand, Type>;
};

export const get = <Brand, Type>(nominal: Nominal<Brand, Type>): Type => {
  return nominal as any as Type;
};

export const Nominal = {
  create,
  get,
};