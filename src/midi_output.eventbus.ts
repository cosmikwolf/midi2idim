// midi_output.eventbus.ts
import { createEventBus } from "ts-event-bus";
import midi_events from "./midi_events";
import MyBasicWebSocketServerChannel from "./MyBasicWebSocketServerChannel";

const EventBus = createEventBus({
  events: midi_events,
  channels: [new MyBasicWebSocketServerChannel("ws://your_host")],
});