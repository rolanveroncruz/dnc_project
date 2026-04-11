import { CommonModule } from '@angular/common';
import { Component, Inject } from '@angular/core';
import {
    FormControl,
    FormGroup,
    ReactiveFormsModule,
    Validators,
    ValidationErrors,
    AbstractControl,
} from '@angular/forms';
import {
    MAT_DIALOG_DATA,
    MatDialogModule,
    MatDialogRef,
} from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';

export interface UploadHighEndServiceFilesDialogData {
    verification_id: number;
    date: string | Date;
    dentist_id: number;
    dentist_name: string;
    master_list_member_id: number;
    master_list_member_name: string;
    dental_service_id: number;
    dental_service_name: string;
}

export interface UploadHighEndServiceFilesDialogResult {
    confirmed: boolean;
    files: File[];
}

function fileCountValidator(
    min: number,
    max: number
) {
    return (control: AbstractControl): ValidationErrors | null => {
        const files = control.value as File[] | null;

        if (!files || files.length < min) {
            return { minFiles: true };
        }

        if (files.length > max) {
            return { maxFiles: true };
        }

        return null;
    };
}

@Component({
    selector: 'app-upload-high-end-service-files',
    standalone: true,
    imports: [
        CommonModule,
        ReactiveFormsModule,
        MatDialogModule,
        MatButtonModule,
        MatFormFieldModule,
        MatInputModule,
    ],
    templateUrl: './upload-high-end-service-files-component.html',
    styleUrl: './upload-high-end-service-files-component.scss',
})
export class UploadHighEndServiceFilesComponent {
    readonly maxFiles = 3;

    readonly form: FormGroup<{
        files: FormControl<File[] | null>;
    }>;

    constructor(
        private dialogRef: MatDialogRef<
            UploadHighEndServiceFilesComponent,
            UploadHighEndServiceFilesDialogResult
        >,
        @Inject(MAT_DIALOG_DATA) public data: UploadHighEndServiceFilesDialogData,
    ) {
        this.form = new FormGroup({
            files: new FormControl<File[] | null>(null, {
                validators: [fileCountValidator(1, 3)],
            }),
        });
    }

    get selectedFiles(): File[] {
        return this.form.controls.files.value ?? [];
    }

    onFilesSelected(event: Event): void {
        const input = event.target as HTMLInputElement;
        const fileList = input.files;

        if (!fileList) {
            this.form.controls.files.setValue(null);
            this.form.controls.files.markAsTouched();
            return;
        }

        const files = Array.from(fileList).slice(0, this.maxFiles);
        this.form.controls.files.setValue(files);
        this.form.controls.files.markAsTouched();
        this.form.controls.files.updateValueAndValidity();

        input.value = '';
    }

    removeFile(index: number): void {
        const updatedFiles = [...this.selectedFiles];
        updatedFiles.splice(index, 1);

        this.form.controls.files.setValue(updatedFiles.length ? updatedFiles : null);
        this.form.controls.files.markAsTouched();
        this.form.controls.files.updateValueAndValidity();
    }

    cancel(): void {
        this.dialogRef.close({
            confirmed: false,
            files: [],
        });
    }

    submit(): void {
        if (this.form.invalid) {
            this.form.markAllAsTouched();
            return;
        }

        this.dialogRef.close({
            confirmed: true,
            files: this.selectedFiles,
        });
    }
}
