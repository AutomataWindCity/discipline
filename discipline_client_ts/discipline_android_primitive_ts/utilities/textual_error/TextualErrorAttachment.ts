import { Branded } from "../../x.ts";

const brand = Symbol();

export type TextualErrorAttachment = Branded<typeof brand, {
  readonly name: string,
  readonly value: string,
}>;

export const create = (name: string, value: string): TextualErrorAttachment => {
  return Branded(brand, {
    name,
    value,
  });
};

export const TextualErrorAttachment = {
  create,
};