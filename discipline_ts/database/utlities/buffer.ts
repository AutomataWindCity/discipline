import { NamedWriteDestination } from "./mod.ts";

export interface Buffer {
  readonly buffer: string[],
}

const create = (): Buffer => {
  return {
    buffer: [],
  };
};

const write = (it: Buffer, slice: string) => {};

const namedWriteDestination = NamedWriteDestination.autoImplementForWritable<Buffer>({
  write,
});

export const Buffer = {
  create,
  write,
  namedWriteDestination,
};

