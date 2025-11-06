import { TUnique, Unique } from "../mod.ts";

export type TEquality<A, B = A> = TUnique<"Equality", {
  isEqualTo: (a: A, b: B) => boolean;
  isInequalTo: (a: A, b: B) => boolean;
}>;

export const implement = <A, B = A>({
  isEqualTo,
  isInequalTo,
}: {
  isEqualTo: (a: A, b: B) => boolean;
  isInequalTo?: (a: A, b: B) => boolean;
}): TEquality<A, B> => {
  return Unique.create({
    isEqualTo,
    // isInequalTo(a, b) {
    //   return true
    // },
    isInequalTo: isInequalTo ?? ((a, b) => {
      return !isEqualTo(a, b);
    }),
  });
};