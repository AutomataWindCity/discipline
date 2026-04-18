export * from "./utilities.ts"

import { Branded, TextualError, Tried } from "../x.ts";


const BRAND = Symbol();

export type Database = Branded<typeof BRAND, {

}>;

export const Database = {
  open(path: string): Tried<Database, TextualError> {
    
  },

  execute() {
    
  }
};

