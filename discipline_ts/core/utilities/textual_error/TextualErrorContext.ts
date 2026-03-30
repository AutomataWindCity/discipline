import { Branded } from "../../x.ts";
import { TextualErrorAttachment } from "./TextualErrorAttachment.ts"

const brand = Symbol();

export type TextualErrorContext = Branded<typeof brand, {
  readonly action: string,
  readonly errorMessages: string[],
  readonly infoMessages: string[],
  readonly attachements: TextualErrorAttachment[],
}>;

export const create = (action: string): TextualErrorContext => {
  return Branded(brand, {
    action,
    attachements: [],
    infoMessages: [],
    errorMessages: [],
  });
};

export const TextualErrorContext = {
  create,
};