export const SUCCESS = Symbol();
export const FAILURE = Symbol();

export type SuccessCode = typeof SUCCESS;
export type FailureCode = typeof FAILURE;
export type TriedCode = SuccessCode | FailureCode;

export const TriedCode = {

};