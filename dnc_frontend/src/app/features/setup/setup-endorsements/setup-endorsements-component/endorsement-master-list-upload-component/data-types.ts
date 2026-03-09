export type ExistingMasterListMeta = {
    master_list_id: number;
    original_filename: string;
    uploaded_by: string;
    uploaded_at: string; // ISO
    total_rows: number;
};

export type MasterListIssue = {
    row?: number;              // 1-based row index in sheet (optional)
    field?: string;            // e.g. "member_id"
    severity: 'error' | 'warn';
    message: string;
};

export type MasterListPreviewRow = {
    // keep flexible; depends on your schema
    member_id?: string;
    full_name?: string;
    birthdate?: string; // ISO or raw cell string
    // ...
};

export type MasterListPreview = {
    temp_upload_id: string;     // server-side temp id
    sheet_name?: string;
    total_rows: number;
    preview_rows: MasterListPreviewRow[]; // first N rows
    issues: MasterListIssue[];
};
