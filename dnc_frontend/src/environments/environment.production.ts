export const environment = {
    production: true,
    apiUrl: '/api',
    // same-origin path that your reverse proxy will forward to the collector
    otelTracesUrl: `${window.location.origin}/otlp/v1/traces`,
    apiBaseUrl: '/api',
};
