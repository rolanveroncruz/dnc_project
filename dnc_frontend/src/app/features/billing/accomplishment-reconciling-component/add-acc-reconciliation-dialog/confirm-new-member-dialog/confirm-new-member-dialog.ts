import {Component, Inject} from '@angular/core';
import {FormControl, ReactiveFormsModule} from '@angular/forms';
import {
    MAT_DIALOG_DATA,
    MatDialogActions,
    MatDialogContent,
    MatDialogRef,
    MatDialogTitle
} from '@angular/material/dialog';
import {MatFormField, MatLabel} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {MatButton} from '@angular/material/button';

export interface ConfirmNewMemberDialogData {
    name: string;
}

export interface ConfirmNewMemberDialogResult {
    confirmed: boolean;
    account_number: string | null;
}

@Component({
  selector: 'app-confirm-new-member-dialog',
    standalone: true,
    imports: [
        MatDialogContent,
        ReactiveFormsModule,
        MatFormField,
        MatLabel,
        MatDialogActions,
        MatDialogTitle,
        MatInputModule,
        MatButton
    ],
  templateUrl: './confirm-new-member-dialog.html',
  styleUrl: './confirm-new-member-dialog.scss',
})
export class ConfirmNewMemberDialog {
    readonly accountNumber = new FormControl<string>('', {nonNullable: true});

    constructor(
        private readonly dialogRef: MatDialogRef<
            ConfirmNewMemberDialog,
            ConfirmNewMemberDialogResult
        >,
        @Inject(MAT_DIALOG_DATA) public readonly data: ConfirmNewMemberDialogData,
    ) {}

    decline(): void {
        this.dialogRef.close({
            confirmed: false,
            account_number: null,
        });
    }

    confirm(): void {
        const accountNumber = this.accountNumber.value.trim();

        this.dialogRef.close({
            confirmed: true,
            account_number: accountNumber || null,
        });
    }
}
