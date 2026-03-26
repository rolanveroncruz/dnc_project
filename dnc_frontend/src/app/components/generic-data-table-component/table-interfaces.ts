export interface TableColumn<T extends object = any> {
    key: string;       // The property name in your JSON (e.g., 'email')
    label: string;     // The text to display in the header (e.g., 'Email Address')
    sortable?: boolean; // Whether to allow sorting on this column
    widthPx?: number; // fixed width in pixels
    minWidthPx?: number; // minimum width in pixels
    maxWidthPx?: number; // maximum width in pixels
    cellTemplateKey?: TableCellTemplateKey; // e.g. 'chips', 'date', 'money', etc.
    actionButton?: TableActionButton<T>;
}

export interface FilterConfig {
    key: string;
    label: string;
    options?: string[]; // If omitted, we can auto-calculate unique values from data
}
export type TableCellTemplateKey =
    | 'default'
    | 'date'
    | 'datetime'
    | 'chips'
    | 'check'
    | 'checkonly'
    | 'actions';

export interface TableActionButton<T = any> {
    label: string | ((row:T)=>string);
    icon?: string| ((row:T)=>string);
    onClick: (row: T) => void;
    color?: 'primary' | 'accent' | 'warn';
    disabled?: (row: T) => boolean;
    hidden?: (row: T) => boolean;
}
