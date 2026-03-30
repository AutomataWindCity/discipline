import { Branded } from "../../x.ts";
import { TextualErrorContext } from "./TextualErrorContext.ts";
import { TextualErrorAttachment } from "./TextualErrorAttachment.ts";

const brand = Symbol();

export type TextualError = Branded<typeof brand, {
  context: TextualErrorContext,
  readonly earlierContexts: TextualErrorContext[],
}>;

export const create = (action: string): TextualError => {
  return Branded(brand, {
    context: TextualErrorContext.create(action),
    earlierContexts: [],
  });
};

export const changeContext = (it: TextualError, newContextAction: string): TextualError => {
  it.earlierContexts.push(it.context);
  it.context = TextualErrorContext.create(newContextAction);
  return it;
};

export const changeContextGivenTextualError = (it: TextualError, otherTextualError: TextualError): TextualError => {
  // it.earlierContexts.push(it.context);
  // it.context = TextualErrorContext.create(newContextAction);
  return it;
};

export const addMessage = (it: TextualError, message: string): TextualError => {
  it.context.errorMessages.push(message);
  return it;
}

export const addStringAttachment = (it: TextualError, name: string, value: string) => {
  it.context.attachements.push(TextualErrorAttachment.create(
    name,
    `"${value.replaceAll(/"/g, '\\"').replaceAll(/\n/g, '\\n')}"`
  ));
  return it;
}

export const addErrorAttachment = (it: TextualError, name: string, value: Error) => {
  it.context.attachements.push(TextualErrorAttachment.create(
    name,
    `${value}`
  ));
  return it;
}

export const addNumberAttachment = (it: TextualError, name: string, value: number) => {
  it.context.attachements.push(TextualErrorAttachment.create(
    name,
    value.toString(),
  ));
  return it;
}

export const addBooleanAttachment = (it: TextualError, name: string, value: boolean) => {
  it.context.attachements.push(TextualErrorAttachment.create(
    name,
    value ? "true" : "false",
  ));
  return it;
}

export const addNullAttachment = (it: TextualError, name: string) => {
  it.context.attachements.push(TextualErrorAttachment.create(
    name,
    "null",
  ));
  return it;
};

export const addUnknownAttachment = (
  it: TextualError,
  name: string,
  value: unknown,
): TextualError => {
  throw new Error("Not implemented")
};

export const addPrimitiveAttachment = (
  it: TextualError, 
  name: string, 
  value: null | string | number | boolean,
): TextualError => {
  if (value === null) {
    return addNullAttachment(it, name);
  }
  switch (typeof value) {
    case "string": {
      return addStringAttachment(it, name, value);
    }
    case "number": {
      return addNumberAttachment(it, name, value);
    }
    case "boolean": {
      return addBooleanAttachment(it, name, value);
    }
  }
};

export const print = (it: TextualError) => ""
export const prettyPrint = (it: TextualError) => ""

export const TextualError = {
  create,
  changeContext,
  changeContextGivenTextualError,
  addMessage,
  addBooleanAttachment,
  addErrorAttachment,
  addNullAttachment,
  addNumberAttachment,
  addPrimitiveAttachment,
  addStringAttachment,
  addUnknownAttachment,
  print,
  prettyPrint,
};