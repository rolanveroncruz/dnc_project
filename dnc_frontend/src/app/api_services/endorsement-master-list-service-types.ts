
export interface UploadedMasterListMemberRow {
    row_number: number;
    corporate_number: string;
    account_number: string;
    last_name: string;
    first_name: string;
    middle_name: string;
}

export interface ExistingDuplicateRow {
    id: number;
    endorsement_id: number;
    master_list_id: number | null;
    account_number: string;
    last_name: string;
    first_name: string;
    middle_name: string;
    email_address: string | null;
    mobile_number: string | null;
    birth_date: string | null;
    is_active: boolean;
}
export interface DuplicateRowResponse {
    uploaded_row: UploadedMasterListMemberRow;
    existing_row: ExistingDuplicateRow;
}

export interface UploadEndorsementMasterListResponse {
    master_list_id: number;
    file_name: string;
    endorsement_id: number;
    inserted_count: number;
    inserted_rows: UploadedMasterListMemberRow[];
    skipped_corporate_number_mismatch_count: number;
    duplicate_count: number;
    duplicates: DuplicateRowResponse[];
}
