import { Unique } from "../x.ts";

const BRAND = Symbol();
export type Float = Unique<typeof BRAND, "Float", number>;
