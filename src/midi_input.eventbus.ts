// midi_input.eventbus.ts
import { createEventBus } from "ts-event-bus";
import midi_events from "./midi_events.ts";
import MyBasicWebSocketClientChannel from "./MyBasicWebSocketClientChannel.ts";

const EventBus = createEventBus({
  events: midi_events,
  channels: [new MyBasicWebSocketClientChannel("ws://your_host")],
});

export default EventBus;