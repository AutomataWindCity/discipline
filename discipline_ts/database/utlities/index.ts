import { Unique } from "../../x.ts";

const BRAND = Symbol();

export type Index = Unique<typeof BRAND, "Index", number>;