// midi_events.ts
import { slot, Slot } from "ts-event-bus";

const MyEvents = {
  sayHello: slot<string>(),
  getTime: slot<null, string>(),
  multiply: slot<{ a: number; b: number }, number>(),
  ping: slot<void>(),
};

export default MyEvents;