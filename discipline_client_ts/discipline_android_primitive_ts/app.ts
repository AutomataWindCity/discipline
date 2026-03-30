import { Branded, Database, State } from "./x.ts";

const BRAND = Symbol();

export type App = Branded<typeof BRAND, {
  state: State,
  database: Database,
}>;

export const open = () => {

};

export const App = {

};