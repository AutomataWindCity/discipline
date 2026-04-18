const UNIQUE = Symbol();
const THIS_TYPE = Symbol()


export type Unique<Id extends symbol, Name extends string, Value> = 
  // & Omit<Value, typeof THIS_TYPE>
  Value
  & { readonly [UNIQUE]: "Unique" }
  & { readonly [THIS_TYPE]: Value }
  & { readonly [Key in Id]: Name }
;
