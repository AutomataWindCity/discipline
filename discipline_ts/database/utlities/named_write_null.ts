import { Name } from "./mod.ts";

/**
 * 
 * Implemented by compound types that are to be stored in SQLite.
 */
export interface NamedWriteNull<Names> {
  readonly write: <Destination>(
    names: Names, 
    destination: Destination,
    destinationImpl: NamedWriteNullDestination<Destination>,
  ) => void;
}

export interface NamedWriteNullDestination<It> {
  readonly writeNull: (it: It, name: Name) => void;
}

export const NamedWriteNull = {
  implement: <Names>(initializer: NamedWriteNull<Names>): NamedWriteNull<Names> => {
    return initializer;
  },
};