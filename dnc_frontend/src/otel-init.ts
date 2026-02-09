import { WebTracerProvider } from '@opentelemetry/sdk-trace-web';
import { SimpleSpanProcessor,} from '@opentelemetry/sdk-trace-base';
import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-http';
import { registerInstrumentations } from '@opentelemetry/instrumentation';
import { FetchInstrumentation } from '@opentelemetry/instrumentation-fetch';
import { Resource } from '@opentelemetry/resources';
import {ATTR_SERVICE_NAME } from '@opentelemetry/semantic-conventions';
import { ZoneContextManager } from '@opentelemetry/context-zone';
import {environment} from './environments/environment';

// 1. Setup Resource (v1.x style)
const resource = new Resource({
  [ATTR_SERVICE_NAME]: 'my-angular-frontend',
});

const exporter = new OTLPTraceExporter({
  url: environment.otelTracesUrl,
});

const provider = new WebTracerProvider({ resource,
spanProcessors:[
  new SimpleSpanProcessor(exporter),
  // new SimpleSpanProcessor(new ConsoleSpanExporter())
]
});

// 2. Use SimpleSpanProcessor to force immediate export for debugging

// 3. Register with ZoneContextManager
provider.register({
  contextManager: new ZoneContextManager(),
});

// 4. Instrument Fetch
registerInstrumentations({
  instrumentations: [
    new FetchInstrumentation({
      propagateTraceHeaderCorsUrls: [
          environment.apiBaseUrl.startsWith('http')
              ? new RegExp(`^${escapeRegExp(environment.apiBaseUrl)}/.*`)
              : /\/api\/.*/ // if you keep apiBaseUrl as "/api" in prod
      ],
    }),
  ],
});
import { trace } from '@opentelemetry/api';
import {escapeRegExp} from '@angular/compiler';

console.log("DIAGNOSTIC: Attempting manual trace...");

const tracer = trace.getTracer('diagnostic-tracer');
const span = tracer.startSpan('manual-diagnostic-span');

// End the span after 500ms and see if it triggers a network call
setTimeout(() => {
  span.end();
  console.log("DIAGNOSTIC: Manual span ended. Look for a POST to :4318 in the Network tab.");
}, 500);
