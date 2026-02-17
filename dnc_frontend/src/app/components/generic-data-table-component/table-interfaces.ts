export interface TableColumn<T extends object = any> {
    key: string;       // The property name in your JSON (e.g., 'email')
    label: string;     // The text to display in the header (e.g., 'Email Address')
    sortable?: boolean; // Whether to allow sorting on this column
    cellTemplateKey?: string; // e.g. 'chips', 'date', 'money', etc.
    widthPx?: number; // fixed width in pixels
    minWidthPx?: number; // minimum width in pixels
    maxWidthPx?: number; // maximum width in pixels
}

export interface FilterConfig {
    key: string;
    label: string;
    options?: string[]; // If omitted, we can auto-calculate unique values from data
}
