import { Component, Inject } from '@angular/core';
import { MAT_DIALOG_DATA, MatDialogActions, MatDialogContent, MatDialogRef, MatDialogTitle } from '@angular/material/dialog';
import { MatButton } from '@angular/material/button';
import { UploadEndorsementMasterListResponse} from '../../../../../../api_services/endorsement-master-list-service-types';

// ✅ Keep these only if they are not already declared/imported elsewhere.
export interface UploadedMasterListMemberRow {
    // Add fields later if this dialog ever needs them.
}

export interface DuplicateRowResponse {
    // Add fields later if this dialog ever needs them.
}

@Component({
    selector: 'app-upload-statistics-dialog',
    imports: [
        MatDialogTitle,
        MatDialogContent,
        MatDialogActions,
        MatButton,
    ],
    templateUrl: './upload-statistics-dialog.html',
    styleUrl: './upload-statistics-dialog.scss',
})
export class UploadStatisticsDialog {

    constructor(
        private dialogRef: MatDialogRef<UploadStatisticsDialog>,
        @Inject(MAT_DIALOG_DATA)
        public data: UploadEndorsementMasterListResponse,
    ) {
    }

    close(): void {
        this.dialogRef.close();
    }
}
