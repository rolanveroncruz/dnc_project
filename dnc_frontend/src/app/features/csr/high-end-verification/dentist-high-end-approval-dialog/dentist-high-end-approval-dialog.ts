import { CommonModule } from '@angular/common';
import { Component, Inject } from '@angular/core';
import {
    MAT_DIALOG_DATA,
    MatDialogModule,
    MatDialogRef,
} from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import {HighEndVerificationResponse, HighEndFileResponse} from '../../../../api_services/high-end-verifications-service';

@Component({
    selector: 'app-dentist-high-end-approval-dialog',
    standalone: true,
    imports: [
        CommonModule,
        MatDialogModule,
        MatButtonModule,
    ],
    templateUrl: './dentist-high-end-approval-dialog.html',
    styleUrl: './dentist-high-end-approval-dialog.scss',
})
export class DentistHighEndApprovalDialogComponent {
    constructor(
        // ✅ receive the clicked row as dialog data
        @Inject(MAT_DIALOG_DATA) public data: HighEndVerificationResponse,
        private dialogRef: MatDialogRef<DentistHighEndApprovalDialogComponent>
    ) {}

    close(): void {
        this.dialogRef.close();
    }
}
