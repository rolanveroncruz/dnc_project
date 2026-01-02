import { WebTracerProvider } from '@opentelemetry/sdk-trace-web';
import { BatchSpanProcessor, ConsoleSpanExporter } from '@opentelemetry/sdk-trace-base';
import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-http';
import { registerInstrumentations } from '@opentelemetry/instrumentation';
import { FetchInstrumentation } from '@opentelemetry/instrumentation-fetch';

// Import the factory function instead of the Resource class
import { resourceFromAttributes } from '@opentelemetry/resources';
import { ATTR_SERVICE_NAME } from '@opentelemetry/semantic-conventions';

// 1. Create the Resource using the factory function (Bypasses TS2693)
const resource = resourceFromAttributes({
  [ATTR_SERVICE_NAME]: 'my-angular-frontend',
});

// 2. Initialize the Provider
// Note: In v2.x, we pass processors in the constructor array
const exporter = new OTLPTraceExporter({
  url: 'http://localhost:4318/v1/traces',
});
const provider = new WebTracerProvider({
  resource: resource,
  spanProcessors: [
    new BatchSpanProcessor(exporter as any ),
    new BatchSpanProcessor(new ConsoleSpanExporter()),
  ],
});

// 3. Register with ZoneContextManager (Required for Angular)
provider.register();

// 4. Instrument Fetch calls
registerInstrumentations({
  instrumentations: [
    new FetchInstrumentation({
      propagateTraceHeaderCorsUrls: [
        /http:\/\/localhost:3000\/.*/,
      ],
    }),
  ],
});

console.log('OpenTelemetry v2.2.0 Initialized Successfully');
