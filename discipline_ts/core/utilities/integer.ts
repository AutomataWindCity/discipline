export const fromString = (string: string): number | null => {
  let result: number;

  try {
    result = parseInt(string);
  } catch (error) {
    return null;
  }

  if (!Number.isSafeInteger(result)) {
    return null;
  }

  return result;
};


export const isInteger = Number.isInteger as ((value: unknown) => value is number);

export const parseInteger = (string: string) => {
  let number;

  try {
    number = parseInt(string);
  } catch (error) {
    return null;
  }

  if (number !== number) {
    return null;
  }

  return number;
};